import sys
import tty
import termios
import select


def read_key():
    char = sys.stdin.read(1)
    if char == "\x1b":
        r, _, _ = select.select([sys.stdin], [], [], 0.1)
        if r:
            char += sys.stdin.read(2)
    return char


if sys.stdin.isatty():
    fd = sys.stdin.fileno()
    old = termios.tcgetattr(fd)
    try:
        tty.setcbreak(fd)
        # print("Press a key (q to quit)")
        # while True:
        #     k = read_key()
        #     print(repr(k))
        #     if k == 'q': break
    finally:
        termios.tcsetattr(fd, termios.TCSADRAIN, old)
else:
    print("Skipping: stdin is not a TTY", file=sys.stderr)
