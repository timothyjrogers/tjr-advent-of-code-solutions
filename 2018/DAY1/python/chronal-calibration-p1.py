#!/usr/bin/env python

"""Solution to Advent of Code 2018 Day 1, Challenge 1: Chronal Calibration"""
__author__ = "Timothy J. Rogers"

def main():
    freq = 0
    with open('../chronal-calibration-input.txt', 'r') as f:
        freq_data = f.readlines()
    for fr in freq_data:
        freq = freq + int(fr)
    print('Part 1 Answer: ' + str(freq))


if __name__ == "__main__":
    main()