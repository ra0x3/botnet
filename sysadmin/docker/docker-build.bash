#!/bin/bash

set -exo

# IMPORTANT: run from repo root

rootdir=$PWD

cd bitsy-rs
docker build -t ralston3/bitsy:rs-latest -f $rootdir/sysadmin/docker/Dockerfile.bitsy.rs .

cd $rootdir/bitsy-py
docker build -t ralston3/bitsy:py-latest -f $rootdir/sysadmin/docker/Dockerfile.bitsy.py .

cd $rootdir/bitsy-www
docker build -t ralston3/bitsy:www-latest -f $rootdir/sysadmin/docker/Dockerfile.bitsy.www .
