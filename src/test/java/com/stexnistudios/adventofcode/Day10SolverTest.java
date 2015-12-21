package com.stexnistudios.adventofcode;

import org.junit.Test;
import org.junit.runner.RunWith;
import org.junit.runners.Parameterized;

import java.util.ArrayList;
import java.util.Collection;
import java.util.List;

import static org.junit.Assert.assertEquals;

@RunWith(Parameterized.class)
public class Day10SolverTest {

    @Parameterized.Parameters
    public static Collection<Object[]> getParameters() {
        List<Object[]> params = new ArrayList<>();
        params.add(new Object[]{"1", "11"});
        params.add(new Object[]{"11", "21" });
        params.add(new Object[]{"21", "1211"});
        params.add(new Object[]{"1211", "111221"});
        params.add(new Object[]{"111221", "312211"});

        return params;
    }

    final String input;
    final String expected;

    public Day10SolverTest(String input, String expected) {
        this.input = input;
        this.expected = expected;
    }

    @Test
    public void testSamples() {
        List<String> in = new ArrayList<>();
        in.add(input);
        in.add("1");
        Day10Solver solver = new Day10Solver(in);

        solver.run();
        assertEquals(expected, solver.getResult());
    }
}
