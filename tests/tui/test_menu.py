import sys
import tty
import termios


def ask_multi(prompt, options, defaults):
    if not sys.stdout.isatty():
        return defaults

    selected = list(defaults)
    current_idx = 0

    print(f"\033[1m{prompt}\033[0m")
    print(
        f"\033[2m  (Use UP/DOWN arrows to navigate, SPACE or 1..{len(options)} to toggle, ENTER to confirm)\033[0m"
    )

    for _ in options:
        print()

    def render():
        sys.stdout.write(f"\033[{len(options)}A")
        for i, opt in enumerate(options):
            marker = "\033[32m[x]\033[0m" if selected[i] else "\033[2m[ ]\033[0m"
            cursor = "\033[36m>\033[0m" if i == current_idx else " "
            sys.stdout.write(f"\033[2K\r  {cursor} {i + 1}. {marker} {opt}\n")
        sys.stdout.flush()

    render()

    fd = sys.stdin.fileno()
    old_settings = termios.tcgetattr(fd)
    try:
        tty.setcbreak(fd)
        while True:
            char = sys.stdin.read(1)
            if char == "\n" or char == "\r":
                break
            elif char == " ":
                selected[current_idx] = not selected[current_idx]
                render()
            elif char in "123456789":
                idx = int(char) - 1
                if 0 <= idx < len(options):
                    selected[idx] = not selected[idx]
                    current_idx = idx
                    render()
            elif char == "\x1b":
                seq = sys.stdin.read(2)
                if seq == "[A":
                    current_idx = max(0, current_idx - 1)
                    render()
                elif seq == "[B":
                    current_idx = min(len(options) - 1, current_idx + 1)
                    render()
            elif char == "\x03":
                raise KeyboardInterrupt
    finally:
        termios.tcsetattr(fd, termios.TCSADRAIN, old_settings)

    return selected


if __name__ == "__main__":
    pass
