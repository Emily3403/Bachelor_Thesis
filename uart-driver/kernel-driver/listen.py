#!/usr/bin/env python3

from serial import Serial


def read_indefinitely():
    print("Listening indefinitely...", flush=True)
    buf = b""
    with Serial('/dev/ttyS0', 9600) as ser:
        while True:
            it = ser.readline()
            try:
                print(f"{it.decode()}")
            except Exception:
                print("Decoding failed!")

if __name__ == "__main__":
    read_indefinitely()
