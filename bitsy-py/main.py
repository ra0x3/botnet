import argparse
from typing import *


def _extract_arg(kwargs: Dict[str, Any], short: str, long: str) -> Any:
    return kwargs.get(short) or kwargs[long]

def main(config_path: str, password: Optional[str] = None, wallet: Optional[str] = None) -> int:

    return 0


if __name__ == "__main__":

    parser = argparse.ArgumentParser(description="bitsy.")
    parser.add_argument("-c", "--config", type=str, required=True, help="Bitsy configuration file.")
    parser.add_argument(
        "-p", "--password", type=str, help="Password for wallet file"
    )
    parser.add_argument("-w", "--wallet", type=str, help="Location of wallet")

    kwargs = vars(parser.parse_args())

    config = _extract_arg(kwargs, "c", "config")
    password = _extract_arg(kwargs, "p", "password")
    wallet = _extract_arg(kwargs, "w", "wallet")

    main(config_path=config, password=password, wallet=wallet)
