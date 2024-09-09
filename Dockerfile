FROM debian:bookworm

# docker build . -t quote-wallpaper
# docker run --user 1000:1000 --mount "type=bind,source=$(pwd),target=/app" quote-wallpaper

ARG FILENAME="wallpaper.png"
ENV FILENAME=${FILENAME}

RUN apt-get update && apt-get upgrade
RUN apt-get install -y jq imagemagick procps
RUN apt-get install -y vim

# RUN mkdir -p /app
# COPY create_wallpaper.sh /app
# COPY quotes.json /app
# COPY wallpapers /app/wallpapers/
# VOLUME ["/app/out"]

CMD /app/create_wallpaper.sh /app ${FILENAME}
