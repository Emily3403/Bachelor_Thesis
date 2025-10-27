import re
from dataclasses import dataclass

from more_itertools import consecutive_groups

from test_cases import TestCase
from utils import find_nonces


# Key Metrics:
#   - Bit Error Rate (BER): The percentage of bytes received in error.
#   - Framing Errors → "The Module does not check for framing errors"
#   - Overrun Errors
#
# Optional:
#   - CPU Utilization
#   - IRQs per byte transferred
#
# How to measure?
#   - (Sustainable) Throughput
#       - Need some way of detecting "first byte" and "last byte"
#
#   - Latency
#     - New binary: recv byte → send byte
#     → Jitter




@dataclass
class BlockResult:
    id: int
    in_pattern: str
    out_pattern: str

    line: str
    correctly_delimited: bool


def fit_into_blocks(test: TestCase, stdin: str, stdout: str) -> dict[int, BlockResult]:
    """
    Assumption: The stdout pattern will never be longer that the original pattern
    """
    stdin_nonces = set(find_nonces(stdin))
    stdout_nonces = set()

    blocks = {}

    # Add all correctly delimited output lines
    for out_line in stdout.splitlines(keepends=True):
        search = re.search("^(\w*)\|(\d+)\n$", out_line)
        if search is None:
            continue

        stdout_nonces.add(search.group(2))
        blocks[int(search.group(2))] = BlockResult(
            id=int(search.group(2)),
            in_pattern=test.test_pattern,
            out_pattern=search.group(1),
            line=out_line,
            correctly_delimited=True
        )

    for _group in consecutive_groups(sorted(list({int(it) for it in stdin_nonces} - {int(it) for it in stdout_nonces}))):
        group = list(_group)
        assert group[0] != 0

        prev_result = blocks[group[0] - 1]
        next_result = blocks[group[-1] + 1]

        start_index = stdout.find(prev_result.line) + len(prev_result.line)
        end_index = stdout.find(next_result.line)

        blocks[group[0]] = BlockResult(
            id=group[0],
            in_pattern=test.test_pattern,
            out_pattern=stdout[start_index:end_index],
            line=stdout[start_index:end_index],
            correctly_delimited=False
        )

    # Sanity check: Every line in order should make up the stdout back up again.
    resulting_stdout = "".join(it[1].line for it in sorted(blocks.items(), key=lambda it: it[1].id))
    assert resulting_stdout == stdout

    return blocks
