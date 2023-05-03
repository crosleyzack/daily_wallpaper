#!/bin/bash

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

# https://askubuntu.com/questions/742870/background-not-changing-using-gsettings-from-cron
REAL_UID=$(id --real --user)
PID=$(pgrep --euid $REAL_UID gnome-session | head -n 1)
export DBUS_SESSION_BUS_ADDRESS=$(grep -z DBUS_SESSION_BUS_ADDRESS /proc/$PID/environ|cut -d= -f2- | sed -e "s/\x0//g")

# Set path to wallpaper
WALLPAPER="$SCRIPT_DIR/wallpaper.png"

# Remove old wallpaper
rm -f $WALLPAPER
gsettings set org.gnome.desktop.background picture-uri ""
gsettings set org.gnome.desktop.background picture-uri-dark ""

# Create new background
TEXT=$(shuf -n 1 $SCRIPT_DIR/quotes.txt | xargs -n9)
files=($SCRIPT_DIR/wallpapers/*)
randomfile=$(printf "%s\n" "${files[RANDOM % ${#files[@]}]}")
logger "Setting wallpaper $randomfile with text $TEXT"
echo "wallpaper = $randomfile, TEXT = $TEXT"
convert "$randomfile" -pointsize 70 -fill white -gravity North -annotate +0+100 "$TEXT" -quality 100 "$WALLPAPER"

# Update background
gsettings set org.gnome.desktop.background picture-uri "file://$WALLPAPER"
gsettings set org.gnome.desktop.background picture-uri-dark "file://$WALLPAPER"
gsettings set org.gnome.desktop.background picture-options "spanned"
gsettings set org.gnome.desktop.background show-desktop-icons "true"
logger "set_wallpaper.sh completed"
