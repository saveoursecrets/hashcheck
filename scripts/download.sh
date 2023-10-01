#!/usr/bin/env bash

(cd easypwned/downloader && cargo install --path .)

easypwned_haveibeenpwned_downloader --sink-bloom-file=database/hibp.bloom

date -Idate > database/last-updated.txt
wc -c < database/hibp.bloom | tr -d ' ' > database/size.txt
