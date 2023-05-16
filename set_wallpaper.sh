#!/bin/bash

# Set wallpaper to file

# Ensure a base directory was provided.
if [ -z "$1" ]
then
    echo "set_wallpaper.sh \$1 must be the wallpaper file"
    exit 1
fi
WALLPAPER=$(realpath $1)
echo "set_wallpaper WALLPAPER = $WALLPAPER"
logger "set_wallpaper WALLPAPER = $WALLPAPER"

# https://askubuntu.com/questions/742870/background-not-changing-using-gsettings-from-cron
REAL_UID=$(id --real --user)
PID=$(pgrep --euid $REAL_UID gnome-session | head -n 1)
export DBUS_SESSION_BUS_ADDRESS=$(grep -z DBUS_SESSION_BUS_ADDRESS /proc/$PID/environ|cut -d= -f2- | sed -e "s/\x0//g")
logger "set_wallpaper.sh UID = $REAL_UID; PID = $REAL_PID; DBUS = $DBUS_SESSION_BUS_ADDRESS"

# Remove old wallpaper
gsettings set org.gnome.desktop.background picture-uri ""
gsettings set org.gnome.desktop.background picture-uri-dark ""

# Update background
gsettings set org.gnome.desktop.background picture-uri "file://$WALLPAPER"
gsettings set org.gnome.desktop.background picture-uri-dark "file://$WALLPAPER"
gsettings set org.gnome.desktop.background picture-options "spanned"
gsettings set org.gnome.desktop.background show-desktop-icons "true"
logger "set_wallpaper.sh completed"
