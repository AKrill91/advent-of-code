package com.stexnistudios.adventofcode;

import org.javatuples.Pair;

import java.util.regex.Matcher;
import java.util.regex.Pattern;

public class Day09aSolver extends Solver {
    public Day09aSolver(String input) {
        super(input);
    }

    @Override
    public Integer call() {
        StringBuilder output = new StringBuilder();
        boolean inMarker = false;
        String input = getInput().replaceAll("\\s", "");
        StringBuilder marker = new StringBuilder();
        int inputLen = input.length();

        for (int i = 0; i < inputLen; ++i) {
            char c = input.charAt(i);
            if (c == '(' && !inMarker) {
                inMarker = true;

                //Reset marker buffer
                marker.setLength(0);

            } else if (c == ')' && inMarker) {
                inMarker = false;
                Pair<Integer, Integer> parsed = parseMarker(marker.toString());

                StringBuilder dataSection = new StringBuilder();

                //Go to next character
                ++i;
                for (int j = 0; j < parsed.getValue0() && i + j < inputLen; ++j) {
                    dataSection.append(input.charAt(i + j));
                }

                for (int j = 0; j < parsed.getValue1(); ++j) {
                    output.append(dataSection);
                }

                i += parsed.getValue0() - 1;

            } else if (inMarker) {
                marker.append(c);
            } else {
                output.append(c);
            }
        }

        return output.length();
    }

    private Pair<Integer, Integer> parseMarker(String marker) {
        Pattern pattern = Pattern.compile("(\\d+)x(\\d+)");
        Matcher matcher = pattern.matcher(marker);

        if (!matcher.matches()) {
            logger.warn("Unable to parse marker {} to components", marker);
            return new Pair<>(0, 0);
        }

        return new Pair<>(
            Integer.parseInt(matcher.group(1)),
            Integer.parseInt(matcher.group(2))
        );
    }
}
