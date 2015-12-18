package com.stexnistudios.adventofcode;

import org.junit.Test;

import java.util.ArrayList;
import java.util.List;

import static org.junit.Assert.*;

public class Day08SolverTest {

    @Test
    public void testRunEmptyString() {
        List<String> input = new ArrayList<>();
        input.add("\"\"");

        Day08Solver solver = new Day08Solver(input);

        solver.run();

        assertEquals(2, solver.getCodeCount());
        assertEquals(0, solver.getLiteralCount());
        assertEquals(6, solver.getEncodedCount());
    }

    @Test
    public void testRunEscapedQuote() {
        List<String> input = new ArrayList<>();
        input.add("\"aaa\\\"aaa\"");

        Day08Solver solver = new Day08Solver(input);
        solver.run();

        assertEquals(10, solver.getCodeCount());
        assertEquals(7, solver.getLiteralCount());
        assertEquals(16, solver.getEncodedCount());
    }

    @Test
    public void testRunUnicode() {
        List<String> input = new ArrayList<>();
        input.add("\"\\x27\"");

        Day08Solver solver = new Day08Solver(input);
        solver.run();

        assertEquals(6, solver.getCodeCount());
        assertEquals(1, solver.getLiteralCount());
        assertEquals(11, solver.getEncodedCount());
    }
}
