package com.stexnistudios.adventofcode.day20;

import com.stexnistudios.adventofcode.Solver;
import com.stexnistudios.adventofcode.util.Range;

import java.util.ArrayList;
import java.util.Collections;
import java.util.List;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

public class Day20aSolver extends Solver {
    private final List<Range<Long>> blacklists;

    public Day20aSolver(String input) {
        super(input);

        blacklists = new ArrayList<>();
    }

    @Override
    public Long call() {
        Pattern rangePattern = Pattern.compile("(\\d+)-(\\d+)");
        for (String range : getInput().split("\n")) {
            range = range.trim();
            Matcher matcher = rangePattern.matcher(range);

            if (!matcher.matches()) {
                logger.warn("Invalid range found: {}", range);
            }

            Long min = Long.parseLong(matcher.group(1));
            Long max = Long.parseLong(matcher.group(2));

            blacklists.add(new Range<>(min, max));
        }

        Collections.sort(blacklists);

        long position = 0;
        for (int i = 0; i < blacklists.size(); ++i) {
            Range<Long> range = blacklists.get(i);

            if (range.includes(position)) {
                position = range.getMaximum() + 1;
            } else if(range.getMinimum() > position) {
                break;
            }
        }

        return position;
    }


}
