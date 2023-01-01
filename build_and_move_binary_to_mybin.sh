#!/bin/bash

set -eo pipefail;

cargo build -r;

if ! [[ -d ~/.mybin ]]; then
	mkdir ~/.mybin;
fi

cp ./target/release/chapter_checker $HOME/.mybin/

echo "Done";

exit 0;
