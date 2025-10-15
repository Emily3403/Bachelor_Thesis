import sys
from argparse import Namespace, ArgumentParser


def parse_args() -> Namespace:
    parser = ArgumentParser("send" if "send.py" in sys.argv[0] else "listen")

    parser.add_argument("-b", "--baudrate", help="The baudrate", type=int, default=9600)
    parser.add_argument("-f", "--file", help="The file to transmit", type=str, default="./str.txt")

    return parser.parse_args()
