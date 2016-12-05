package com.stexnistudios.adventofcode;

import org.junit.Assert;
import org.junit.Test;

public class Day05SolverTest {
    @Test
    public void testA() throws Exception {
        Day05aSolver solver = new Day05aSolver("abc");
        Assert.assertEquals("18f47a30", solver.call());
    }
}