package com.stexnistudios.adventofcode.year2017.day10;

import org.junit.Assert;
import org.junit.Test;

import static org.junit.Assert.*;

public class Day10SolverTest {

    @Test
    public void solveA() {
        String input = "3,4,1,5";

        Day10Solver solver = new Day10Solver(input, 5);

        Assert.assertEquals(12, (int) solver.solveA());
    }

    @Test
    public void solveB() {
        Assert.assertEquals(
            "a2582a3a0e66e6e86e3812dcb672a272",
            new Day10Solver("").solveB()
        );

        Assert.assertEquals(
            "33efeb34ea91902bb2f59c9920caa6cd",
            new Day10Solver("AoC 2017").solveB()
        );

        Assert.assertEquals(
            "3efbe78a8d82f29979031a4aa0b16a9d",
            new Day10Solver("1,2,3").solveB()
        );

        Assert.assertEquals(
            "63960835bcdc130f0b66d7ff4f6a5a8e",
            new Day10Solver("1,2,4").solveB()
        );
    }
}