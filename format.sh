#!/bin/bash
# RUN BEFORE chmod +x format.sh

format_directory() {
    leptosfmt "$1"
}

if [ "$#" -eq 0 ]; then
    echo "Usage: $0 [--all] <directory>"
    exit 1
fi

if [ "$1" == "--all" ]; then
    format_directory .
elif [ "$#" -eq 1 ]; then
    format_directory "$1"
else
    echo "Usage: $0 [--all] <directory>"
    exit 1
fi
