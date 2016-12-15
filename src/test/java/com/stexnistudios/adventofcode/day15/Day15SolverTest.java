package com.stexnistudios.adventofcode.day15;

import org.junit.Assert;
import org.junit.Test;

public class Day15SolverTest {
    @Test
    public void testA() {
        String input = "Disc #1 has 5 positions; at time=0, it is at position 4.\n" +
            "Disc #2 has 2 positions; at time=0, it is at position 1.";

        Assert.assertEquals(5, new Day15aSolver(input).call().intValue());
    }
}