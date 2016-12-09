package com.stexnistudios.adventofcode;

import org.javatuples.Pair;
import org.junit.Assert;
import org.junit.Test;

import java.util.Arrays;
import java.util.List;

public class Day09aSolverTest {

    @Test
    public void testA() {
        List<Pair<String, Integer>> tests = Arrays.asList(
            new Pair<>("ADVENT", 6),
            new Pair<>("A(1x5)BC", 7),
            new Pair<>("(3x3)XYZ", 9),
            new Pair<>("A(2x2)BCD(2x2)EFG", 11),
            new Pair<>("(6x1)(1x3)A", 6),
            new Pair<>("X(8x2)(3x3)ABCY", 18)
        );

        tests.forEach(
            pair -> {
                Day09aSolver solver = new Day09aSolver(pair.getValue0());
                Assert.assertEquals(pair.getValue1(), solver.call());
            }
        );
    }
}