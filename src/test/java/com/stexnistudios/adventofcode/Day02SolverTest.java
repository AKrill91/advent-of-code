package com.stexnistudios.adventofcode;

import com.stexnistudios.adventofcode.day02.Day02aSolver;
import com.stexnistudios.adventofcode.day02.Day02bSolver;
import org.junit.Assert;
import org.junit.Test;

public class Day02SolverTest {
    @Test
    public void testA() throws Exception {
        String input = "ULL\n" +
            "RRDDD\n" +
            "LURDL\n" +
            "UUUUD";

        Day02aSolver solver = new Day02aSolver(input);

        Assert.assertEquals("1985", solver.call());
    }

    @Test
    public void testB() throws Exception {
        String input ="ULL\n" +
            "RRDDD\n" +
            "LURDL\n" +
            "UUUUD";

        Day02bSolver solver = new Day02bSolver(input);

        Assert.assertEquals("5DB3", solver.call());
    }

}