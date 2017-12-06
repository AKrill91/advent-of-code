package com.stexnistudios.adventofcode.year2017.day05;

import com.stexnistudios.adventofcode.Solver;

import java.util.ArrayList;
import java.util.List;

public class Day05Solver extends Solver {
    public Day05Solver(String input) {
        super(input);
    }

    @Override
    public Object solveA() {
        List<Integer> steps = getSteps();
        int step = 0;
        int index = 0;

        while (index < steps.size()) {
            int stride = steps.get(index);
            int next = index + stride;
            steps.set(index, stride + 1);
            index = next;
            ++step;
        }

        return step;
    }

    @Override
    public Object solveB() {
        List<Integer> steps = getSteps();
        int numJumps = 0;
        int index = 0;

        while (index >= 0 && index < steps.size()) {
            ++numJumps;

            int offset = steps.get(index);
            int diff = offset >= 3 ? -1 : 1;

            steps.set(index, offset + diff);
            index += offset;
        }

        return numJumps;
    }

    private List<Integer> getSteps() {
        List<Integer> steps = new ArrayList<>();

        for (String step : getInput().split("\n")) {
            steps.add(Integer.parseInt(step));
        }

        return steps;
    }
}
