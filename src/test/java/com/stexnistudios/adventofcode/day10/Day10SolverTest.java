package com.stexnistudios.adventofcode.day10;

import org.junit.Assert;
import org.junit.Test;

public class Day10SolverTest {
    @Test
    public void testA() {
        String input = "value 5 goes to bot 2\n" +
            "bot 2 gives low to bot 1 and high to bot 0\n" +
            "value 3 goes to bot 1\n" +
            "bot 1 gives low to output 1 and high to bot 0\n" +
            "bot 0 gives low to output 2 and high to output 0\n" +
            "value 2 goes to bot 2";

        Day10aSolver solver = new Day10aSolver(input, 2, 5);

        Assert.assertEquals(2, solver.call().intValue());
    }
}