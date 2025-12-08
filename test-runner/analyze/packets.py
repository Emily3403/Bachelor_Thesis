from __future__ import annotations

import json
from dataclasses import dataclass


@dataclass
class Packet:
    seq_num: int
    checksum: int
    data: list[int]

    stats: UARTStats

    @classmethod
    def from_json(cls, it: str):
        loaded = json.loads(it)
        loaded["stats"] = UARTStats(**loaded["stats"])
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
