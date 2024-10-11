# random_desktop_quote
Generates a background on restart which picks and image and quote.

Run `task install` to setup execution on startup and cron.

Put desired base wallpapers in `wallpapers` directory and quotes in `quotes.json` (schema below).

Wallpapers can be generated locally with `task create` and set as wallpaper on a gnome desktop with `task set`

`task sync` will pull todays wallpaper from the github repository. Daily wallpaper should be generated at midnight.

On startup, cron will run pull desktop from github repository and set it as the current wallpaper.

To create your own wallpaper locally, run `create_wallpaper.sh` and `set_wallpaper.sh` with appropriate args or `task create` and `task set`

# Today's Wallpaper

![Todays Wallpaper](assets/wallpaper.png)

# Quotes Schema

Schema to use for quote entries in the `quotes.json` file. All values other than `quote` and `author` have a default in `create_wallpaper.sh` and can be ommitted.

```json
{
  "type": "object",
  "description": "quotes.json schema",
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
      "description": "Defines the font size to be used. If unspecified, uses default. See https://imagemagick.org/script/command-line-options.php#pointsize",
    },
    "font": {
      "type": "string",
      "description": "Defines the font to print in. If unspecified, uses default. See https://imagemagick.org/script/command-line-options.php#font",
      
    },
    "fontColor": {
      "type": "string",
      "description": "Defines the color for the font to be printed in. If unspecified, uses default. See https://imagemagick.org/script/command-line-options.php#fill",
    },
    "gravity": {
      "type": "string",
      "description": "Defines the base location to print text on the wallpaper. If unspecified, uses default (North). See https://imagemagick.org/script/command-line-options.php#gravity",
    },
    "annotate": {
      "type": "string",
      "description": "Defines the location adjustment for font location to be printed. If unspecified, uses default. See https://imagemagick.org/script/command-line-options.php#annotate",
    },
  },
  "required": [ "quote", "author" ],
}
```

:warning:  Just because a field can be changed doesn't mean it should

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
  {
    "wallpapers": "cross.png",
    "author": "gstudioimagen",
    "source": "https://www.vecteezy.com/photo/24622803-christian-cross-on-majestic-mountain-peak-at-sunset-generative-ai",
  },
  {
    "wallpapers": "dune.png",
    "author": "Illustrated Vectors",
    "source": "https://www.deviantart.com/illustratedvectors/art/Sand-Dune-Mojave-899873021",
  },
]
```
