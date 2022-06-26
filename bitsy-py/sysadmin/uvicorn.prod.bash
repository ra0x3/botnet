#!/bin/bash

uvicorn bitsy:app \
    --host 0.0.0.0 \
    --port 8000 \
    --workers 3 \
    --timeout-keep-alive 5
