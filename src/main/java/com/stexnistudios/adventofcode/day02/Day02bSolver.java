package com.stexnistudios.adventofcode.day02;

import com.stexnistudios.adventofcode.util.Point;

import java.util.Arrays;

public class Day02bSolver extends Day02Solver {

    public Day02bSolver(String input) {
        super(
            input,
            Arrays.asList(
                Arrays.asList(null, null, "1", null, null),
                Arrays.asList(null, "2", "3", "4", null),
                Arrays.asList("5", "6", "7", "8", "9"),
                Arrays.asList(null, "A", "B", "C", null),
                Arrays.asList(null, null, "D", null, null)
            ),
            new Point(0, 2)
        );
    }
}
