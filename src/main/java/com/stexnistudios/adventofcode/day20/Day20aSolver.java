package com.stexnistudios.adventofcode.day20;

import com.stexnistudios.adventofcode.Solver;
import com.stexnistudios.adventofcode.util.Range;

import java.util.ArrayList;
import java.util.Collections;
import java.util.List;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

public class Day20aSolver extends Solver {
    protected final List<Range<Long>> blacklists;

    public Day20aSolver(String input) {
        super(input);

        blacklists = getBlackLists(input);
    }

    @Override
    public Long call() {
        long position = 0;
        for (Range<Long> range : blacklists) {
            if (range.includes(position)) {
                position = range.getMaximum() + 1;
            } else if (range.getMinimum() > position) {
                break;
            }
        }

        return position;
    }

    private static List<Range<Long>> getBlackLists(String input) {
        List<Range<Long>> ranges = new ArrayList<>();

        Pattern rangePattern = Pattern.compile("(\\d+)-(\\d+)");
        for (String range : input.split("\n")) {
            range = range.trim();
            Matcher matcher = rangePattern.matcher(range);

            if (!matcher.matches()) {
                throw new RuntimeException(
                    String.format("Invalid range found: %s", range)
                );
            }

            Long min = Long.parseLong(matcher.group(1));
            Long max = Long.parseLong(matcher.group(2));

            ranges.add(new Range<>(min, max));
        }

        Collections.sort(ranges);

        List<Range<Long>> merged = new ArrayList<>();
        Range<Long> current = ranges.get(0);
        long min = current.getMinimum();
        long max = current.getMaximum();
        for (int i = 1; i < ranges.size(); ++i) {
            Range<Long> range = ranges.get(i);

            //Check for overlap
            if (range.getMinimum() <= max + 1) {
                if (range.getMaximum() > max) {
                    max = range.getMaximum();
                }
            } else {
                merged.add(new Range<>(min, max));
                min = range.getMinimum();
                max = range.getMaximum();
            }
        }

        merged.add(new Range<>(min, max));

        return merged;
    }
}
