import os
from dataclasses import dataclass
from logging import error, info
from subprocess import Popen, PIPE
from time import sleep
from typing import Any

from src.settings import CACHE_RESULTS, WORKING_DIR
from src.test_cases.dir_config import DIR_CONFIG


@dataclass
class TestCase:
    """
    An Instance of a TestCase, having all the information needed for executing it
    """
    iteration: int
    number: int  # Number in the global list of TESTCASES
    test_name: str
    test_pattern: str

    baudrate: int
    realtime: bool
    packet_num_data_bytes: int

    just_command: str
    listen_command: None | Popen = None
    send_command: None | Popen = None
    compile_command: None | Popen = None

    _setup = False

    def generate_test_data(self, env: dict[Any, Any]) -> bool:
        os.makedirs(DIR_CONFIG.localhost_output(self), exist_ok=True)
        self.generate_stdin_file()
        self._setup = True

        return True

    def try_compile(self, env: dict[Any, Any]) -> bool:
        self.compile_command =  self.popen_just(env, self.just_command, only_compile=True)
        if self.compile_command.wait():
            print(self.compile_command.stderr.read())
            return False

        return True

    def run(self, env: dict[Any, Any]) -> bool:
        if not self._setup:
            error("Data is not set up yet!")
            return False

        if CACHE_RESULTS and DIR_CONFIG.localhost_stdout_file(self).exists():
            info(f"Skipped TestCase {self}")
            return True

        if not self.spawn_and_wait_listen_command(env):
            error("Could not spawn listen command!")
            return False

        if not self.send_string(env):
            error("Could not `just kernel-send`")
            return False

        self.kill_listen_command()
        return True

    def spawn_and_wait_listen_command(self, env: dict[Any, Any]) -> bool:
        # TODO: This implementation of waiting for listen is suboptimal
        self.listen_command = self.popen_just(env, self.just_command)

        output = []
        while True:
            stderr = self.listen_command.stderr.read()

            if stderr is not None:
                it = stderr.decode()
                output.append(it)
                if "infinite listen" in it:
                    break

            sleep(0.1)
            ret = self.listen_command.poll()
            if ret is not None:
                error(f"listen has terminated: {ret} {self!r}\n" + "\n".join(output))
                return False

        return True

    def send_string(self, env: dict[Any, Any]) -> bool:
        print("Sending...")
        self.send_command = self.popen_just(env, "kernel-send")
        ret = self.send_command.wait()
        if ret != 0:
            error(f"Sending String exited with {ret}: {self!r}\n" + self.send_command.stderr.read().decode())

        print("Done!")

    def kill_listen_command(self) -> None:
        if self.listen_command:
            os.system("ssh $RASPI_CUSTOM_KERNEL_HOST 'kill $(lsof -t /dev/uio0) || true &> /dev/null'")

    # === Utils ===
    def popen_just(self, env: dict[Any, Any], command: str, only_compile: bool = False) -> Popen:
        it = Popen([
            "just",
            f"baudrate={self.baudrate}",
            f"num-data-bytes={self.packet_num_data_bytes}",
            f"only-compile={only_compile}",

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
        path = DIR_CONFIG.localhost_stdin_file(self)
        if path.exists():
            return

        wanted_len = int(0.5 * self.baudrate)

        num_reps, num_chars_left = divmod(wanted_len, len(self.test_pattern) + 5)
        stdin = "".join(self.test_pattern for _ in range(num_reps)) + self.test_pattern[:num_chars_left]

        path.write_text(stdin)
