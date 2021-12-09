#!/usr/bin/env python3

# Note: Some of this code is based on this sudoku example: https://www.cs.toronto.edu/~victorn/tutorials/sat20/sudoku.py

from z3 import *
from typing import List, Dict
from itertools import combinations

possible_vars = ['a', 'b', 'c', 'd', 'e', 'f', 'g']

VALUE_TO_SEGMENTS = [
    ['a', 'b', 'c', 'e', 'f', 'g'],
    ['c', 'f'],
    ['a', 'c', 'd', 'e', 'g'],
    ['a', 'c', 'd', 'f', 'g'],
    ['b', 'c', 'd', 'f'],
    ['a', 'b', 'd', 'f', 'g'],
    ['a', 'b', 'd', 'e', 'f', 'g'],
    ['a', 'c', 'f'],
    ['a', 'b', 'c', 'd', 'e', 'f', 'g'],
    ['a', 'b', 'c', 'd', 'f', 'g'],
]

LEN_TO_VALUE = {
    2: 1,
    3: 7,
    4: 4,
    7: 8,
}

def digit_from_segments(segments: List[str]) -> int:
    a = LEN_TO_VALUE.get(len(segments))
    if a:
        return a
    for i, segs in enumerate(VALUE_TO_SEGMENTS):
        found = True
        for s in segs:
            if not s in segments:
                found = False
        if found:
            return i

def exactly_one(s, literals):
    clauses = [literals] # at least one of the literals is true
    # Now encode no more than one literal is true.
    # Hint: there is no pair of literals such that both are true.
    for comb in combinations(literals, 2):
        clauses += [[Not(comb[0]), Not(comb[1])]]
    for c in clauses:
        s.add(Or(c))

def idx(c: str) -> int:
    return ord(c) - ord('a')

def dsp(i: int) -> str:
    return chr(ord('a') + i)

def format_solutions(model, lits) -> Dict[str, str]:
    m = {}
    for var in possible_vars:
        print(f'{var} -> ', end='')
        for i, v in enumerate(lits[var]):
            if model.evaluate(v):
                print(f'{dsp(i)}')
                m[dsp(i)] = var
    return m

def remap(mappings: Dict[str, str], input: List[str]) -> List[str]:
    return list(map(lambda s: mappings[s], input))

def solve_line(input_values: List[str], target_digits: List[str]):
    s = Solver()

    mappings = {}
    for input_signal in possible_vars:
        a = [Bool(f'c_{input_signal}_{output_signal}') for output_signal in possible_vars]
        mappings[input_signal] = a
        exactly_one(s, a)
    
    for i in range(len(mappings['a'])):
        values = []
        for v in mappings.values():
            values.append(v[i])
        exactly_one(s, values)
    
    for v in input_values:
        l = len(v)
        if l == 2 or l == 3 or l == 4 or l == 7:
            value = LEN_TO_VALUE[l]
            target_chars = VALUE_TO_SEGMENTS[value]
            for c in target_chars:
                statement = None
                # for a in v:
                #     if statement:
                #         statement = Or(statement, mappings[idx(c)][idx(a)] == True)
                #     else:
                #         statement = mappings[c][idx(a)] == True
                statement = Or(list(map(lambda a: mappings[c][idx(a)] == True, v)))
                s.add(statement)

    if str(s.check()) != 'sat':
        print(f'Cannot solve line')
    else:
        mappings = format_solutions(s.model(), mappings)
        for digit in target_digits:
            segments = remap(mappings, list(digit))
            print(digit_from_segments(segments), end='')
        print()

with open('test_input.txt') as f:
    lines = f.readlines()

for line in lines:
    pts = line.strip().split(' | ')
    solve_line(pts[0].split(' '), pts[1].split(' '))
