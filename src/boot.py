import sys
import ctypes


def windows_enable_ANSI() -> None:
    """Configures the Windows cmd.exe to initialize ANSI escape codes before
    running the TUI."""
    kernel32 = ctypes.windll.kernel32
    stdout_handle = kernel32.GetStdHandle(-11)
    kernel32.SetConsoleMode(stdout_handle, 7)


def configure_virtual_terminal() -> None:
    if sys.platform.startswith("win32"):
        windows_enable_ANSI()
