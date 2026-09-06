#!/bin/bash

FILE_PATH=$(realpath $BASH_SOURCE)
DIR_PATH=$(dirname $FILE_PATH)
REPO_DIR=$(realpath "$DIR_PATH/..")
ASSET_DIR=$(realpath "$REPO_DIR/assets")
DATA_DIR=$(realpath "$REPO_DIR/data")

# Update permissions
chmod 644 $DATA_DIR/quotes.json
chmod 755 $ASSET_DIR/wallpapers
find $ASSET_DIR/wallpapers -type f -exec chmod 644 {} \;

# Dumb permissions stuff
# sudo chown $USER:$USER ~/.config/dconf -R
chmod u+w ~/.config/dconf -R
echo "updated permissions"

# Pull wallpaper from remote and set it, via the taskfile
RUN="task -d $REPO_DIR"
COMMAND="sh -c \"$RUN sync && $RUN set\""

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
DESKTOP_FILE="/wallpaper.desktop"
APP_DIR="$HOME/.local/share/applications"
mkdir -p $AUTOSTART_DIR
rm -f $AUTOSTART_DIR/$DESKTOP_FILE
ln -s $WALLPAPER_FILE $AUTOSTART_DIR/$DESKTOP_FILE
mkdir -p $APP_DIR
rm -f $APP_DIR/$DESKTOP_FILE
ln -s $WALLPAPER_FILE $APP_DIR/$DESKTOP_FILE
echo "Set to run at startup"
echo "Install complete"