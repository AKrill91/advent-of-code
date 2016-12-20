package com.stexnistudios.adventofcode.day19;

import org.junit.Assert;
import org.junit.Test;

public class Day19bSolverTest {

    @Test
    public void test() throws Exception {
        Assert.assertEquals(2, new Day19bSolver("5").call().intValue());
        Assert.assertEquals(27, new Day19bSolver("54").call().intValue());
        Assert.assertEquals(27, new Day19bSolver("27").call().intValue());
        Assert.assertEquals(71, new Day19bSolver("76").call().intValue());
    }
}