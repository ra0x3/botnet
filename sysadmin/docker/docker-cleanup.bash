#!/bin/bash

set -exo

docker container prune -f

docker volume prune -f

docker image prune -f
