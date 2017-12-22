package com.stexnistudios.adventofcode.year2017.day03;

import org.junit.Assert;
import org.junit.Test;

public class Day03SolverTest {

    @Test
    public void solveA() {
        Assert.assertEquals(0, (long) new Day03Solver("1").solveA());
        Assert.assertEquals(3, (long) new Day03Solver("12").solveA());
        Assert.assertEquals(2, (long) new Day03Solver("23").solveA());
        Assert.assertEquals(31, (long) new Day03Solver("1024").solveA());
    }

    @Test
    public void solveB() {
        Assert.assertEquals(122, (long) new Day03Solver("59").solveB());
        Assert.assertEquals(362, (long) new Day03Solver("352").solveB());
        Assert.assertEquals(133, (long) new Day03Solver("123").solveB());
    }
}