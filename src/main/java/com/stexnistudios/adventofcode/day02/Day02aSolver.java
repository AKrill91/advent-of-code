package com.stexnistudios.adventofcode.day02;

import com.stexnistudios.adventofcode.util.Point;

import java.util.Arrays;

public class Day02aSolver extends Day02Solver {

    public Day02aSolver(String input) {
        super(
            input,
            Arrays.asList(
                Arrays.asList("1", "2", "3"),
                Arrays.asList("4", "5", "6"),
                Arrays.asList("7", "8", "9")
            ),
            new Point(1, 1)
        );
    }
}
