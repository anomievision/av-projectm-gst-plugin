#!/bin/bash

export GST_PLUGIN_PATH=`pwd`/target/debug

# Tasks
# - Read audio file, convert to audio/x-raw
# - Feed audio/x-raw into projectm, which will output a video/x-raw
# - Write video/x-raw to .mp4 file

gst-launch-1.0 \
    filesrc location=`pwd`/sample/audio/AlexiAction-Lifelike.mp3 \
    ! decodebin ! audioresample ! audioconvert ! audio/x-raw,format=S24LE,rate=48000 ! \
    projectm preset_file=`pwd`/sample/presets/preset-1.milk ! x264enc ! mp4mux ! \
    filesink location=`pwd`/sample/output.mp4