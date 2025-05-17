#!/bin/bash

# Set wallpaper to file

# Ensure a base directory was provided.
if [ -z "$1" ]
then
    echo "set_wallpaper.sh \$1 must be the wallpaper file"
    exit 1
fi
WALLPAPER=$(realpath $1)
echo "set_wallpaper.sh WALLPAPER = $WALLPAPER"
logger "set_wallpaper.sh WALLPAPER = $WALLPAPER"

# https://askubuntu.com/questions/742870/background-not-changing-using-gsettings-from-cron
REAL_UID=$(id --real --user)
PID=$(pgrep --euid $REAL_UID gnome-session | head -n 1)
echo "set_wallpaper.sh1 $PID"
logger "set_wallpaper.sh1 $PID"
PID=$(pgrep -f 'gnome-session' | head -n1)
echo "set_wallpaper.sh2 $PID"
logger "set_wallpaper.sh2 $PID"
PID=$(pgrep --euid $EUID gnome-session)
echo "set_wallpaper.sh3 $PID"
logger "set_wallpaper.sh3 $PID"
export DBUS_SESSION_BUS_ADDRESS=$(grep -z DBUS_SESSION_BUS_ADDRESS /proc/$PID/environ|cut -d= -f2-)
echo "set_wallpaper.sh4 $DBUS_SESSION_BUS_ADDRESS"
logger "set_wallpaper.sh4 $DBUS_SESSION_BUS_ADDRESS"
# export DBUS_SESSION_BUS_ADDRESS=$(grep -z DBUS_SESSION_BUS_ADDRESS /proc/$PID/environ|cut -d= -f2- | sed -e "s/\x0//g")
echo "set_wallpaper.sh UID = $REAL_UID; PID = $REAL_PID; DBUS = $DBUS_SESSION_BUS_ADDRESS"
logger "set_wallpaper.sh UID = $REAL_UID; PID = $REAL_PID; DBUS = $DBUS_SESSION_BUS_ADDRESS"

# Update background
echo "set_wallpaper.sh setting $WALLPAPER"
logger "set_wallpaper.sh setting $WALLPAPER"
gsettings set org.gnome.desktop.background picture-uri "file://$WALLPAPER"
gsettings set org.gnome.desktop.background picture-uri-dark "file://$WALLPAPER"
gsettings set org.gnome.desktop.background picture-options "spanned"
gsettings set org.gnome.desktop.background show-desktop-icons "true"
echo "set_wallpaper.sh completed"
logger "set_wallpaper.sh completed"
dconf update
