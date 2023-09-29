#!/usr/bin/env bash

(cd easypwned/downloader && cargo install --path .)

easypwned_haveibeenpwned_downloader --sink-bloom-file=database/hibp.bloom --sink-stdout > database/hibp.txt

date -Idate > database/last-updated.txt
