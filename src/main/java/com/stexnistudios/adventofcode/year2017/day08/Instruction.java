package com.stexnistudios.adventofcode.year2017.day08;

import java.util.HashMap;
import java.util.Map;
import java.util.Optional;
import java.util.function.BiFunction;

public class Instruction {
    private final String register;
    private final int direction;
    private final int amount;

    private final String compare;
    private final BiFunction<Integer, Integer, Boolean> func;
    private final int right;

    private static final Map<String, BiFunction<Integer, Integer, Boolean>> funcs;

    static {
        Map<String, BiFunction<Integer, Integer, Boolean>> tmp = new HashMap<>();

        tmp.put(">", (left, right) -> left > right);
        tmp.put("<", (left, right) -> left < right);
        tmp.put(">=", (left, right) -> left >= right);
        tmp.put("<=", (left, right) -> left <= right);
        tmp.put("==", Integer::equals);
        tmp.put("!=", (left, right) -> !left.equals(right));

        funcs = tmp;
    }

    public Instruction(String line) {
        String[] parts = line.split(" ");
        assert parts.length == 7;
        register = parts[0];
        direction = parts[1].equals("inc") ? 1 : -1;
        amount = Integer.parseInt(parts[2]);
        compare = parts[4];
        func = funcs.get(parts[5]);
        right = Integer.parseInt(parts[6]);
    }

    public void apply(Map<String, Integer> registers) {
        int current = Optional.ofNullable(registers.get(register)).orElse(0);

        int left = Optional.ofNullable(registers.get(compare)).orElse(0);
        if (func.apply(left, right)) {
            current += direction * amount;
        }

        registers.put(register, current);
    }
}
