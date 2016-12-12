package com.stexnistudios.adventofcode.day12;

import org.junit.Assert;
import org.junit.Test;

public class Day12SolverTest {

    @Test
    public void testA() {
        String input = "cpy 41 a\n" +
            "inc a\n" +
            "inc a\n" +
            "dec a\n" +
            "jnz a 2\n" +
            "dec a";

        Day12aSolver solver = new Day12aSolver(input);

        Assert.assertEquals(42, solver.call().intValue());
    }
}