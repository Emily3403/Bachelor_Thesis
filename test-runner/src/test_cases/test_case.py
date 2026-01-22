import os
from dataclasses import dataclass
from subprocess import Popen, PIPE
from time import sleep
from typing import Any

from src.settings import CACHE_RESULTS
from src.test_cases.dir_config import DIR_CONFIG


@dataclass
class TestCase:
    """
    An Instance of a TestCase, having all the information needed for executing it
    """
    iteration: int
    test_name: str
    test_pattern: str

    baudrate: int
    realtime: bool
    packet_num_data_bytes: int

    just_command: str
    listen_command: None | Popen = None
    send_command: None | Popen = None

    def setup(self) -> None:
        os.makedirs(DIR_CONFIG.localhost_stdout(self), exist_ok=True)
        self.generate_stdin_file()

    def run(self, env: dict[Any, Any]) -> None:
        if CACHE_RESULTS and DIR_CONFIG.localhost_data_file(self).exists():
            print(f"Skipped TestCase {self}")
            return

        self.setup()
        self.spawn_listen_command(env)
        self.send_string(env)
        self.kill_listen()
        self.get_results(env)

    def spawn_listen_command(self, env: dict[Any, Any]) -> None:
        self.listen_command = self.popen_just(env, self.just_command)

        if self.listen_command.stderr is None:
            return None

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

    def send_string(self, env: dict[Any, Any]) -> None:
        print("Sending...")
        self.send_command = self.popen_just(env, "kernel-send")
        ret = self.send_command.wait()
        if ret != 0:
            print(f"ERROR: Sending {self}")

        print("Done!")

    def kill_listen(self) -> None:
        if self.listen_command:
            os.system("ssh $RASPI_CUSTOM_KERNEL_HOST 'kill $(lsof -t /dev/uio0) || true &> /dev/null'")

    def get_results(self, env: dict[Any, Any]) -> bool:
        return Popen(["rsync", "-a", f"$RASPI_CUSTOM_KERNEL_HOST:{DIR_CONFIG.raspi_stdout(self)}/", f"{DIR_CONFIG.localhost_stdout(self)}/"], env=env).wait() == 0

    # === Utils ===
    def popen_just(self, env: dict[Any, Any], command: str) -> Popen:
        it = Popen([
            "just",
            f"baudrate={self.baudrate}",
            f"packet-num-data-bytes={self.packet_num_data_bytes}",

            f"save-dir={DIR_CONFIG.raspi_base(self)}",
            f"send-file={DIR_CONFIG.raspi_stdin(self)}",
            command
        ], env=env, stdout=PIPE, stderr=PIPE)

        if it.stderr is not None:
            os.set_blocking(it.stderr.fileno(), False)

        if it.stdout is not None:
            os.set_blocking(it.stdout.fileno(), False)

        return it

    def generate_stdin_file(self) -> None:
        """Generates a file with a certain length, depending on the baudrate"""
        path = DIR_CONFIG.localhost_stdin(self)
        if path.exists():
            return

        wanted_len = int(0.5 * self.baudrate)

        num_reps, num_chars_left = divmod(wanted_len, len(self.test_pattern) + 5)
        stdin = "".join(self.test_pattern for _ in range(num_reps)) + self.test_pattern[:num_chars_left]

        path.write_text(stdin)
