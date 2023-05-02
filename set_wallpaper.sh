#!/bin/bash

WALLPAPER="$HOME/Pictures/wallpaper.png"
TEXT=$(shuf -n 1 quotes.txt | xargs -n9)
files=(./wallpapers/*)
randomfile=$(printf "%s\n" "${files[RANDOM % ${#files[@]}]}")
echo "wallpaper = $randomfile, TEXT = $TEXT"
# randomfile="file://$HOME/dev/dotfiles/wallpaper/epictetus.jpg"
convert "$randomfile" -pointsize 70 -fill white -gravity North -annotate +0+100 "$TEXT" -quality 100 "$WALLPAPER"
# -font "Source Code Pro"
gsettings set org.gnome.desktop.background picture-uri "$WALLPAPER"
gsettings set org.gnome.desktop.background picture-uri-dark "$WALLPAPER"
gsettings set org.gnome.desktop.background picture-options "spanned"
gsettings set org.gnome.desktop.background show-desktop-icons "true"
# xwallpaper --stretch $randomfile
