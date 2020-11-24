#!/bin/bash

sudo add-apt-repository ppa:jonathonf/ffmpeg-4 -y
sudo apt-get update -q
sudo apt-get install \
  ffmpeg \
  libavcodec-dev \
  libavformat-dev \
  libavutil-dev \
  libavdevice-dev \
  libavfilter-dev \
  libavresample-dev \
  libpostproc-dev \
  libswscale-dev -y
