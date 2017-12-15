package com.stexnistudios.adventofcode.year2017.day10;

import com.stexnistudios.adventofcode.Solver;

public class Day10Solver extends Solver {
    private final int length;

    public Day10Solver(String input) {
        this(input, 256);
    }

    public Day10Solver(String input, int length) {
        super(input);

        this.length = length;
    }

    @Override
    public Integer solveA() {
        String[] parts = getInput().split(",");

        char[] lens = new char[parts.length];

        for (int i = 0; i < parts.length; ++i) {
            lens[i] = (char) Integer.parseInt(parts[i]);
        }

        char[] elements = getStartingElements();

        int position = 0;
        int skipSize = 0;

        for (char len : lens) {
            if (len >= 2 && len <= length) {
                char[] orig = new char[len];

                for (char i = 0; i < len; ++i) {
                    orig[i] = elements[(position + i) % length];
                }

                for (char i = 0; i < len; ++i) {
                    elements[(position + i) % length] = orig[len - i - 1];
                }
            }

            position += (len + skipSize) % length;

            ++skipSize;
        }

        char first = elements[0];
        char second = elements[1];

        logger.info("first: {}, second: {}", (int) first, (int) second);

        return first * second;
    }

    @Override
    public String solveB() {
        final char[] extra = new char[]{
            17, 31, 73, 47, 23
        };
        int inputLength = getInput().length();
        int extraLength = extra.length;
        char[] input = getInput().toCharArray();
        char[] lens = new char[inputLength + extraLength];

        System.arraycopy(input, 0, lens, 0, inputLength);
        System.arraycopy(extra, 0, lens, inputLength, extraLength);

        char[] elements = getStartingElements();

        int position = 0;
        int skipSize = 0;

        for (int j = 0; j < 64; ++j) {
            for (char len : lens) {
                if (len >= 2 && len <= length) {
                    char[] orig = new char[len];

                    for (char i = 0; i < len; ++i) {
                        orig[i] = elements[(position + i) % length];
                    }

                    for (char i = 0; i < len; ++i) {
                        elements[(position + i) % length] = orig[len - i - 1];
                    }
                }

                position += (len + skipSize) % length;

                ++skipSize;
            }
        }

        char[] denseElements = new char[16];

        for (int i = 0; i < 16; ++i) {
            char num = 0;
            for (int j = 0; j < 16; ++j) {
                num = (char) (num ^ elements[i * 16 + j]);
            }

            denseElements[i] = num;
        }

        StringBuilder output = new StringBuilder();

        for (int i = 0; i < denseElements.length; ++i) {
            output.append(String.format("%02X", (int) denseElements[i]).toLowerCase());
        }

        return output.toString();
    }

    private char[] getStartingElements() {
        char[] output = new char[length];

        for (char i = 0; i < length; ++i) {
            output[i] = i;
        }

        return output;
    }
}
