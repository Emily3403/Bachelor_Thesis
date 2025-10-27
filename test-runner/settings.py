import os
from pathlib import Path

app_path = Path(os.path.dirname(__file__))

TEST_LINE_DELIMITER = "\n"
TEST_LINE_NONCE_DELIMITER = "|"

def delimit(id: int, max_len: int) -> str:
    return f"|{str(id).rjust(max_len, '0')}"

TEST_LOOKAHEAD = 3