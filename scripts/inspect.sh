#!/bin/bash

cargo build

# Inspect gst plugin
gst-inspect-1.0 `pwd`/target/debug/libavprojectmgstplugin.so