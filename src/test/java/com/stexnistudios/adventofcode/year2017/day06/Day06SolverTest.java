package com.stexnistudios.adventofcode.year2017.day06;

import org.junit.Assert;
import org.junit.Test;

import static org.junit.Assert.*;

public class Day06SolverTest {

    @Test
    public void solveA() {
        String input = "0\t2\t7\t0";

        Integer expected = 5;

        Assert.assertEquals(expected, new Day06Solver(input).solveA());
    }

    @Test
    public void solveB() {
        String input = "0\t2\t7\t0";

        Integer expected = 4;

        Assert.assertEquals(expected, new Day06Solver(input).solveB());
    }
}