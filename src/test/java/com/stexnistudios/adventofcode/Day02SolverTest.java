package com.stexnistudios.adventofcode;

import org.junit.Assert;
import org.junit.Test;

public class Day02SolverTest {
    @Test
    public void call() throws Exception {
        String input = "ULL\n" +
            "RRDDD\n" +
            "LURDL\n" +
            "UUUUD";

        Day02Solver solver = new Day02Solver(input);

        Assert.assertEquals(1985, solver.call().intValue());
    }

}