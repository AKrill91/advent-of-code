package com.stexnistudios.adventofcode.day19;

import com.stexnistudios.adventofcode.Solver;

public class Day19bSolver extends Solver {
    private final int numElves;

    public Day19bSolver(String input) {
        super(input);

        numElves = Integer.parseInt(input);
    }

    @Override
    public Integer call() throws Exception {
        int powThree = findNearestPowThree();
        int power = (int) Math.pow(3, powThree);
        int remainder = numElves - power;

        if (remainder > power) {
            remainder = 2 * remainder - power;
        }

        if (remainder == 0) {
            remainder = power;
        }

        return remainder;
    }

    private int findNearestPowThree() {
        int power = 1;
        long sum = 3;

        while (sum <= numElves) {
            sum *= 3;
            ++power;
        }

        return power - 1;
    }
}
