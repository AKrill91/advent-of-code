package com.stexnistudios.adventofcode.day17;

import org.junit.Assert;
import org.junit.Test;

import java.util.LinkedHashMap;
import java.util.Map;

public class Day17SolverTest {

    @Test
    public void testA() throws Exception {
        Map<String, String> samples = new LinkedHashMap<>();
        samples.put("ihgpwlah", "DDRRRD");
        samples.put("kglvqrro", "DDUDRLRRUDRD");
        samples.put("ulqzkmiv", "DRURDRUDDLLDLUURRDULRLDUUDDDRR");

        for (Map.Entry<String, String> entry : samples.entrySet()) {
            Day17aSolver solver = new Day17aSolver(entry.getKey());

            Assert.assertEquals(entry.getValue(), solver.call());
        }
    }

    @Test
    public void testB() throws Exception {
        Map<String, Integer> samples = new LinkedHashMap<>();
        samples.put("ihgpwlah", 370);
        samples.put("kglvqrro", 492);
        samples.put("ulqzkmiv", 830);

        for (Map.Entry<String, Integer> entry : samples.entrySet()) {
            Day17bSolver solver = new Day17bSolver(entry.getKey());

            Assert.assertEquals(entry.getValue(), solver.call());
        }
    }

}