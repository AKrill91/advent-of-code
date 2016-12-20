package com.stexnistudios.adventofcode.day20;

import com.stexnistudios.adventofcode.util.Range;

public class Day20bSolver extends Day20aSolver {
    private final long minimum;
    private final long maximum;

    public Day20bSolver(String input) {
        this(input, 0, 4294967295L);
    }

    public Day20bSolver(String input, long minimum, long maximum) {
        super(input);

        this.minimum = minimum;
        this.maximum = maximum;
    }

    @Override
    public Long call() {
        long allowedCount = 0;
        int rangeIndex = 0;

        for (long i = minimum; i <= maximum; ++i) {
            if (rangeIndex < blacklists.size()) {
                Range<Long> blacklist = blacklists.get(rangeIndex);

                if (blacklist.includes(i)) {
                    i = blacklist.getMaximum();
                    ++rangeIndex;
                } else if (blacklist.getMinimum() > i) {
                    allowedCount += blacklist.getMinimum() - i;
                    i = blacklist.getMinimum();
                } else {
                    ++rangeIndex;
                }
            } else {
                allowedCount += maximum - i + 1;
                break;
            }
        }

        return allowedCount;
    }
}
