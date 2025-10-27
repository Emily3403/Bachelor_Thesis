import os
import re
import subprocess
from pathlib import Path


def load_env(path=".envrc"):
    # Written by Claude.AI

    env = os.environ.copy()
    env["PATH"] += ":/home/emily/bin"
    env["SSH_AUTH_SOCK"] = "/run/user/1000/ssh-agent.socket"

    with open(path, 'r') as f:
        for line in f:
            line = line.strip()

            # Skip comments and empty lines
            if not line or line.startswith('#'):
                continue

            # Parse export statements
            if line.startswith('export '):
                line = line[7:]  # Remove 'export '

                if '=' in line:
                    key, value = line.split('=', 1)
                    key = key.strip()
                    value = value.strip().strip('"').strip("'")

                    # Expand variables in the value
                    value = expand_variables(value, env)
                    env[key] = value
                    os.environ[key] = value

    return env


def expand_variables(value, env):
    # Written by Claude.AI

    """Expand ${VAR} and $VAR and $(command) patterns."""

    # Expand ${VAR} patterns
    def replace_braced(match):
        var_name = match.group(1)
        return env.get(var_name, '')

    value = re.sub(r'\$\{([^}]+)\}', replace_braced, value)

    # Expand $VAR patterns (word boundaries)
    def replace_simple(match):
        var_name = match.group(1)
        return env.get(var_name, '')

    value = re.sub(r'\$([A-Za-z_][A-Za-z0-9_]*)', replace_simple, value)

    # Expand $(command) patterns
    def replace_command(match):
        command = match.group(1)
        try:
            result = subprocess.run(
                command,
                shell=True,
                capture_output=True,
                text=True,
                env=env
            )
            return result.stdout.strip()
        except:
            return ''

    value = re.sub(r'\$\(([^)]+)\)', replace_command, value)

    return value

def check_ending(a: str, b: str):
    return a[-5:] == b[-5:]

def find_nonces(it: str) -> list[str]:
    return re.findall("\|(\d+)\n", it)
