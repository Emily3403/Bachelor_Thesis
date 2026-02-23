#!/usr/bin/env python3
from argparse import Namespace
from itertools import batched
from pathlib import Path
from serial import Serial

from utils import parse_args, calculate_checksum, BYTEORDER, PACKET_DATA_LENGTH


def write_serial(ser: Serial, s: str):
    encoded = s.encode()

    for i, data in enumerate(batched(encoded, PACKET_DATA_LENGTH), start=5):
        data_to_send = bytes(data)

        ser.write((i % 255).to_bytes(1, BYTEORDER))  # seq_num
        ser.write(calculate_checksum(data_to_send))  # checksum
        ser.write(data_to_send)  # data

    ser.flush()
    print(f"Sent: {i * (2 + PACKET_DATA_LENGTH)} bytes")


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
