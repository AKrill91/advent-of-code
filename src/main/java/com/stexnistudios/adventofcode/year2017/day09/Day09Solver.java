package com.stexnistudios.adventofcode.year2017.day09;

import com.stexnistudios.adventofcode.Solver;

public class Day09Solver extends Solver {

    private static final char GROUP_START = '{';
    private static final char GROUP_END = '}';
    private static final char GARBAGE_START = '<';
    private static final char GARBAGE_END = '>';
    private static final char CANCEL_CHAR = '!';

    public Day09Solver(String input) {
        super(input);
    }

    @Override
    public Integer solveA() {
        int total = 0;

        int score = 0;
        char[] chars = getInput().toCharArray();
        boolean cancelActive = false;
        boolean inGarbage = false;
        for (char c : chars) {
            if (cancelActive) {
                cancelActive = false;
            } else {
                if (c == CANCEL_CHAR) {
                    cancelActive = true;
                    continue;
                }
                if (inGarbage) {
                    if (c == GARBAGE_END) {
                        inGarbage = false;
                    }
                } else {
                    switch (c) {
                        case GARBAGE_START:
                            inGarbage = true;
                            break;
                        case GROUP_START:
                            score += 1;
                            break;
                        case GROUP_END:
                            total += score;
                            score -= 1;
                            break;
                    }
                }
            }
        }

        return total;
    }

    @Override
    public Object solveB() {
        int total = 0;

        char[] chars = getInput().toCharArray();
        boolean cancelActive = false;
        boolean inGarbage = false;
        for (char c : chars) {
            if (cancelActive) {
                cancelActive = false;
            } else {
                if (c == CANCEL_CHAR) {
                    cancelActive = true;
                    continue;
                }
                if (inGarbage) {
                    if (c == GARBAGE_END) {
                        inGarbage = false;
                    } else {
                        total++;
                    }
                } else {
                    switch (c) {
                        case GARBAGE_START:
                            inGarbage = true;
                            break;
                    }
                }
            }
        }

        return total;
    }
}
