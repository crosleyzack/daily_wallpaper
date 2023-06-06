#!/bin/bash

# Generate new wallpaper

# Set number of words per line (indicated by space boundaries).
WORDS_PER_LINE=11
FONT_SIZE=85
WALLPAPER_NAME="desktop.png"
MOBILE_NAME="mobile.png"
MOBILE_WORDS_PER_LINE=6
QUOTE_FILE="quotes.csv"
DELIMITER=";"

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
TEXT=$( tail -n +2 $BASE_DIR/$QUOTE_FILE | shuf -n 1 )
AUTHOR=$(echo $TEXT | cut -d $DELIMITER -f 1)
FILE_REGEX=$(echo $TEXT | cut -d $DELIMITER -f 2)
QUOTE=$(echo $TEXT | cut -d $DELIMITER -f 3)
logger "create_wallpaper.sh read quotes line QUOTE='$QUOTE', AUTHOR='$AUTHOR', REGEX='$FILE_REGEX'"
echo "create_wallpaper.sh read quotes line QUOTE='$QUOTE', AUTHOR='$AUTHOR', REGEX='$FILE_REGEX'"
# FILES=( $BASE_DIR/wallpapers/* )
RANDOM_FILE=$( find $BASE_DIR/wallpapers/* -regex ".*$FILE_REGEX.*" | shuf -n 1)
# logger "create_wallpaper.sh File = $RANDOM_FILE, TEXT = $TEXT"
# RANDOM_FILE=$(printf "%s\n" "${FILES[RANDOM % ${#FILES[@]}]}")
# RANDOM_FILE=${FILES[ $RANDOM % ${#FILES[@]} ]}

# Create desktop wallpaper
logger "create_wallpaper.sh Generating wallpaper $RANDOM_FILE with text $TEXT"
echo "create_wallpaper.sh Generating wallpaper = $RANDOM_FILE, quote = $QUOTE, author = $AUTHOR"
rm -f $WALLPAPER
# We have to explicitly set the delimiter so xargs will ignore single quotes and other reserved chars in string.
DESKTOP_TEXT=$(echo $QUOTE | xargs -n $WORDS_PER_LINE -d ' ')
DESKTOP_TEXT="$DESKTOP_TEXT\n$AUTHOR"
convert "$RANDOM_FILE" -pointsize "$FONT_SIZE" -fill white -gravity North -annotate +0+100 "$DESKTOP_TEXT" -quality 100 "$WALLPAPER"

# Create mobile wallpaper
# MOBILE_TEXT=$(echo $TEXT | xargs -n $MOBILE_WORDS_PER_LINE)
# convert "$RANDOM_FILE" -pointsize "$FONT_SIZE" -fill white -gravity North -annotate +0+100 "$MOBILE_TEXT" -quality 100 -crop 1080x2160 "$MOBILE"

echo "create_wallpaper.sh Successfully created new wallpaper"
logger "create_wallpaper.sh Successfully created new wallpaper"
