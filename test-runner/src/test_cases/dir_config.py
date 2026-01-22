from dataclasses import dataclass
from pathlib import Path

from src.settings import WORKING_DIR
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

    def localhost_base(self, test: TestCase) -> Path:
        """Absolute Path on the Local System"""
        return WORKING_DIR / self._main_data_dir / str(test.iteration) / test.test_name / str(test.baudrate)

    def localhost_stdin(self, test: TestCase) -> Path:
        return self.localhost_base(test) / self._data_in

    # Where all the output of *this* TestCase resides
    def localhost_stdout(self, test: TestCase) -> Path:
        return self.localhost_base(test) / test.just_command

    def localhost_data_file(self, test: TestCase) -> Path:
        return self.localhost_stdout(test) / self._data_out

    def localhost_packet_file(self, test: TestCase) -> Path:
        return self.localhost_stdout(test) / self._packet_out

    # === Raspi ===
    def raspi_base(self, test: TestCase) -> Path:
        # TODO: Do I want to overwrite the test data every time or create the "iteration" dir also on the Raspis?
        return Path(self._main_data_dir) / str(test.iteration) / test.test_name / str(test.baudrate)

    def raspi_stdin(self, test: TestCase) -> Path:
        return self.raspi_base(test) / self._data_in

    def raspi_stdout(self, test: TestCase) -> Path:
        return self.raspi_base(test) / test.just_command


DIR_CONFIG = DirConfig()
