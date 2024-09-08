#!/bin/bash
if [ $# -eq 0 ]; then
    adb shell wm size
else
    for dev in "$@"; do
        echo "Specified device: $dev"
        adb -s $dev shell wm size
    done
fi
