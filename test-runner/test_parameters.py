import itertools
import string

from test_cases import TestCase

TEST_NAME_AND_PATTERNS = [
    ("ASCII", string.ascii_letters),
    ("Repeated: A", "A")
]

# BAUDRATES = [9600, 19200, 57600, 115200]
BAUDRATES = [57600]
JUST_COMMANDS = [
    "polling-listen",
    "irq-dint-listen",
    "irq-scratch-listen",
]
REALTIME = [False, True]

ITERATION = 0

TESTCASES = [
    TestCase(
        iteration=ITERATION,
        test_name=name,
        test_pattern=pattern,
        just_command=command,
        baudrate=baud,
        realtime=rt,
    )
    for (name, pattern), baud, command, rt in itertools.product(TEST_NAME_AND_PATTERNS, BAUDRATES, JUST_COMMANDS, REALTIME)
]
