package com.stexnistudios.adventofcode.day20;

import org.junit.Assert;
import org.junit.Test;

public class Day20SolverTest {
    @Test
    public void testA() throws Exception {
        String input = "5-8\n" +
            "0-2\n" +
            "4-7";

        Assert.assertEquals(3L, new Day20aSolver(input).call().longValue());

        input = "5-8\n" +
            "0-3\n" +
            "4-7";

        Assert.assertEquals(9L, new Day20aSolver(input).call().longValue());
    }

    @Test
    public void testB() throws Exception {
        String input = "5-8\n" +
            "0-2\n" +
            "4-7";

        Assert.assertEquals(2L, new Day20bSolver(input, 0, 9).call().longValue());
    }
}