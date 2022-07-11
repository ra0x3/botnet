#!/bin/bash

DOTENV=env/.env.dev ENV=dev CONFIG=config/bitsy.dev.yaml \
    uvicorn bitsy:app --reload \
        --host 127.0.0.1 \
        --port 8000 \
        --workers 3 \
        --use-colors \
        --timeout-keep-alive 5
