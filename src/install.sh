#!/bin/bash

# When do you want cronjob to run each day
HOUR=06
MIN=00

FILE_PATH=$(realpath $BASH_SOURCE)
DIR_PATH=$(dirname $FILE_PATH)
ASSET_DIR=$(realpath "$DIR_PATH/../assets")
DATA_DIR=$(realpath "$DIR_PATH/../data")

CREATE_EXE="$DIR_PATH/create_wallpaper.sh"
SET_EXE="$DIR_PATH/set_wallpaper.sh"
SYNC_EXE="$DIR_PATH/sync_wallpaper.sh"

# Update permissions
chmod 777 $CREATE_EXE
chmod 777 $SET_EXE
chmod 777 $SYNC_EXE
chmod 666 $DATA_DIR/quotes.json
chmod 777 $ASSET_DIR/wallpapers
find $ASSET_DIR/wallpapers -type f -exec chmod 666 {} \;

# Dumb permissions stuff
# sudo chown $USER:$USER ~/.config/dconf -R
chmod u+w ~/.config/dconf -R

echo "updated permissions"

# Command is to pull desktop from remote and set
COMMAND="$SYNC_EXE $ASSET_DIR wallpaper.png; $SET_EXE $ASSET_DIR/wallpaper.png"

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
Exec=$COMMAND
OnlyShowIn=GNOME;" > $WALLPAPER_FILE

# Set for gnome to run at startup
XDG_CONFIG_HOME="${XDG_CONFIG_HOME:-$HOME/.config}"
AUTOSTART_DIR="$XDG_CONFIG_HOME/autostart"
AUTOSTART_FILE="$AUTOSTART_DIR/wallpaper.desktop"
mkdir -p $AUTOSTART_DIR
rm -f $AUTOSTART_FILE
ln -s $WALLPAPER_FILE $AUTOSTART_FILE
echo "Set to run at startup"
echo "Install complete"

# Setup cron
# TODO: systemd timer as alternative?
setup_crontab () {
    crontab -l > CRONTAB_NEW
    found=$( grep "$COMMAND" CRONTAB_NEW )
    if [ -z "$found" ]
    then
        echo "wallpaper_cron adding cron to crontab"
        logger "wallpaper_cron adding cron to crontab"
        # TODO if github cron works, this should just sync
        ENTRY="$MIN $HOUR * * * $COMMAND"
        echo "$ENTRY" >> CRONTAB_NEW
    fi
    crontab CRONTAB_NEW
    rm CRONTAB_NEW
}
