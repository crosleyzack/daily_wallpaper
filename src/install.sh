#!/bin/bash

FILE_PATH=$(realpath $BASH_SOURCE)
DIR_PATH=$(dirname $FILE_PATH)

# Update permissions
chmod 777 $DIR_PATH/set_wallpaper.sh
chmod 777 $DIR_PATH/create_wallpaper.sh
chmod 777 $DIR_PATH/sync_wallpaper.sh
chmod 777 $DIR_PATH/server.sh
chmod 777 $DIR_PATH/wallpaper_cron.sh
chmod 666 $DIR_PATH/../data/quotes.json
chmod 777 $DIR_PATH/../assets/wallpapers
find $DIR_PATH/../assets/wallpapers -type f -exec chmod 666 {} \;

# Dumb permissions stuff
# sudo chown $USER:$USER ~/.config/dconf -R
chmod u+w ~/.config/dconf -R

echo "updated permissions"

# make desktop startup file
# https://specifications.freedesktop.org/desktop-entry-spec/desktop-entry-spec-latest.html
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

echo "Wallpaper file created"

# Set for gnome to run at startup
XDG_CONFIG_HOME="${XDG_CONFIG_HOME:-$HOME/.config}"
AUTOSTART_DIR="$XDG_CONFIG_HOME/autostart"
AUTOSTART_FILE="$AUTOSTART_DIR/wallpaper.desktop"
mkdir -p $AUTOSTART_DIR
rm -f $AUTOSTART_FILE
ln -s $WALLPAPER_FILE $AUTOSTART_FILE
echo "Set to run at startup"
echo "Install complete"
