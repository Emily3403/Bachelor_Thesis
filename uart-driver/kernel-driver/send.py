#!/usr/bin/env python3
# TODO: Accept strings from command line

from serial import Serial


def write_serial(ser: Serial, s: str):
    enc = (s).encode()
    ser.write(enc)
    ser.flush()
    print(f"Sent: {len(enc)} bytes")


def send_string(s: str, baud: int):
    # 9600, 19200, 38400, 57600, 115200, 500000
    with Serial('/dev/ttyS0', baud, timeout=1) as ser:
        ser.reset_output_buffer()
        write_serial(ser, s)

if __name__ == "__main__":
    with open("baud.txt") as f:
        baud = int(f.readline().strip())

    with open("str.txt") as f:
        send_string(f.readline().strip(), baud)

