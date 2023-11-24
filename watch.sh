#!/bin/bash

# Run npx tailwindcss in the background
npx tailwindcss -i ./input.css -o ./style/main.css --watch &

# Capture the PID of the last background process
TAILWIND_PID=$!

cargo leptos watch

# Wait for the background process to finish
kill $TAILWIND_PID