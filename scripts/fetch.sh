#!/usr/bin/env bash

set -e

BUCKET=${1:-hibp.saveoursecrets.com}
AWS_PROFILE=sos-release aws s3 sync s3://$BUCKET ./database
