import os
import random
from dataclasses import dataclass, field
from math import log
from pathlib import Path
from subprocess import Popen, PIPE
from time import sleep
from typing import Any

from settings import app_path, delimit


@dataclass
class TestCase:
    """
    An Instance of a TestCase, having all the information needed for executing it
    """
    test_name: str
    test_pattern: str
    iteration: int

    baudrate: int
    realtime: bool
    packet_num_data_bytes: int

    just_command: str
    listen_command: None | Popen = field(default=None)

    def run(self, env: dict[Any, Any]) -> None:
        if self.stdout_path().exists():
            print("SKIPPED")
            return

        self.setup(env)

        self.listen_command = Popen(["just", f"baudrate={self.baudrate}", f"save-dir={self.raspi_save_dir()}", f"packet-num-data-bytes={self.packet_num_data_bytes}", self.just_command], env=env, stdout=PIPE, stderr=PIPE)
        os.set_blocking(self.listen_command.stderr.fileno(), False)

        output = []

        while True:
            stderr = self.listen_command.stderr.read()

            if stderr is not None:
                output.append(stderr.decode())
                if "infinite listen" in stderr.decode():
                    break

            sleep(0.1)
            ret = self.listen_command.poll()
            if ret is not None:
                print(f"ERROR: listen has terminated: {ret} {self!r}\n")
                print("\n".join(output))
                return

        self.send_string(env)
        self.kill_listen()

        listen_out = self.listen_command.communicate()

        self.get_results(env)

    def send_string(self, env: dict[Any, Any]) -> None:
        print("Sending...")
        ret = Popen(["just", f"baudrate={self.baudrate}", f"send-file=./test-cases/{self.test_name}/{self.baudrate}/stdin", "kernel-send"], env=env).wait()
        if ret != 0:
            print("ERROR: Send command")
            return

        print("Done!")

    def kill_listen(self) -> None:
        if self.listen_command:
            os.system("ssh $RASPI_CUSTOM_KERNEL_HOST 'kill $(lsof -t /dev/uio0) || true &> /dev/null'")

    def setup(self, env: dict[Any, Any]):
        os.makedirs(self.test_case_path(), exist_ok=True)
        self.generate_stdin_file()

    def raspi_save_dir(self) -> Path:
        return Path("./test-cases/") / self.test_name / str(self.baudrate) / self.just_command


    def test_dir_path(self) -> Path:
        return app_path / "test-cases" / str(self.iteration) / self.test_name / str(self.baudrate)

    def test_case_path(self) -> Path:
        return self.test_dir_path() / self.just_command

    def stdin_path(self) -> Path:
        return self.test_dir_path() / "stdin"

    def stdout_path(self) -> Path:
        return self.test_case_path() / "stdout"

    def generate_stdin_file(self) -> None:
        """Generates a file with a certain length, depending on the baudrate"""
        path = self.stdin_path()
        if path.exists():
            return

        wanted_len = int(0.5 * self.baudrate)

        num_reps, num_chars_left = divmod(wanted_len, len(self.test_pattern) + 5)
        patterns = [[self.test_pattern, chr(i % 65535), "\n"] for i in range(num_reps)]
        stdin = "".join(it for row in patterns for it in row)

        path.write_text(stdin)

    def get_results(self, env: dict[Any, Any]) -> int:
        return Popen(["rsync", "-a", f"b1:{self.raspi_save_dir()}/", f"{self.test_case_path()}/"], env=env).wait()
