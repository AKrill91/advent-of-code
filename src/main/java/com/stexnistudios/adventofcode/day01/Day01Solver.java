package com.stexnistudios.adventofcode.day01;

import com.stexnistudios.adventofcode.Solver;

import java.util.ArrayList;
import java.util.List;

public abstract class Day01Solver extends Solver {

    protected final List<String> instructions;

    public Day01Solver(String input) {
        super(input);

        instructions = new ArrayList<>();
        String[] parts = getInput().split(",");
        for (String part : parts) {
            instructions.add(part.trim());
        }
    }
}
