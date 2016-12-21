package com.stexnistudios.adventofcode.day21;

import org.junit.Assert;
import org.junit.Test;

public class Day21SolverTest {

    @Test
    public void testA() {
        String input = "swap position 4 with position 0\n" +
            "swap letter d with letter b\n" +
            "reverse positions 0 through 4\n" +
            "rotate left 1 step\n" +
            "move position 1 to position 4\n" +
            "move position 3 to position 0\n" +
            "rotate based on position of letter b\n" +
            "rotate based on position of letter d";

        Assert.assertEquals("decab", new Day21aSolver(input, "abcde").call());
    }
}