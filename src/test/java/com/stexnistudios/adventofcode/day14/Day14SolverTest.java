package com.stexnistudios.adventofcode.day14;

import org.junit.Assert;
import org.junit.Ignore;
import org.junit.Test;

public class Day14SolverTest {
    @Test
    @Ignore("Takes ~1.5s")
    public void testA() throws Exception {
        String input = "abc";
        Assert.assertEquals(22728, new Day14aSolver(input).call().intValue());
    }

    @Test
    @Ignore("Ignoring since it takes forever")
    public void testB() throws Exception {
        String input = "abc";
        Assert.assertEquals(22551, new Day14bSolver(input).call().intValue());
    }
}