from subprocess import Popen
from typing import Any, TYPE_CHECKING

from src.test_cases.dir_config import DIR_CONFIG

if TYPE_CHECKING:
    from src.test_cases.test_case import TestCase


def raspi_push_fs(env: dict[Any, Any]) -> bool:
    p1 = Popen(["rsync", "-aL", "--mkpath", f"{DIR_CONFIG._base()}/", f"{env['RASPI_CUSTOM_KERNEL_HOST']}:{DIR_CONFIG._raspi_base()}/", ], env=env)
    p2 = Popen(["rsync", "-aL", "--mkpath", f"{DIR_CONFIG._base()}/", f"{env['RASPI_STOCK_KERNEL_HOST']}:{DIR_CONFIG._raspi_base()}/", ], env=env)

    return p1.wait() == p2.wait() == 0


def get_results(test: TestCase, env: dict[Any, Any]) -> bool:
    # TODO: This function should return the results as an object instead of only modifying the filesystem
    return Popen(["rsync", "-a", f"{env['RASPI_CUSTOM_KERNEL_HOST']}:{DIR_CONFIG.raspi_output(test)}/", f"{DIR_CONFIG.localhost_output(test)}/"], env=env).wait() == 0
