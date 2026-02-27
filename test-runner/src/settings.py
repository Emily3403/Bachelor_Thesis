import os
from pathlib import Path

TEST_LINE_DELIMITER = "\n"
TEST_LINE_NONCE_DELIMITER = "|"

WORKING_DIR = Path(os.path.dirname(__file__)) / ".."
CACHE_RESULTS = False  # If False, every testcase is always executed


def delimit(id: int, max_len: int) -> str:
    return f"|{str(id).rjust(max_len, '0')}"


TEST_LOOKAHEAD = 3
