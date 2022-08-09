#!/bin/bash

for CRATE in $(find . -maxdepth 1 -type d -printf '%P\n'); do
    echo "Cleaning $CRATE..."
    cd $CRATE && cargo clean && cd ..
done