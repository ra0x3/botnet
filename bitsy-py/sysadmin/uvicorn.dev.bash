#!/bin/bash

uvicorn bitsy:app --reload \
    --host 127.0.0.1 \
    --port 8000 \
    --workers 1 \
    --timeout-keep-alive 5
