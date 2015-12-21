package com.stexnistudios.adventofcode;

import org.junit.Test;

import java.util.ArrayList;
import java.util.List;

import static org.junit.Assert.assertEquals;

public class Day09SolverTest {

    @Test
    public void sampleTest() {
        List<String> input = new ArrayList<>();
        input.add("London to Dublin = 464");
        input.add("London to Belfast = 518");
        input.add("Dublin to Belfast = 141");

        Day09Solver solver = new Day09Solver(input);
        solver.run();

        assertEquals(605, solver.getShortestRoute());
    }
}
