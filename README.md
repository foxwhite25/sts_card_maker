# Slay the Spire Card Maker

This project is made so that I can easily replace vanilla card art by just providing this programme with a 500 * 380 image.

Originally I used [JohnnyBazooka89/StSModdingToolCardImagesCreator](https://github.com/JohnnyBazooka89/StSModdingToolCardImagesCreator) to mask the image,
but it is so god dame slow (maybe because it is written in Java IDK) that I also integrated the masking into this program. So I guess with some modifications this program can also replace that Java one. 
This programme can mask 200 image in about 1 second, compare to the minutes for the java one I'd say it is a lot faster.

## Usage
To use this programme without modification, you will need the `cards.atlas` file and the big card image from 1 to 5, which both can be found in the game's jar file. I would not include those file here because I don't want to get DMCA'd.

Of course, you also need the image you want to replace named with the card id. Strikes and defend are treated differently as all classes have them, so you need to provide the image with the name `strike_<color>.png` and `defend_<color>.png` respectively. For example watcher's defend will be `defend_p.png` (p because purple).

With these fill you will need to put them in this structure
```
sts_card_maker
  data
    cards.atlas
    cards.png
    cards2.png
    cards3.png
    cards4.png
    cards5.png
    
    original
      <the card you want to replace ...>
```

Run this rust program and it will generate a `new` folder with the modified `cards.png` in it. You can then replace the original files or make a mod with it.
```bash 
cargo run --release
```