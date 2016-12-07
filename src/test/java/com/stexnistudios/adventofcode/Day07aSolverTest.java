package com.stexnistudios.adventofcode;

import org.junit.Assert;
import org.junit.Test;

import static org.junit.Assert.*;

public class Day07aSolverTest {
    @Test
    public void testA() throws Exception {
        String input = "abba[mnop]qrst\n" +
            "abcd[bddb]xyyx\n" +
            "aaaa[qwer]tyui\n" +
            "ioxxoj[asdfgh]zxcvbn";

        Day07aSolver solver = new Day07aSolver(input);

        Assert.assertEquals(2, solver.call().intValue());
    }
}