package com.stexnistudios.adventofcode.year2017.day05;

import org.junit.Assert;
import org.junit.Before;
import org.junit.Test;

import static org.junit.Assert.*;

public class Day05SolverTest {

    private static final String TEST_INPUT = "0\n" +
        "3\n" +
        "0\n" +
        "1\n" +
        "-3";

    private Day05Solver solver;

    @Before
    public void beforeEach() {
        solver = new Day05Solver(TEST_INPUT);
    }

    @Test
    public void solveA() {
        Assert.assertEquals(
            5,
            solver.solveA()
        );
    }

    @Test
    public void solveB() {
        Assert.assertEquals(
            10,
            solver.solveB()
        );
    }
}