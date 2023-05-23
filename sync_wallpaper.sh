#!/bin/bash

# Sync repository.
# TODO this is untested!
# TODO this probably requires some token to bypass yubikey

# Ensure a base directory was provided.
if [ -z "$1" ]
then
    echo "sync_wallpaper.sh \$1 must be the wallpaper repo"
    exit 1
fi
REPO_DIR=$(realpath $1)
if [ -z "$2" ]
then
    echo "sync_wallpaper.sh \$2 must be the wallpaper file"
    exit 1
fi
WALLPAPER="$REPO_DIR/$2"

# https://askubuntu.com/questions/742870/background-not-changing-using-gsettings-from-cron
REAL_UID=$(id --real --user)
PID=$(pgrep --euid $REAL_UID gnome-session | head -n 1)
export DBUS_SESSION_BUS_ADDRESS=$(grep -z DBUS_SESSION_BUS_ADDRESS /proc/$PID/environ|cut -d= -f2- | sed -e "s/\x0//g")
logger "set_wallpaper.sh UID = $REAL_UID; PID = $REAL_PID; DBUS = $DBUS_SESSION_BUS_ADDRESS"

cd $REPO_DIR
# if origin has commits we do not.
git fetch
git rebase
# if we have commits origin does not.
git add *
# generate string with date
git commit -m "Updated wallpaper"
git push
