package com.stexnistudios.adventofcode;

import com.stexnistudios.adventofcode.day07.Day07aSolver;
import com.stexnistudios.adventofcode.day07.Day07bSolver;
import org.junit.Assert;
import org.junit.Test;

public class Day07SolverTest {
    @Test
    public void testA() {
        String input = "abba[mnop]qrst\n" +
            "abcd[bddb]xyyx\n" +
            "aaaa[qwer]tyui\n" +
            "ioxxoj[asdfgh]zxcvbn";

        Day07aSolver solver = new Day07aSolver(input);

        Assert.assertEquals(2, solver.call().intValue());
    }

    @Test
    public void testB() {
        String input = "aba[bab]xyz\n" +
            "xyx[xyx]xyx\n" +
            "aaa[kek]eke\n" +
            "zazbz[bzb]cdb";

        Day07bSolver solver = new Day07bSolver(input);
        Assert.assertEquals(3, solver.call().intValue());
    }
}