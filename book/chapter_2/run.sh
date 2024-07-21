#!/bin/bash

cargo watch -x -c run

# Check if compilation was successful
if [ $? -eq 0 ]; then
  # Run the compiled program
  ./output
else
  echo "Compilation failed."
fi
