from analyze.block_result import fit_into_blocks
from analyze.packets import Packet
from test_cases import TestCase


def analyze_testcase(it: TestCase) -> None:
    stdin = it.stdin_path().read_text()
    try:
        stdout = it.stdout_path().read_text()
    except Exception as e:
        print(f"ERROR reading {it.stdout_path()}: {e!r}")
        return

    packets = []
    for line in stdout.splitlines():
        if line.startswith("Packet "):
            packets.append(Packet.from_json(line[7:]))
        elif line.startswith("Checksum Error"):
            pass

    pass


