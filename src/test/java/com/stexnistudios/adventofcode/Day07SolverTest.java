package com.stexnistudios.adventofcode;

import com.stexnistudios.adventofcode.gate.IGate;
import org.junit.Test;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.Map;

import static org.junit.Assert.*;

public class Day07SolverTest {

    @Test
    public void testSample() {
        List<String> input = Arrays.asList(
            (
                "123 -> x\n" +
                    "456 -> y\n" +
                    "x AND y -> d\n" +
                    "x OR y -> e\n" +
                    "x LSHIFT 2 -> f\n" +
                    "y RSHIFT 2 -> g\n" +
                    "NOT x -> h\n" +
                    "NOT y -> i"
            ).split("\n")
        );

        List<String> ids = new ArrayList<>();
        ids.add("d");
        ids.add("e");
        ids.add("f");
        ids.add("g");
        ids.add("h");
        ids.add("i");
        ids.add("x");
        ids.add("y");

        Day07Solver solver = new Day07Solver(input);

        solver.run();

        Map<String, IGate> wires = solver.getWires();

        ids.forEach(id -> {
            assertTrue("Has wire " + id, wires.containsKey(id));
        });

        assertEquals(72, wires.get("d").getValue());
        assertEquals(65079, wires.get("i").getValue());
    }
}
