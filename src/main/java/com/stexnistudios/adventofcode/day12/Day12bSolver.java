package com.stexnistudios.adventofcode.day12;

import java.util.HashMap;
import java.util.Map;

public class Day12bSolver extends Day12Solver {

    public Day12bSolver(String input) {
        super(input, getRegisters());
    }

    private static Map<Character, Integer> getRegisters() {
        Map<Character, Integer> registers = new HashMap<>();
        registers.put('a', 0);
        registers.put('b', 0);
        registers.put('c', 1);
        registers.put('d', 0);

        return registers;
    }
}
