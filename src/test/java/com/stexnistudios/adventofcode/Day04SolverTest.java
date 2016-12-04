package com.stexnistudios.adventofcode;

import org.junit.Assert;
import org.junit.Test;

public class Day04SolverTest {
    @Test
    public void testDay04a() {
        String input = "aaaaa-bbb-z-y-x-123[abxyz]\n" +
            "a-b-c-d-e-f-g-h-987[abcde]\n" +
            "not-a-real-room-404[oarel]\n" +
            "totally-real-room-200[decoy]\n";

        Day04aSolver solver = new Day04aSolver(input);

        Assert.assertEquals(1514, solver.call().intValue());
    }
}