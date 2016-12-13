package com.stexnistudios.adventofcode.day13;

import com.stexnistudios.adventofcode.util.Point;
import org.junit.Assert;
import org.junit.Test;

public class Day13SolverTest {

    @Test
    public void testA() {
        String input = "10";

        Day13aSolver solver = new Day13aSolver(
            input,
            new Point(1, 1),
            new Point(7, 4)
        );

        Assert.assertEquals(11, solver.call().intValue());
    }

}