from __future__ import annotations

import json
from dataclasses import dataclass
from enum import Enum


@dataclass
class Packet:
    seq_num: int
    checksum: int
    data: list[int]

    stats: list[UARTStats]
    errors: list[PacketErrors]

    @classmethod
    def from_json(cls, it: str):
        loaded = json.loads(it)
        loaded["stats"] = [UARTStats(**it) for it in loaded["stats"]]
        loaded["errors"] = [PacketErrors(**it) for it in loaded["errors"]]
        return cls(**loaded)


@dataclass
class UARTStats:
    tx_num_bytes: int
    rx_num_bytes: int
    tx_done: bool
    tx_empty: bool

    cts: bool
    rts: bool

    tx_full: bool
    rx_overrun: bool

    tx_idle: bool
    rx_idle: bool

    tx_ready: bool
    rx_ready: bool


class PacketErrors(Enum):
    CHECKSUM_MISMATCH = 0b0001
    SEQNUM_MISMATCH = 0b0010
