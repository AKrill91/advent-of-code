package com.stexnistudios.adventofcode.year2017.day08;

import com.stexnistudios.adventofcode.Solver;

import java.util.*;
import java.util.stream.Collectors;

public class Day08Solver extends Solver {
    public Day08Solver(String input) {
        super(input);
    }

    @Override
    public Integer solveA() {
        Map<String, Integer> registers = new HashMap<>();
        List<Instruction> instructions = Arrays.stream(getInput().split("\n"))
            .map(Instruction::new)
            .collect(Collectors.toList());

        for (Instruction instruction : instructions) {
            instruction.apply(registers);
        }

        return registers.values()
            .stream()
            .max(Comparator.naturalOrder())
            .get();

    }

    @Override
    public Integer solveB() {
        Map<String, Integer> registers = new HashMap<>();
        List<Instruction> instructions = Arrays.stream(getInput().split("\n"))
            .map(Instruction::new)
            .collect(Collectors.toList());

        int absoluteMax = 0;

        for (Instruction instruction : instructions) {
            instruction.apply(registers);

            absoluteMax = Math.max(
                absoluteMax,
                registers.values()
                    .stream()
                    .max(Comparator.naturalOrder())
                    .get()
            );
        }

        return absoluteMax;
    }
}
