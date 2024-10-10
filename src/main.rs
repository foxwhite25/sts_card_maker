use image::{GenericImage, GenericImageView};
use itertools::Itertools;

use log::{debug, info, warn};
use rayon::prelude::*;
use std::fs;
use std::io::BufReader;
use std::ops::Add;
use std::path::Path;

#[derive(Debug)]
struct CardInfo {
    character: String,
    file_name: String,
    type_name: Option<String>,
    card_name: String,
    position: (u32, u32),
    size: (u32, u32),
}

impl CardInfo {
    fn modified_card_name(&self) -> String {
        match self.card_name.as_str() {
            "defend" | "strike" => self
                .card_name
                .clone()
                .add(format!("_{}", self.character.chars().next().unwrap()).as_str()),
            _ => self.card_name.clone(),
        }
    }
}

macro_rules! unwrap_or_return {
    ( $e:expr ) => {
        match $e {
            Ok(x) => x,
            Err(_) => return,
        }
    };
}

fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();
    let input = fs::read_to_string("data/cards.atlas").unwrap();
    let cards = parse_input(input);

    mask_sprites(&cards);
    check_not_exist(&cards);
    modify_sprites(&cards);
    move_sprites(&cards);
    check_valid_sprites(&cards);

    info!("Done!");
}

fn parse_input(input: String) -> Vec<CardInfo> {
    info!("Parsing cards.atlas file");
    input
        .trim()
        .split("\n\n")
        .map(|x| {
            let mut line_iter = x.lines();
            let file_name = line_iter.next().unwrap();
            line_iter
                .skip(4)
                .chunks(7)
                .into_iter()
                .filter_map(|x| {
                    let mut iter = x.into_iter();
                    let path = iter.next().unwrap();
                    iter.next();
                    let position = iter.next().unwrap();

                    let path_iter = path.split("/").collect_vec();
                    let (character, type_name, card_name) = if path_iter.len() == 2 {
                        (path_iter[0].to_string(), None, path_iter[1].to_string())
                    } else {
                        (
                            path_iter[0].to_string(),
                            Some(path_iter[1].to_string()),
                            path_iter[2].to_string(),
                        )
                    };
                    let position = position.split(": ").nth(1).unwrap();
                    let (x, y) = position.split_once(", ").expect(position);
                    let x = x.parse::<u32>().unwrap();
                    let y = y.parse::<u32>().unwrap();

                    let size = iter.next().unwrap();
                    let size = size.split(": ").nth(1).unwrap();
                    let (w, h) = size.split_once(", ").expect(size);
                    let w = w.parse::<u32>().unwrap();
                    let h = h.parse::<u32>().unwrap();
                    Some(CardInfo {
                        character,
                        file_name: file_name.to_string(),
                        type_name,
                        card_name,
                        position: (x, y),
                        size: (w, h),
                    })
                })
                .collect_vec()
        })
        .flatten()
        .collect_vec()
}

fn check_not_exist(cards: &Vec<CardInfo>) {
    info!("Checking not exist");
    cards
        .iter()
        .for_each(|card| {
            let path = format!("data/masked/{}.png", card.modified_card_name());
            if !Path::new(&path).exists() {
                warn!("{} does not exist", card.modified_card_name());
            }
        });
}

fn mask_sprites(cards: &Vec<CardInfo>) {
    info!("Masking sprites");
    cards.par_iter().for_each(|card| {
        let mask_name = match card.type_name {
            Some(ref x) => match x.as_str() {
                "attack" => "AttackMask_p.png",
                "power" => "PowerMask_p.png",
                _ => "SkillMask_p.png",
            },
            None => "SkillMask_p.png",
        };
        let mask = image::open(format!("data/masks/{}", mask_name)).unwrap();
        let file = unwrap_or_return!(fs::File::open(format!(
            "data/original/{}.png",
            card.modified_card_name()
        )));
        let buf_reader = BufReader::new(file);
        let mut original = image::io::Reader::new(buf_reader)
            .with_guessed_format()
            .unwrap()
            .decode()
            .unwrap()
            .into_rgba8();

        debug!("Masking {} with {}", card.modified_card_name(), mask_name);

        // If the mask is black on that pixel, turn it to transparent
        mask.pixels().for_each(|(x, y, pixel)| {
            if pixel[0] == 0 {
                original.put_pixel(x, y, image::Rgba([0, 0, 0, 0]));
            }
        });

        let p_path = format!("data/masked/{}_p.png", card.modified_card_name());
        let s_path = format!("data/masked/{}.png", card.modified_card_name());
        create_path_if_not_exist(&p_path);
        create_path_if_not_exist(&s_path);

        original.save(&p_path).unwrap();
        let p = image::open(p_path).unwrap();
        let resized = p.resize(250, 190, image::imageops::FilterType::CatmullRom);
        resized.save(s_path).unwrap();
    });
}

fn create_path_if_not_exist(path: &str) {
    let parent_path = Path::new(&path).parent().unwrap();
    if !parent_path.exists() {
        fs::create_dir_all(parent_path).unwrap();
    }
}

fn check_valid_sprites(cards: &Vec<CardInfo>) {
    info!("Checking valid sprites");
    let mut card_map = std::collections::HashMap::new();
    cards.iter().for_each(|card| {
        card_map.insert(card.modified_card_name(), card);
    });

    fs::read_dir("data/original/").unwrap().for_each(|file| {
        let file_name = file.unwrap().file_name().into_string().unwrap();
        if !file_name.ends_with(".png") {
            warn!("Card file not end with png: {}", file_name);
            return;
        }
        let card_name = file_name.strip_suffix(".png").unwrap();
        if card_name.ends_with("_p") {
            return;
        }
        if !card_map.contains_key(card_name) {
            warn!("Missing card: {}", card_name);
        }
    })
}

fn modify_sprites(cards: &Vec<CardInfo>) {
    info!("Modifying sprites");
    let mut original = image::open("data/cards.png").unwrap();
    let mut original2 = image::open("data/cards2.png").unwrap();
    let mut original3 = image::open("data/cards3.png").unwrap();
    let mut original4 = image::open("data/cards4.png").unwrap();
    let mut original5 = image::open("data/cards5.png").unwrap();
    let mut originals = vec![
        &mut original,
        &mut original2,
        &mut original3,
        &mut original4,
        &mut original5,
    ];

    cards.iter().for_each(|card| {
        let card_image = unwrap_or_return!(image::open(format!(
            "data/masked/{}.png",
            card.modified_card_name()
        )));
        debug!("Modifying card: {:?}", card);
        let modified = card_image.crop_imm(0, 0, card.size.0.clone(), card.size.1.clone());

        let original_idx = card
            .file_name
            .strip_prefix("cards")
            .unwrap()
            .strip_suffix(".png")
            .unwrap()
            .parse::<usize>()
            .unwrap_or(1)
            - 1;
        let original = originals.get_mut(original_idx).unwrap();
        original
            .copy_from(&modified, card.position.0.clone(), card.position.1.clone())
            .expect(format!("Failed to copy card {:?}, {}", card, original_idx).as_str());
    });
    create_path_if_not_exist("data/new/cards/cards.png");
    originals.iter().enumerate().for_each(|(idx, original)| {
        if idx == 0 {
            original.save("data/new/cards/cards.png").unwrap();
        } else {
            original
                .save(format!("data/new/cards/cards{}.png", idx + 1))
                .unwrap();
        }
    })
}

fn move_sprites(cards: &Vec<CardInfo>) {
    info!("Moving sprites");
    cards.par_iter().for_each(|card| {
        let og_path = format!("data/masked/{}_p.png", card.modified_card_name());
        let new_path = match card.type_name {
            Some(ref type_name) => {
                if type_name == "status" && card.card_name == "beta" {
                    return;
                }
                format!(
                    "data/new/images/1024Portraits/{}/{}/{}.png",
                    card.character, type_name, card.card_name
                )
            },
            None => format!(
                "data/new/images/1024Portraits/{}/{}.png",
                card.character, card.card_name
            ),
        };

        create_path_if_not_exist(&new_path);

        if Path::new(&og_path).exists() {
            fs::copy(og_path, new_path).unwrap();
        }
    });
}
