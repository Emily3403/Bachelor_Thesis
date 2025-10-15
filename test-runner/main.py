#!/usr/bin/env python3
from dataclasses import dataclass

from test_cases import TESTCASES
from utils import load_env


def main() -> None:
    env = load_env()

    for testcase in TESTCASES[:1]:
        testcase.run(env)
    pass


if __name__ == '__main__':
    main()