# random_desktop_quote
Generates a background on restart which picks and image and quote.

Run `install.sh` to install necessary requirements.

Put desired base wallpapers in `wallpapers` directory and quotes in `quotes.json` (schema below).

Run `wallpaper_cron.sh` on startup. `install.sh` places a `.desktop` file in `~/.config/autostart` automatically.

# Quotes Schema

Schema to use for quote entries in the `quotes.json` file. All values other than `quote` and `author` have a default in `create_wallpaper.sh` and can be ommitted.

```json
{
  "type": "object",
  "properties": {
    "quote": {
      "type": "string",
      "description": "Specifies the quote to print",
      "exclusiveMinimum": 0,
    },
    "author": {
      "type": "string",
      "description": "Specifies the author for the quote",
      "exclusiveMinimum": 0,
    },
    "fileRegex": {
      "type": "string",
      "description": "Defines the regex to filter wallpapers to select from. If unspecified, selects from all",
    },
    "wordsPerLine": {
      "type": "int",
      "description": "Defines words printed per line on the wallpaper. If unspecified, uses default",
    },
    "fontSize": {
      "type": "int",
      "description": "Defines the font size to be used. If unspecified, uses default",
    },
    "font": {
      "type": "string",
      "description": "Defines the font to print in. If unspecified, uses default",
    },
    "fontColor": {
      "type": "string",
      "description": "Defines the color for the font to be printed in. If unspecified, uses default",
    },
    "gravity": {
      "type": "string",
      "description": "Defines the base location to print text on the wallpaper. If unspecified, uses default",
    },
    "annotate": {
      "type": "string",
      "description": "Defines the location adjustment for font location to be printed. If unspecified, uses default",
    },
  },
  "required": [ "quote", "author" ],
}
```

> :warning: Just because a field can be changed doesn't mean it should

# Credits

```json
[
  {
    "wallpapers": "landscape*.png",
    "author": "Louis Coyle",
    "source": "https://louie.co.nz/25th_hour/",
  },
  {
    "wallpapers": "buddha.png",
    "author": "Phil Distress",
    "source": "https://www.deviantart.com/phildistress/art/Moonlight-Meditation-811660312",
  },
  {
    "wallpapers": "iroh*.png",
    "author": "Damion Mauville",
    "source": "https://www.deviantart.com/damionmauville/gallery/63670706/avatar-wallpapers",
  },
]
```
