import os
import sys
import logging

workdir = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
if workdir not in sys.path:
    sys.path.append(workdir)

migrations_dir = os.path.dirname(os.path.abspath(__file__))
if migrations_dir not in sys.path:
    sys.path.append(migrations_dir)

logger = logging.getLogger("migrations")


def migration_name(filename):
    return os.path.splitext(filename)[0]
