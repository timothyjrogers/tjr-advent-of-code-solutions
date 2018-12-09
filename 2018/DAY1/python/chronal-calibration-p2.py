#!/usr/bin/env python

"""Solution to Advent of Code 2018 Day 1, Challenge 2: Chronal Calibration"""
__author__ = "Timothy J. Rogers"

def main():
    freq = 0
    freqs = set([freq])
    p2_answer = None
    with open('../chronal-calibration-input.txt', 'r') as f:
        freq_data = f.readlines()
    while p2_answer == None:
        for fr in freq_data:
            freq = freq + int(fr)
            if freq in freqs:
                p2_answer = freq
                break
            else:
                freqs.add(freq)
    print('Part 2 Answer: ' + str(p2_answer))


if __name__ == "__main__":
    main()