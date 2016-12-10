package com.stexnistudios.adventofcode;

import com.stexnistudios.adventofcode.day06.Day06Solver;
import com.stexnistudios.adventofcode.day06.Day06aSolver;
import com.stexnistudios.adventofcode.day06.Day06bSolver;
import org.junit.Assert;
import org.junit.Test;

public class Day06SolverTest {
    @Test
    public void testA() {
        String input = "eedadn\n" +
            "drvtee\n" +
            "eandsr\n" +
            "raavrd\n" +
            "atevrs\n" +
            "tsrnev\n" +
            "sdttsa\n" +
            "rasrtv\n" +
            "nssdts\n" +
            "ntnada\n" +
            "svetve\n" +
            "tesnvt\n" +
            "vntsnd\n" +
            "vrdear\n" +
            "dvrsen\n" +
            "enarar";

        Day06Solver solver = new Day06aSolver(input);
        Assert.assertEquals("easter", solver.call());
    }

    @Test
    public void testB() {
        String input = "eedadn\n" +
            "drvtee\n" +
            "eandsr\n" +
            "raavrd\n" +
            "atevrs\n" +
            "tsrnev\n" +
            "sdttsa\n" +
            "rasrtv\n" +
            "nssdts\n" +
            "ntnada\n" +
            "svetve\n" +
            "tesnvt\n" +
            "vntsnd\n" +
            "vrdear\n" +
            "dvrsen\n" +
            "enarar";

        Day06Solver solver = new Day06bSolver(input);
        Assert.assertEquals("advent", solver.call());
    }
}