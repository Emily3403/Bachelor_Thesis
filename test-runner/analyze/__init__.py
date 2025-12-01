from analyze.block_result import fit_into_blocks
from analyze.packets import analyze_packets
from test_cases import TestCase


def analyze_testcase(it: TestCase) -> None:
    stdin = it.stdin_path().read_text()
    try:
        stdout = it.stdout_path().read_text()
    except Exception as e:
        pass

    packets = analyze_packets(it, stdin, stdout)

    blocks = fit_into_blocks(it, stdin, stdout)
