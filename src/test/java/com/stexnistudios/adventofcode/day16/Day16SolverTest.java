package com.stexnistudios.adventofcode.day16;

import org.junit.Assert;
import org.junit.Test;

public class Day16SolverTest {

    @Test
    public void testA() {
        String input = "10000";
        Assert.assertEquals("01100", new Day16aSolver(input, 20).call());
    }
}