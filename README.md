# random_desktop_quote
Generates a background on restart which picks and image and quote.

Run `install.sh` to install necessary requirements.

Put desired base wallpapers in `wallpapers` directory and quotes in individual lines of `quotes.txt`.

Run `wallpaper_cron.sh` on startup.

# Quotes Schema

```json
{
  "type": "object",
  "properties": {
    "quote": {
      "type": "string",
      "description": "Specifies the quote to print",
    },
    "author": {
      "type": "string",
      "description": "Specifies the author for the quote",
    },
    "fileRegex": {
      "type": "string",
      "description": "Defines the regex to filter wallpapers to select from",
    },
    "wordsPerLine": {
      "type": "int",
      "description": "Defines the number of words to be printed per line on the wallpaper",
    },
    "fontSize": {
      "type": "int",
      "description": "Defines the font size to be printed on the wallpaper",
    },
    "font": {
      "type": "string",
      "description": "Defines the font to print in",
    },
    "fontColor": {
      "type": "string",
      "description": "Defines the color for the font to be printed in",
    },
    "gravity": {
      "type": "string",
      "description": "Defines the base location to print text on the wallpaper",
    },
    "annotate": {
      "type": "string",
      "description": "Defines the location adjustment for font location to be printed",
    },
  },
}
```

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
