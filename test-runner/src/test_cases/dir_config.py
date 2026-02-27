from dataclasses import dataclass
from pathlib import Path
from typing import TYPE_CHECKING

from src.settings import WORKING_DIR


if TYPE_CHECKING:
    from src.test_cases.test_case import TestCase


@dataclass
class DirConfig:
    """
    All methods without a `file` suffix are return dir Path's
    """

    _main_data_dir = Path("results")

    _data_in = Path("stdin")
    _data_out = Path("stdout")
    _packet_out = Path("packets.log")

    def _base(self) -> Path:
        from src.test_cases.parameters import ITERATION

        """Absolute Path on the Local System"""
        return WORKING_DIR / self._main_data_dir / str(ITERATION)

    def localhost_base(self, test: TestCase) -> Path:
        """Absolute Path on the Local System"""
        return self._base() / test.test_name / str(test.baudrate)

    def localhost_stdin_file(self, test: TestCase) -> Path:
        return self.localhost_base(test) / self._data_in

    def localhost_stdin_file_link(self, test: TestCase) -> Path:
        return self.localhost_base(test) / test.just_command / self._data_in

    def localhost_output(self, test: TestCase) -> Path:
        """Where all the output of *this* TestCase resides"""
        return self.localhost_base(test) / test.just_command

    def localhost_stdout_file(self, test: TestCase) -> Path:
        return self.localhost_output(test) / self._data_out

    def localhost_packet_file(self, test: TestCase) -> Path:
        return self.localhost_output(test) / self._packet_out

    # === Raspi ===
    def _raspi_base(self) -> Path:
        return Path(self._main_data_dir)

    def raspi_base(self, test: TestCase) -> Path:
        # TODO: Do I want to overwrite the test data every time or create the "iteration" dir also on the Raspis?
        return self._raspi_base() / test.test_name / str(test.baudrate)

    def raspi_stdin(self, test: TestCase) -> Path:
        return self.raspi_base(test) / self._data_in

    def raspi_output(self, test: TestCase) -> Path:
        """Where all the output of *this* TestCase resides"""
        return self.raspi_base(test) / test.just_command

    def raspi_stdout(self, test: TestCase) -> Path:
        return self.raspi_output(test) / self._data_out


DIR_CONFIG = DirConfig()
