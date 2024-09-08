#!/bin/bash
if [ $# -eq 0 ]; then
    scrcpy
else
    for ip in "$@"; do
        echo "Specified address: $ip"
        adb connect "$ip":5555
        scrcpy -s "$ip":5555
    done
fi
