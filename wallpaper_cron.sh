#!/bin/bash

# When do you want cronjob to run each day
HOUR=06
MIN=00

# Get executable to run
SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
echo "wallpaper_cron SCRIPT_DIR = $SCRIPT_DIR"
logger "wallpaper_cron SCRIPT_DIR = $SCRIPT_DIR"
CREATE_EXE="$SCRIPT_DIR/create_wallpaper.sh"
SET_EXE="$SCRIPT_DIR/set_wallpaper.sh"

# Set new wallpaper since this is run on startup.
bash $CREATE_EXE "$SCRIPT_DIR" "wallpaper.png"
bash $SET_EXE "$SCRIPT_DIR" "wallpaper.png"

# Add new cronjob if doesn't already exist
# sh crontab -e
crontab -l > crontab_new
found=$( grep $EXE crontab_new )
if [ -z "$found" ]
then
    echo "wallpaper_cron adding cron to crontab"
    logger "wallpaper_cron adding cron to crontab"
    echo "$MIN $HOUR * * * $EXE $SCRIPT_DIR" >> crontab_new
fi
crontab crontab_new
rm crontab_new
