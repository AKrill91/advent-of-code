package com.stexnistudios.adventofcode.day18;

import org.junit.Assert;
import org.junit.Test;

public class Day18SolverTest {

    @Test
    public void testA() throws Exception {
        String input = "..^^.";

        Assert.assertEquals(6, new Day18aSolver(input, 3).call().intValue());
        input = ".^^.^.^^^^";
        Assert.assertEquals(38, new Day18aSolver(input, 10).call().intValue());
    }
}