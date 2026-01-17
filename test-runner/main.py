#!/usr/bin/env python3
from analyze import analyze_testcase
from test_parameters import TESTCASES
from utils import load_env


def main() -> None:
    env = load_env()
    print(f"Testing {len(TESTCASES)} items...")

    for testcase in TESTCASES:
        testcase.run(env)
        analyze_testcase(testcase)
        # TODO: Export testcase data (stats) to LaTeX .dat → pgfplots


if __name__ == '__main__':
    main()
