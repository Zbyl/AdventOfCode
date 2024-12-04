import itertools
import logging
import argparse
import re


def main():
    logging.basicConfig(level=logging.INFO)

    parser = argparse.ArgumentParser(description='Advent of Code.')
    parser.add_argument('-i', '--input', default='input.txt', help='Input file.')
    args = parser.parse_args()

    with open(args.input, 'rt') as f:
        lines = f.readlines()

    count = 0
    enabled = True
    for rawline in lines:

        def cut_out_disabled(rawline: str) -> list[str]:
            nonlocal enabled
            enabled_start = 0
            pieces = []
            for match in re.finditer(r'do\(\)|don\'t\(\)', rawline):
                new_enabled = match[0] == 'do()'
                if enabled != new_enabled:
                    if enabled:
                        pieces.append(rawline[enabled_start:match.span()[0]])
                    else:
                        enabled_start = match.span()[1]
                    enabled = new_enabled

            if enabled:
                pieces.append(rawline[enabled_start:])

            return pieces

        def count_line(line):
            count = 0
            matches = re.findall(r'mul\((\d{1,3}),(\d{1,3})\)', line)
            for a, b in matches:
                count += int(a) * int(b)
            return count

        pieces = cut_out_disabled(rawline)
        for piece in pieces:
            count += count_line(piece)

    print(count)


if __name__ == '__main__':
 ()
