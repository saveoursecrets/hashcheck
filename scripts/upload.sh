#!/usr/bin/env bash

set -e

BUCKET=${1:-hibp.saveoursecrets.com}
aws s3 sync ./database s3://$BUCKET
