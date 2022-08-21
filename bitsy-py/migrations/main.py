import os
import datetime as dt
import sys
import importlib

workdir = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
if workdir not in sys.path:
    sys.path.append(workdir)

migrations_dir = os.path.dirname(os.path.abspath(__file__))
if migrations_dir not in sys.path:
    sys.path.append(migrations_dir)

from bitsy import config
from migrations import migration_name, migrations_dir, logger


def has_migration(name):
    cursor = config.connection.cursor()
    cursor.execute("SELECT * FROM __bitsy_migrations;")
    return cursor.fetchall()


def migrate_up(*args, **kwargs):
    migration_files = os.listdir(migrations_dir)
    migration_files.sort()

    ignore = {"main.py", "__init__.py", "__pycache__"}

    for file in migration_files:
        if os.path.basename(file) in ignore:
            continue

        name = migration_name(file)

        if has_migration(name):
            logger.info(f"👍 {name}")
            continue

        module = importlib.import_module(f"migrations.{name}")
        logger.info(f"Migrating {name}")
        module.up()
        config.connection.cursor().execute(
            f"INSERT INTO __bitsy_migrations (name, created_at) VALUES ('{name}', '{dt.datetime.now().strftime('%Y-%m-%d %H:%M:%S')}');"
        )
        logger.info("OK ✅")

    config.connection.commit()


if __name__ == "__main__":

    migrate_up()
