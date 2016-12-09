package com.stexnistudios.adventofcode.day03;

import com.stexnistudios.adventofcode.Solver;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collections;
import java.util.List;

public class Day03aSolver extends Solver {

    public Day03aSolver(String input) {
        super(input);
    }

    @Override
    public Integer call() throws Exception {
        String[] lines = getInput().split("\r\n");

        int validCount = 0;

        for (String line : lines) {
            String[] sides = line.trim().split("\\s+");
            int first = Integer.parseInt(sides[0]);
            int second = Integer.parseInt(sides[1]);
            int third = Integer.parseInt(sides[2]);

            List<Integer> ints = new ArrayList<>(Arrays.asList(first, second, third));
            Collections.sort(ints);

            if(ints.get(0) + ints.get(1) > ints.get(2)) {
                ++validCount;
            }
        }

        return validCount;
    }
}
