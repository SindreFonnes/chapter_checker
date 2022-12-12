#!/bin/bash

set -eo pipefail;

cargo build -r;

cp ./target/release/chapter_checker $HOME/.mybin/

echo "Done";

exit 0;
