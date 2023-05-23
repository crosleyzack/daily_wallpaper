#!/bin/bash

# Generate new wallpaper

# Set number of words per line (indicated by space boundaries).
WORDS_PER_LINE=11
FONT_SIZE=85
WALLPAPER_NAME="desktop.png"
MOBILE_NAME="mobile.png"
MOBILE_WORDS_PER_LINE=6

# Ensure a base directory was provided.
if [ -z "$1" ]
then
    echo "create_wallpaper.sh \$1 must be base directory"
    exit 1
fi
BASE_DIR=$(realpath $1)
if [ -z "$2" ]
then
    # We will assume wallpaper name if not provided
    WALLPAPER_NAME="wallpaper.png"
else
    WALLPAPER_NAME="$2"
fi
logger "create_wallpaper.sh base_dir = $BASE_DIR, wallpaper_name = $WALLPAPER_NAME"
# SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

# https://askubuntu.com/questions/742870/background-not-changing-using-gsettings-from-cron
REAL_UID=$(id --real --user)
PID=$(pgrep --euid $REAL_UID gnome-session | head -n 1)
export DBUS_SESSION_BUS_ADDRESS=$(grep -z DBUS_SESSION_BUS_ADDRESS /proc/$PID/environ|cut -d= -f2- | sed -e "s/\x0//g")
logger "create_wallpaper.sh UID = $REAL_UID; PID = $REAL_PID; DBUS = $DBUS_SESSION_BUS_ADDRESS"

# Set path to wallpaper file.
WALLPAPER="$BASE_DIR/$WALLPAPER_NAME"
MOBILE="$BASE_DIR/$MOBILE_NAME"

# Get text and base image
TEXT=$(shuf -n 1 $BASE_DIR/quotes.txt)
FILES=( $BASE_DIR/wallpapers/* )
logger "create_wallpaper.sh Files = $FILES, TEXT = $TEXT"
RANDOM_FILE=$(printf "%s\n" "${FILES[RANDOM % ${#FILES[@]}]}")

# Create desktop wallpaper
logger "create_wallpaper.sh Generating wallpaper $RANDOM_FILE with text $TEXT"
echo "create_wallpaper.sh Generating wallpaper = $RANDOM_FILE, TEXT = $TEXT"
rm -f $WALLPAPER
DESKTOP_TEXT=$(echo $TEXT | xargs -n $WORDS_PER_LINE)
convert "$RANDOM_FILE" -pointsize "$FONT_SIZE" -fill white -gravity North -annotate +0+100 "$DESKTOP_TEXT" -quality 100 "$WALLPAPER"

# Create mobile wallpaper
# MOBILE_TEXT=$(echo $TEXT | xargs -n $MOBILE_WORDS_PER_LINE)
# convert "$RANDOM_FILE" -pointsize "$FONT_SIZE" -fill white -gravity North -annotate +0+100 "$MOBILE_TEXT" -quality 100 -crop 1080x2160 "$MOBILE"

echo "create_wallpaper.sh Successfully created new wallpaper"
logger "create_wallpaper.sh Successfully created new wallpaper"
