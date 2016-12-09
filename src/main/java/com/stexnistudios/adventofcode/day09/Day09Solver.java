package com.stexnistudios.adventofcode.day09;

import com.stexnistudios.adventofcode.Solver;
import org.javatuples.Pair;

import java.util.regex.Matcher;
import java.util.regex.Pattern;

public class Day09Solver extends Solver {
    private final boolean recurse;

    public Day09Solver(String input, boolean recurse) {
        super(input);

        this.recurse = recurse;
    }

    @Override
    public Long call() {
        return decompress(getInput().replaceAll("\\s", ""));
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

    private long decompress(String input) {
        boolean inMarker = false;
        StringBuilder marker = new StringBuilder();
        int inputLen = input.length();
        long sum = 0;

        for (int i = 0; i < inputLen; ++i) {
            char c = input.charAt(i);
            if (c == '(' && !inMarker) {
                inMarker = true;

                //Reset marker buffer
                marker.setLength(0);
            } else if (c == ')' && inMarker) {
                inMarker = false;
                Pair<Integer, Integer> parsed = parseMarker(marker.toString());
                //Increment one
                ++i;
                if(recurse) {
                    String substr = input.substring(i, i + parsed.getValue0());
                    sum += decompress(substr) * parsed.getValue1();
                } else {
                    sum += parsed.getValue0() * parsed.getValue1();
                }

                i += parsed.getValue0() - 1;
            } else if (inMarker) {
                marker.append(c);
            } else {
                sum += 1;
            }
        }

        return sum;
    }
}
