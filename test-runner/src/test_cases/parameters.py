import itertools
import string

from src.test_cases.test_case import TestCase

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
PACKET_NUM_DATA_BYTES = [2]

ITERATION = 0

TESTCASES = [
    TestCase(
        iteration=ITERATION,
        test_name=name,
        test_pattern=pattern,
        just_command=command,
        baudrate=baud,
        realtime=rt,
        packet_num_data_bytes=pndb,
    )
    for (name, pattern), baud, command, rt, pndb in itertools.product(TEST_NAME_AND_PATTERNS, BAUDRATES, JUST_COMMANDS, REALTIME, PACKET_NUM_DATA_BYTES)
]
