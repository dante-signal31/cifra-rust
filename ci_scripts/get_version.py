#!/usr/bin/env python3
import configparser
import sys

CONFIG_FILE = "Cargo.toml"


def get_version(config_file: str = CONFIG_FILE) -> str:
    """ Get version configured at Cargo.toml.

    :param config_file: Path to Cargo.toml.
    :return: Version value at Cargo.toml.
    """
    config = configparser.ConfigParser()
    config.read(CONFIG_FILE)
    return config["package"]["version"]


if __name__ == "__main__":
    version: str = ""

    if len(sys.argv) > 1:
        config_file_path = sys.argv[1]
        version = get_version(config_file_path)
    else:
        version = get_version()

    # Remove quotes before printing.
    version = version[1:-1]
    print(version)

