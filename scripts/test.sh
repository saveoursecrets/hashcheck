#!/usr/bin/env bash

#curl -vXPOST http://localhost:3342/ -H 'Content-Type: application/json' --data-binary @./fixtures/hashes.json

curl -vXPOST https://hashcheck.saveoursecrets.com -H 'Content-Type: application/json' --data-binary @./fixtures/hashes.json
