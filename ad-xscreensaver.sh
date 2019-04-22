#!/usr/bin/env bash
cd "${0%/*}"
xterm -geometry 1360x768 -bg Black -fg White -into $XSCREENSAVER_WINDOW -e ./ad.py