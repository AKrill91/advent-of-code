package com.stexnistudios.adventofcode;

import com.stexnistudios.adventofcode.day08.Day08aSolver;
import org.junit.Assert;
import org.junit.Test;

public class Day08SolverTest {
    @Test
    public void testA() throws Exception {
        String input = "rect 3x2\n" +
            "rotate column x=1 by 1\n" +
            "rotate row y=0 by 4\n" +
            "rotate column x=1 by 1";

        Day08aSolver solver = new Day08aSolver(input, 7, 3);

        Assert.assertEquals(6, solver.call().intValue());
    }
}