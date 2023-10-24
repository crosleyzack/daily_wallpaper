#!/bin/bash

sudo apt install imagemagick

chmod 777 ./set_wallpaper.sh
chmod 777 ./create_wallpaper.sh
chmod 777 ./sync_wallpaper.sh
chmod 777 ./server.sh
chmod 777 ./wallpaper_cron.sh
chmod 666 quotes.json
chmod 777 wallpapers
find wallpapers -type f -exec chmod 666 {} \;

# Dumb permissions stuff
sudo chown $USER:$USER ~/.config/dconf -R
chmod u+w ~/.config/dconf -R

# make desktop startup file
# https://specifications.freedesktop.org/desktop-entry-spec/desktop-entry-spec-latest.html
FILE_PATH=$(realpath $BASH_SOURCE)
DIR_PATH=$(dirname $FILE_PATH)
WALLPAPER_FILE="$DIR_PATH/wallpaper.desktop"
rm -f $WALLPAPER_FILE
echo "[Desktop Entry]
Version=1.0
Type=Application
Name=WallpaperScript
GenericName=WallpaperScript
Comment=Create a new wallpaper
Exec=$DIR_PATH/wallpaper_cron.sh
OnlyShowIn=GNOME;" > $WALLPAPER_FILE

# Set for gnome to run at startup
mkdir -p $HOME/.config/autostart
rm -f $HOME/.config/autostart/wallpaper.desktop
ln -s $WALLPAPER_FILE $HOME/.config/autostart/wallpaper.desktop

