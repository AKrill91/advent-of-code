package com.stexnistudios.adventofcode.year2017.day08;

import org.junit.Assert;
import org.junit.Test;

public class Day08SolverTest {

    @Test
    public void solveA() {
        String input = "b inc 5 if a > 1\n" +
            "a inc 1 if b < 5\n" +
            "c dec -10 if a >= 1\n" +
            "c inc -20 if c == 10";

        Assert.assertEquals(
            1,
            (int) new Day08Solver(input).solveA()
        );
    }

    @Test
    public void solveB() {
        String input = "b inc 5 if a > 1\n" +
            "a inc 1 if b < 5\n" +
            "c dec -10 if a >= 1\n" +
            "c inc -20 if c == 10";

        Assert.assertEquals(
            10,
            (int) new Day08Solver(input).solveB()
        );
    }
}