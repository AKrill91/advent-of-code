package com.stexnistudios.adventofcode.day21;

import com.stexnistudios.adventofcode.Solver;

import java.util.regex.Matcher;
import java.util.regex.Pattern;

public class Day21aSolver extends Solver {

    private final Pattern swapPos = Pattern.compile(
        "swap position (\\d+) with position (\\d+)"
    );
    private final Pattern swapLet = Pattern.compile(
        "swap letter ([a-z]) with letter ([a-z])"
    );
    private final Pattern rotLeftSteps = Pattern.compile(
        "rotate left (\\d+) steps?"
    );
    private final Pattern rotRightSteps = Pattern.compile(
        "rotate right (\\d+) steps?"
    );
    private final Pattern rotLetter = Pattern.compile(
        "rotate based on position of letter ([a-z])"
    );
    private final Pattern reverse = Pattern.compile(
        "reverse positions (\\d+) through (\\d+)"
    );
    private final Pattern move = Pattern.compile(
        "move position (\\d+) to position (\\d+)"
    );

    private final char[] initial;

    public Day21aSolver(String input) {
        this(input, "abcdefgh");
    }

    public Day21aSolver(String input, String start) {
        super(input);

        initial = start.toCharArray();
    }

    @Override
    public String call() {
        char[] output = initial;

        for (String step : getInput().split("\n")) {
            step = step.trim();
            char[] before = output;
            if (step.contains("swap")) {
                output = handleSwap(step, output);
            } else if (step.contains("rotate")) {
                output = handleRotate(step, output);
            } else if (step.contains("reverse")) {
                output = handleReverse(step, output);
            } else if (step.contains("move")) {
                output = handleMove(step, output);
            }

            logger.info("{} -> {} -> {}", before, step, output);
        }

        return new String(output);
    }

    private int getIndexOfChar(char needle, char[] haystack) {
        int output = -1;

        for (int i = 0; i < haystack.length && output < 0; ++i) {
            if (haystack[i] == needle) {
                output = i;
            }
        }

        return output;
    }

    private char[] handleSwap(String instruction, char[] input) {
        char[] output = input;
        int from;
        int to;

        Matcher matcher = swapPos.matcher(instruction);

        if (matcher.matches()) {
            from = Integer.parseInt(matcher.group(1));
            to = Integer.parseInt(matcher.group(2));
        } else {
            matcher = swapLet.matcher(instruction);
            if (matcher.matches()) {
                from = getIndexOfChar(matcher.group(1).charAt(0), input);
                to = getIndexOfChar(matcher.group(2).charAt(0), input);
            } else {
                logger.error("Invalid swap instruction found: {}", instruction);
                return output;
            }
        }

        output = swap(
            output,
            from,
            to
        );

        return output;
    }

    private char[] handleRotate(String instruction, char[] input) {
        char[] output = input;

        Matcher matcher = rotLeftSteps.matcher(instruction);

        int amount;

        if (matcher.matches()) {
            amount = -1 * Integer.parseInt(matcher.group(1));
        } else {
            matcher = rotRightSteps.matcher(instruction);

            if (matcher.matches()) {
                amount = Integer.parseInt(matcher.group(1));
            } else {
                matcher = rotLetter.matcher(instruction);

                if (matcher.matches()) {
                    amount = getIndexOfChar(matcher.group(1).charAt(0), input) + 1;

                    if (amount > 4) {
                        ++amount;
                    }
                } else {
                    logger.error(
                        "Invalid rotate instruction found: {}",
                        instruction
                    );

                    return output;
                }
            }
        }

        while (amount < 0) {
            amount += input.length;
        }

        amount %= input.length;

        output = new char[input.length];

        for (int i = 0; i < output.length; ++i) {
            int pos = (output.length - amount + i) % output.length;
            output[i] = input[pos];
        }

        return output;
    }

    private char[] handleReverse(String instruction, char[] input) {
        char[] output = input;

        Matcher matcher = reverse.matcher(instruction);

        if (matcher.matches()) {
            int from = Integer.parseInt(matcher.group(1));
            int to = Integer.parseInt(matcher.group(2));

            output = new char[input.length];

            for (int i = 0; i < output.length; ++i) {
                char c = input[i];

                if (i >= from && i <= to) {
                    c = input[from + to - i];
                }

                output[i] = c;
            }
        } else {
            logger.info("Invalid reverse instruction found: {}", instruction);
        }

        return output;
    }

    private char[] handleMove(String instruction, char[] input) {
        char[] output = input;

        Matcher matcher = move.matcher(instruction);

        if (matcher.matches()) {
            int from = Integer.parseInt(matcher.group(1));
            int to = Integer.parseInt(matcher.group(2));
            int min = Math.min(from, to);
            int max = Math.max(from, to);

            output = new char[input.length];

            for (int i = 0; i < output.length; ++i) {
                int pos = i;

                if (pos >= min && pos <= max) {
                    //Are we moving right or left?
                    int direction = from == min ? 1 : -1;

                    if (pos == to) {
                        pos = from;
                    } else {
                        pos += direction;
                    }
                }

                output[i] = input[pos];
            }
        } else {
            logger.error("Invalid move instruction found: {}", instruction);
        }

        return output;
    }

    private char[] swap(char[] input, int start, int end) {
        char[] output = new char[input.length];

        for (int i = 0; i < input.length; ++i) {
            char c = input[i];

            if (i == start) {
                c = input[end];
            } else if (i == end) {
                c = input[start];
            }

            output[i] = c;
        }

        return output;
    }
}
