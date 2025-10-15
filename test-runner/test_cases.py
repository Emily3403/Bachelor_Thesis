import itertools
import os
import string
from dataclasses import dataclass, field
from time import sleep
from typing import Any
from subprocess import Popen, PIPE

from dotenv import dotenv_values, load_dotenv


@dataclass
class TestCase:
    """
    An Instance of a TestCase, having all the information needed for executing it
    """
    test_pattern: str
    baudrate: int
    realtime: bool

    just_listen_command: list[str]
    listen_command: None | Popen = field(default=None)
    send_command: None | Popen = field(default=None)

    def run(self, env: dict[Any, Any]) -> None:
        self.listen_command = Popen(self.just_listen_command, env=env,stdout=PIPE, stderr=PIPE)

        os.set_blocking(self.listen_command.stdout.fileno(), False)
        os.set_blocking(self.listen_command.stderr.fileno(), False)

        while True:
            stdout = self.listen_command.stdout.read()
            stderr = self.listen_command.stderr.read()

            if stdout:
                print(stdout.decode(), end="")
            if stderr:
                print(stderr.decode(), end="")

                if "infinite listen" in stderr.decode():
                    break

            sleep(0.1)

        print("Done setting up, sending now!")
        self.send_command = Popen(["just", "kernel", "send"], env=env)
        send_out = self.send_command.communicate()
        print("Done sending!")

        # os.system("ssh $RASPI_CUSTOM_KERNEL_HOST 'kill $(lsof -t /dev/uio0) || true &> /dev/null'")
        listen_out = self.listen_command.communicate()



TEST_PATTERNS = [string.ascii_letters, "A"]
BAUDRATES = [9600, 19200, 57600, 115200]
JUST_COMMANDS = [["just", "polling", "listen"], ["just", "irq", "dint-listen"], ["just", "irq", "scratch-listen"]]
REALTIME = [False, True]

TESTCASES = [
    TestCase(test_pattern=pattern, just_listen_command=command, baudrate=baud, realtime=rt)
    for pattern, baud, command, rt in itertools.product(TEST_PATTERNS, BAUDRATES, JUST_COMMANDS, REALTIME)
]
