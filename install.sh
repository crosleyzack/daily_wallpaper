#!/bin/bash

sudo apt install imagemagick

chmod 777 ./set_wallpaper.sh
chmod 777 ./create_wallpaper.sh
chmod 777 ./sync_wallpaper.sh
chmod 777 ./server.sh
chmod 777 ./wallpaper_cron.sh
chmod 666 quotes.txt
chmod 777 wallpapers
find wallpapers -type f -exec chmod 666 {} \;

# Dumb permissions stuff
sudo chown $USER:$USER ~/.config/dconf -R
chmod u+w ~/.config/dconf -R
