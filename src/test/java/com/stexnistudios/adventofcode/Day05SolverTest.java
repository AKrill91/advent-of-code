package com.stexnistudios.adventofcode;

import com.stexnistudios.adventofcode.day05.Day05aSolver;
import com.stexnistudios.adventofcode.day05.Day05bSolver;
import org.junit.Assert;
import org.junit.Ignore;
import org.junit.Test;

public class Day05SolverTest {
    @Test
    @Ignore("Takes a while")
    public void testA() throws Exception {
        Day05aSolver solver = new Day05aSolver("abc");
        Assert.assertEquals("18f47a30", solver.call());
    }

    @Test
    @Ignore("Takes a while")
    public void testB() throws Exception {
        Day05bSolver solver = new Day05bSolver("abc");
        Assert.assertEquals("05ace8e3", solver.call());
    }
}