#!/usr/bin/env bash

(cd easypwned/downloader && cargo install --path .)

easypwned_haveibeenpwned_downloader --sink-bloom-file=hibp.bloom --sink-stdout > hibp.txt

echo date > last-updated.txt
