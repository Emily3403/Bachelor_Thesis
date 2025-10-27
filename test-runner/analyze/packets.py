from dataclasses import dataclass

from test_cases import TestCase


@dataclass
class TestPacket:
    sequence_num: int
    data: str

def analyze_packets(it: TestCase, stdin: str, stdout: str) -> list[TestPacket]:
    pass