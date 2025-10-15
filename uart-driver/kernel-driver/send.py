#!/usr/bin/env python3
from argparse import Namespace
from pathlib import Path

from serial import Serial

from utils import parse_args


def write_serial(ser: Serial, s: str):
    enc = s.encode()
    ser.write(enc)
    ser.flush()
    print(f"Sent: {len(enc)} bytes")


def send_string(s: str, baud: int):
    # 9600, 19200, 38400, 57600, 115200, 500000
    with Serial('/dev/ttyS0', baud, timeout=1) as ser:
        ser.reset_output_buffer()
        write_serial(ser, s)


def main(args: Namespace) -> None:
    contents = Path(args.file).read_text()
    send_string(contents, args.baudrate)


if __name__ == "__main__":
    main(parse_args())
