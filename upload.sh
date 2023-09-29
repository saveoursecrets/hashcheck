#!/usr/bin/env bash

BUCKET=${1:-hibp.saveoursecrets.com}
AWS_PROFILE=sos-release aws s3 sync ./database s3://$BUCKET
