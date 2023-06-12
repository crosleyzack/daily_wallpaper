#!/bin/bash

# Generate new wallpaper

# Set constants.
DEFAULT_WORDS_PER_LINE=11
DEFAULT_FONT_SIZE=85
DEFAULT_WALLPAPER_NAME="wallpaper.png"
DEFAULT_FILE_REGEX='""'
DEFAULT_FONT_COLOR='"white"'
DEFAULT_GRAVITY='"North"'
WALLPAPER_NAME="desktop.png"
MOBILE_NAME="mobile.png"
MOBILE_WORDS_PER_LINE=6
QUOTE_FILE="quotes.json"
WALLPAPER_DIR_NAME="wallpapers"

# Set JSON Keys. NOTE: for some reason, `_` in keys creates some issues
SIZE_KEY="fontSize"
WORDS_PER_LINE_KEY="wordsPerLine"
REGEX_KEY="fileRegex"
COLOR_KEY="fontColor"
GRAVITY_KEY="gravity"
QUOTE_KEY="quote"
AUTHOR_KEY="author"

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
    WALLPAPER_NAME="$DEFAULT_WALLPAPER_NAME"
else
    WALLPAPER_NAME="$2"
fi
logger "create_wallpaper.sh base_dir = $BASE_DIR, wallpaper_name = $WALLPAPER_NAME"

# https://askubuntu.com/questions/742870/background-not-changing-using-gsettings-from-cron
REAL_UID=$(id --real --user)
PID=$(pgrep --euid $REAL_UID gnome-session | head -n 1)
export DBUS_SESSION_BUS_ADDRESS=$(grep -z DBUS_SESSION_BUS_ADDRESS /proc/$PID/environ|cut -d= -f2- | sed -e "s/\x0//g")
logger "create_wallpaper.sh UID = $REAL_UID; PID = $REAL_PID; DBUS = $DBUS_SESSION_BUS_ADDRESS"

# Set path to wallpaper files
QUOTE_FILE="$BASE_DIR/$QUOTE_FILE"
WALLPAPERS_DIR="$BASE_DIR/$WALLPAPER_DIR_NAME"
WALLPAPER="$BASE_DIR/$WALLPAPER_NAME"
MOBILE="$BASE_DIR/$MOBILE_NAME"

# Work on swapping to json
FILE_TEXT=$(<$QUOTE_FILE)
THIS_JSON=$( echo $FILE_TEXT | jq -c '.[]' | shuf -n 1 )
WORDS_PER_LINE=$( jq ".$WORDS_PER_LINE_KEY // $DEFAULT_WORDS_PER_LINE" <<< "$THIS_JSON" | tr -d '"' )
FILE_REGEX=$( jq ".$REGEX_KEY // $DEFAULT_FILE_REGEX" <<< "$THIS_JSON" | tr -d '"' )
FONT_SIZE=$( jq ".$SIZE_KEY // $DEFAULT_FONT_SIZE" <<< "$THIS_JSON" | tr -d '"' )
GRAVITY=$( jq ".$GRAVITY_KEY // $DEFAULT_GRAVITY" <<< "$THIS_JSON" | tr -d '"' )
FONT_COLOR=$( jq ".$COLOR_KEY // $DEFAULT_FONT_COLOR" <<< "$THIS_JSON" | tr -d '"' )
QUOTE=$( jq ".$QUOTE_KEY" <<< "$THIS_JSON" | tr -d '"' )
AUTHOR=$( jq ".$AUTHOR_KEY" <<< "$THIS_JSON" | tr -d '"' )
logger "create_wallpaper.sh read quotes line QUOTE='$QUOTE', AUTHOR='$AUTHOR', REGEX='$FILE_REGEX', FONT_SIZE='$FONT_SIZE', WORDS_PER_LINE='$WORDS_PER_LINE', GRAVITY='$GRAVITY', FONT_COLOR='$FONT_COLOR'"
echo "create_wallpaper.sh read quotes line QUOTE='$QUOTE', AUTHOR='$AUTHOR', REGEX='$FILE_REGEX', FONT_SIZE='$FONT_SIZE', WORDS_PER_LINE='$WORDS_PER_LINE', GRAVITY='$GRAVITY', FONT_COLOR='$FONT_COLOR'"

# Select random file from regex provided.
RANDOM_FILE=$( find $WALLPAPERS_DIR/* -regex ".*$FILE_REGEX.*" | shuf -n 1 )

# Create desktop wallpaper
logger "create_wallpaper.sh Generating wallpaper $RANDOM_FILE, quote = $QUOTE, author = $AUTHOR"
echo "create_wallpaper.sh Generating wallpaper = $RANDOM_FILE, quote = $QUOTE, author = $AUTHOR"
# We have to explicitly set the delimiter so xargs will ignore single quotes and other reserved chars in string.
DESKTOP_TEXT=$(echo $QUOTE | xargs -n $WORDS_PER_LINE -d ' ')
DESKTOP_TEXT="$DESKTOP_TEXT\n$AUTHOR"
convert "$RANDOM_FILE" -pointsize "$FONT_SIZE" -fill "$FONT_COLOR" -gravity "$GRAVITY" -annotate +0+100 "$DESKTOP_TEXT" -quality 100 "$WALLPAPER"

# Create mobile wallpaper
# MOBILE_TEXT=$(echo $TEXT | xargs -n $MOBILE_WORDS_PER_LINE)
# convert "$RANDOM_FILE" -pointsize "$FONT_SIZE" -fill white -gravity North -annotate +0+100 "$MOBILE_TEXT" -quality 100 -crop 1080x2160 "$MOBILE"

echo "create_wallpaper.sh Successfully created new wallpaper"
logger "create_wallpaper.sh Successfully created new wallpaper"
