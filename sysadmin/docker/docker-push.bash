#!/bin/bash

set -exo

docker push ralston3/bitsy:rs-latest

docker push ralston3/bitsy:py-latest

docker push ralston3/bitsy:www-latest
