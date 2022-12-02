import datetime
import os
import sys
import webbrowser


def run(command):
    print(">", command)
    if os.system(command) != 0:
        sys.exit(1)


def main():
    day = datetime.datetime.today().day
    run(f"cargo scaffold {day}")
    run(f"cargo download {day}")
    run("code .")
    webbrowser.open(f"https://adventofcode.com/2022/day/{day}")


if __name__ == "__main__":
    main()
