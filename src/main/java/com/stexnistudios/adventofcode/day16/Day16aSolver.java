package com.stexnistudios.adventofcode.day16;

import com.stexnistudios.adventofcode.Solver;

public class Day16aSolver extends Solver {

    private final int diskSize;

    public Day16aSolver(String input) {
        this(input, 272);
    }

    public Day16aSolver(String input, int diskSize) {
        super(input);

        this.diskSize = diskSize;
    }

    @Override
    public String call() {
        StringBuilder sb = new StringBuilder(diskSize * 2);

        sb.append(getInput());

        while (sb.length() < diskSize) {
            int initialLen = sb.length();

            sb.append('0');

            for (int i = 0; i < initialLen; ++i) {
                char c = sb.charAt(initialLen - i - 1);
                sb.append(c == '1' ? '0' : '1');
            }
        }

        String checksum = sb.substring(0, diskSize);

        do {
            checksum = getChecksum(checksum);
        } while (checksum.length() % 2 == 0);

        return checksum;
    }

    private String getChecksum(String input) {
        int numPairs = input.length() / 2;
        StringBuilder output = new StringBuilder(numPairs);

        for (int i = 0; i < numPairs; ++i) {
            char first = input.charAt(i * 2);
            char second = input.charAt(i * 2 + 1);

            output.append(first == second ? '1' : '0');
        }

        return output.toString();
    }
}
