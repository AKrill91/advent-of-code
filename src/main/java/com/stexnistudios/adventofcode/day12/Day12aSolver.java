package com.stexnistudios.adventofcode.day12;

import com.stexnistudios.adventofcode.Solver;

import java.util.HashMap;
import java.util.Map;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

public class Day12aSolver extends Solver {
    private final Map<Character, Integer> registers;

    private final Pattern cpyValPattern;
    private final Pattern cpyRegPattern;
    private final Pattern incPattern;
    private final Pattern decPattern;
    private final Pattern jnzValPattern;
    private final Pattern jnzRegPattern;

    public Day12aSolver(String input) {
        super(input);

        registers = new HashMap<>();
        registers.put('a', 0);
        registers.put('b', 0);
        registers.put('c', 0);
        registers.put('d', 0);

        cpyValPattern = Pattern.compile("cpy (-?\\d+) ([a-z])");
        cpyRegPattern = Pattern.compile("cpy ([a-z]) ([a-z])");
        incPattern = Pattern.compile("inc ([a-z])");
        decPattern = Pattern.compile("dec ([a-z])");
        jnzValPattern = Pattern.compile("jnz (-?\\d+) (-?\\d+)");
        jnzRegPattern = Pattern.compile("jnz ([a-z]) (-?\\d+)");
    }

    @Override
    public Integer call() {
        String[] instructions = getInput().split("\n");

        for (int i = 0; i < instructions.length; ++i) {
            String instruction = instructions[i].trim();

            switch (instruction.substring(0, 3)) {
                case "cpy":
                    handleCopy(instruction);
                    break;
                case "inc":
                    handleIncrement(instruction);
                    break;
                case "dec":
                    handleDecrement(instruction);
                    break;
                case "jnz":
                    Matcher matcher = jnzRegPattern.matcher(instruction);
                    int amount;
                    int value;
                    if (matcher.matches()) {
                        Character register = matcher.group(1).charAt(0);
                        value = registers.get(register);
                    } else {
                        matcher = jnzValPattern.matcher(instruction);
                        if (matcher.matches()) {
                            value = Integer.parseInt(matcher.group(1));
                        } else {
                            logger.warn(
                                "Invalid instruction found: {}",
                                instruction
                            );
                            continue;
                        }
                    }
                    amount = Integer.parseInt(matcher.group(2));

                    if (value != 0) {
                        i += amount - 1 ;
                    }
                    break;
                default:
                    logger.warn("Invalid instruction found: {}", instruction);
                    break;
            }
        }


        return registers.get('a');
    }

    private void handleCopy(String instruction) {
        Matcher matcher = cpyValPattern.matcher(instruction);
        int value;

        if (!matcher.matches()) {
            matcher = cpyRegPattern.matcher(instruction);
            if (matcher.matches()) {
                value = registers.get(matcher.group(1).charAt(0));
            } else {
                logger.warn("Invalid instruction found: {}", instruction);
                return;
            }
        } else {
            value = Integer.parseInt(matcher.group(1));
        }

        Character to = matcher.group(2).charAt(0);

        registers.put(to, value);
    }

    private void handleIncrement(String instruction) {
        Matcher matcher = incPattern.matcher(instruction);
        if (matcher.matches()) {
            Character register = matcher.group(1).charAt(0);

            registers.put(register, registers.get(register) + 1);
        } else {
            logger.warn("Invalid instruction found: {}", instruction);
        }

    }

    private void handleDecrement(String instruction) {
        Matcher matcher = decPattern.matcher(instruction);

        if (matcher.matches()) {
            Character register = matcher.group(1).charAt(0);

            registers.put(register, registers.get(register) - 1);
        } else {
            logger.warn("Invalid instruction found: {}", instruction);
        }
    }
}
