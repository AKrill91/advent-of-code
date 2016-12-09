package com.stexnistudios.adventofcode;

import com.stexnistudios.adventofcode.day09.Day09aSolver;
import com.stexnistudios.adventofcode.day09.Day09bSolver;
import org.javatuples.Pair;
import org.junit.Assert;
import org.junit.Test;

import java.util.Arrays;
import java.util.List;

public class Day09SolverTest {

    @Test
    public void testA() {
        List<Pair<String, Long>> tests = Arrays.asList(
            new Pair<>("ADVENT", 6L),
            new Pair<>("A(1x5)BC", 7L),
            new Pair<>("(3x3)XYZ", 9L),
            new Pair<>("A(2x2)BCD(2x2)EFG", 11L),
            new Pair<>("(6x1)(1x3)A", 6L),
            new Pair<>("X(8x2)(3x3)ABCY", 18L)
        );

        tests.forEach(
            pair -> {
                Day09aSolver solver = new Day09aSolver(pair.getValue0());
                Assert.assertEquals(pair.getValue1(), solver.call());
            }
        );
    }

    @Test
    public void testB() {
        List<Pair<String, Long>> tests = Arrays.asList(
            new Pair<>("(3x3)XYZ", 9L),
            new Pair<>("X(8x2)(3x3)ABCY", (long)"XABCABCABCABCABCABCY".length()),
            new Pair<>("(27x12)(20x12)(13x14)(7x10)(1x12)A", 241920L),
            new Pair<>(
                "(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN",
                445L
            )
        );

        tests.forEach(
            pair -> {
                Day09bSolver solver = new Day09bSolver(pair.getValue0());
                Assert.assertEquals(pair.getValue1(), solver.call());
            }
        );
    }
}