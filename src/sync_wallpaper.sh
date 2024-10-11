#!/bin/bash

# Sync pulls the wallpaper from github where a daily workflow will generate it

DEFAULT_WALLPAPER_NAME="wallpaper.png"

# Ensure a base directory was provided.
if [ ! -d "$1" ]
then
    echo "sync_wallpaper.sh \$1 must be the assets dir"
    exit 1
fi
if [ -z "$2" ]
then
    # We will assume wallpaper name if not provided
    WALLPAPER_NAME="$DEFAULT_WALLPAPER_NAME"
else
    WALLPAPER_NAME="$2"
fi

ASSETS_DIR=$(realpath $1)

rm -f $ASSETS_DIR/$WALLPAPER_NAME
curl https://raw.githubusercontent.com/CrosleyZack/random_desktop_quote/refs/heads/main/assets/wallpaper.png -o $ASSETS_DIR/$WALLPAPER_NAME
