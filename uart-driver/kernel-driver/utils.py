from __future__ import annotations

import sys
from argparse import Namespace, ArgumentParser
from typing import Literal

PACKET_DATA_LENGTH = 1
BYTEORDER: Literal['big' | 'little'] = "big"


def parse_args() -> Namespace:
    parser = ArgumentParser("send" if "send.py" in sys.argv[0] else "listen")

    parser.add_argument("-b", "--baudrate", help="The baudrate", type=int, default=9600)
    parser.add_argument("-f", "--file", help="The file to transmit", type=str, default="./str.txt")
    parser.add_argument("--num-data-bytes", help="Number of data bytes", type=int, default=1)

    return parser.parse_args()


def calculate_checksum(data: bytes) -> bytes:
    assert len(data) == PACKET_DATA_LENGTH

    return sum(byte.bit_count() for byte in data).to_bytes()
