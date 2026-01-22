#!/usr/bin/env python3
from src.analyze import analyze_testcase
from src.analyze.export import export_testcase_data
from src.test_cases.parameters import TESTCASES
from src.utils import load_env


def main() -> None:
    env = load_env()
    print(f"Testing {len(TESTCASES)} items...")

    for testcase in TESTCASES:
        testcase.run(env)
        analyze_testcase(testcase)
        export_testcase_data(testcase)


if __name__ == '__main__':
    main()
