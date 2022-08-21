import datetime as dt
from bitsy import config
from migrations import logger


def up():
    config.connection.cursor().execute(
        "CREATE TABLE IF NOT EXISTS __bitsy_migrations (id SERIAL PRIMARY KEY, name varchar NOT NULL UNIQUE, created_at TIMESTAMP);"
    )
