package com.stexnistudios.adventofcode.year2017.day11;

import org.junit.Assert;
import org.junit.Test;

import static org.junit.Assert.*;

public class Day11SolverTest {

    @Test
    public void solveA() {
        Assert.assertEquals(3, (int) new Day11Solver("ne,ne,ne").solveA());
        Assert.assertEquals(0, (int) new Day11Solver("ne,ne,sw,sw").solveA());
        Assert.assertEquals(2, (int) new Day11Solver("ne,ne,s,s").solveA());
        Assert.assertEquals(3, (int) new Day11Solver("se,sw,se,sw,sw").solveA());
    }

    @Test
    public void solveB() {
    }
}