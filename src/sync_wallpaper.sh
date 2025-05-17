#!/bin/bash

# Sync pulls the wallpaper from github where a daily workflow will generate it

# Ensure a base directory was provided.
if [ ! -d "$1" ]
then
    echo "sync_wallpaper.sh \$1 must be the assets dir"
    exit 1
fi
if [ -z "$2" ]
then
    # We will assume wallpaper name if not provided
    WALLPAPER_NAME="wallpaper.png"
else
    WALLPAPER_NAME="$2"
fi

ASSETS_DIR=$(realpath $1)

echo "sync_wallpaper.sh retrieving $ASSETS_DIR/$WALLPAPER_NAME"
wget -v -O $ASSETS_DIR/$WALLPAPER_NAME http://raw.githubusercontent.com/CrosleyZack/random_desktop_quote/refs/heads/main/assets/wallpaper.png
# curl -v http://raw.githubusercontent.com/CrosleyZack/random_desktop_quote/refs/heads/main/assets/wallpaper.png -o $ASSETS_DIR/$WALLPAPER_NAME
logger "sync_wallpaper.sh $ASSETS_DIR/$WALLPAPER_NAME retrieved"
