# Advent of Code - Day 1

## Puzzle Description

The safe has a dial with only an arrow on it; around the dial are the numbers 0 through 99 in order. As you turn the dial, it makes a small click noise as it reaches each number.

The attached document (your puzzle input) contains a sequence of rotations, one per line, which tell you how to open the safe. A rotation starts with an L or R which indicates whether the rotation should be to the left (toward lower numbers) or to the right (toward higher numbers). Then, the rotation has a distance value which indicates how many clicks the dial should be rotated in that direction.

So, if the dial were pointing at 11, a rotation of R8 would cause the dial to point at 19. After that, a rotation of L19 would cause it to point at 0.

Because the dial is a circle, turning the dial left from 0 one click makes it point at 99. Similarly, turning the dial right from 99 one click makes it point at 0.

So, if the dial were pointing at 5, a rotation of L10 would cause it to point at 95. After that, a rotation of R5 could cause it to point at 0.

The dial starts by pointing at 50.

You could follow the instructions, but your recent required official North Pole secret entrance security training seminar taught you that the safe is actually a decoy. The actual password is the number of times the dial is left pointing at 0 after any rotation in the sequence.

## User's Plan

1. Build a parser to parse the input text I have
2. Have a data structure that allows me to search cyclically through 0 and 99 without overflowing to negative numbers or over a 100
3. Keep track of each time the dial goes to 0

## User wants a guided tutorial

The user wants to be guided through syntax examples for each piece of the code, not with the actual result.

## Parsing with `match` vs. `unwrap`

The user asked if `match` could be used instead of `unwrap` for parsing, which led to a discussion on `Result` type and explicit error handling in Rust. Explained `match` and also mentioned `expect` and `if let` as alternatives.
