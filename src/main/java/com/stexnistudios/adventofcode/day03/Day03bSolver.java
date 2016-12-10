package com.stexnistudios.adventofcode.day03;

import com.stexnistudios.adventofcode.Solver;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collections;
import java.util.List;

public class Day03bSolver extends Solver {

    public Day03bSolver(String input) {
        super(input);
    }

    @Override
    public Integer call() throws Exception {
        String[] lines = getInput().split("\r\n");
        List<List<Integer>> numbers = new ArrayList<>();

        int validCount = 0;

        for (String line : lines) {
            String[] sides = line.trim().split("\\s+");
            int first = Integer.parseInt(sides[0]);
            int second = Integer.parseInt(sides[1]);
            int third = Integer.parseInt(sides[2]);
            numbers.add(Arrays.asList(first, second, third));
        }

        int numTrianglesInColumn = numbers.size() / 3;

        for (int i = 0; i < numTrianglesInColumn; ++i) {
            for (int j = 0; j < numbers.get(0).size(); ++j) {
                int first = numbers.get(i * 3).get(j);
                int second = numbers.get(i * 3 + 1).get(j);
                int third = numbers.get(i * 3 + 2).get(j);

                List<Integer> ints = new ArrayList<>(Arrays.asList(first, second, third));
                Collections.sort(ints);

                if (ints.get(0) + ints.get(1) > ints.get(2)) {
                    ++validCount;
                }
            }
        }

        return validCount;
    }
}
