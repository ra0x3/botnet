FROM ubuntu:22.04

WORKDIR /app

ENV DEBIAN_FRONTEND noninteractive

RUN apt update && apt-get upgrade -y
RUN apt install software-properties-common -y
RUN add-apt-repository ppa:deadsnakes/ppa -y
RUN apt install -y \
    libpq-dev \
    build-essential \
    python3.9 \
    python3.9-dev \
    python3.9-venv \
    python3.9-distutils \
    python3.9-lib2to3 \
    python3.9-gdbm

COPY . .

RUN python3.9 -m venv venv
RUN ./venv/bin/pip install -r requirements/requirements.txt

EXPOSE 8000

CMD exec ./venv/bin/python3.9 main.py --api-host "${API_HOST}" --api-port "${API_PORT}" --pg-host "${PG_HOST}" --pg-port "${PG_PORT}" --pg-user "${PG_USER}" --pg-password "${PG_PASSWORD}"

# docker run -it -p 8000:8000 -e "PG_HOST=6.tcp.ngrok.io" -e "PG_PORT=19705" -e "PG_USER=postgres" -e "pg_database=bitsy" ralston3/bitsy:py-latest
