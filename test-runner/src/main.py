#!/usr/bin/env python3
import time
from logging import error

from src.analyze import analyze_testcase
from src.analyze.export import export_testcase_data
from src.raspi.io import raspi_push_fs
from src.test_cases.parameters import TESTCASES
from src.test_cases.test_case import TestCase
from src.utils import load_env


def check_testcases_compiled_successfully(testcases: list[TestCase]) -> bool:
    return sum(abs(test.compile_command.wait()) for test in testcases) == 0


def main() -> None:
    env = load_env()
    print(f"Testing {len(TESTCASES)} items...")

    # We only ever want to execute testcases sequentially because there is only one hardware device. Run them all first as fast as possible though
    for testcase in TESTCASES:
        testcase.try_compile(env)
        testcase.generate_test_data(env)


    if not check_testcases_compiled_successfully(TESTCASES):
        error("Could not compile Testcase: ")
        return

    raspi_push_fs(env)

    for testcase in TESTCASES:
        testcase.run(env)

    for testcase in TESTCASES:
        analyze_testcase(testcase)
        export_testcase_data(testcase)


if __name__ == '__main__':
    main()
