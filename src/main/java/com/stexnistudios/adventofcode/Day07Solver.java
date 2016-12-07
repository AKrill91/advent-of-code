package com.stexnistudios.adventofcode;

import java.util.Arrays;
import java.util.List;

public abstract class Day07Solver extends Solver {
    public Day07Solver(String input) {
        super(input);
    }

    @Override
    public Integer call() {
        List<String> inputs = Arrays.asList(getInput().split("\n"));
        int validCount = 0;

        for (String input : inputs) {
            if (isInputValid(input)) {
                ++validCount;
            }
        }

        return validCount;
    }

    protected abstract boolean isInputValid(String input);
}
