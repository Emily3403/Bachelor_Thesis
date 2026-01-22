from src.analyze.packets import Packet
from src.test_cases.dir_config import DIR_CONFIG
from src.test_cases.test_case import TestCase


def analyze_testcase(it: TestCase) -> None:
    stdin = DIR_CONFIG.localhost_stdin(it).read_text()
    stdout_path = DIR_CONFIG.localhost_stdout(it)

    try:
        stdout = stdout_path.read_text()
    except Exception as e:
        print(f"ERROR reading {stdout_path}: {e!r}")
        return

    packets = []
    for line in stdout.splitlines():
        if line.startswith("Packet "):
            packets.append(Packet.from_json(line[7:]))
        elif line.startswith("Checksum Error"):
            pass

    pass
