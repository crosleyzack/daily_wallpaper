#!/bin/bash

source "${BASH_SOURCE%/*}/../tools/install_tools.sh"

is_installed convert
if [ "false" = "$INSTALLED" ]
then
    sudo apt install imagemagick
    # Create sym link, so this is visible to startup
    sudo ln -s $(which convert) /usr/local/bin/convert
fi

# Update permissions
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
AUTOSTART_DIR="$HOME/.config/autostart"
AUTOSTART_FILE="$AUTOSTART_DIR/wallpaper.desktop"
mkdir -p $AUTOSTART_DIR
rm -f $AUTOSTART_FILE
ln -s $WALLPAPER_FILE $AUTOSTART_FILE

