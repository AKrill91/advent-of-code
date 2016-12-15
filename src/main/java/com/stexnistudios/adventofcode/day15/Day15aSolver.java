package com.stexnistudios.adventofcode.day15;

import com.stexnistudios.adventofcode.Solver;

import java.util.ArrayList;
import java.util.List;
import java.util.regex.Matcher;
import java.util.regex.Pattern;
import java.util.stream.IntStream;

public class Day15aSolver extends Solver {

    private static class Disc {
        private final int id;
        private final int totalPositions;
        private final int initialPosition;

        private Disc(int id, int totalPositions, int initialPosition) {
            this.id = id;
            this.totalPositions = totalPositions;
            this.initialPosition = initialPosition;
        }

        public boolean isOpenAtTime(int time) {
            return positionAtTime(time) == 0;
        }

        public int positionAtTime(int time) {
            return (initialPosition + time) % totalPositions;
        }
    }

    private final List<Disc> discs;
    private final Pattern discPattern;

    public Day15aSolver(String input) {
        super(input);

        discs = new ArrayList<>();

        discPattern = Pattern.compile(
            "^Disc #(\\d+) has (\\d+) positions?; at time=0, it is at position (\\d+).$"
        );
    }

    @Override
    public Integer call() {
        for (String instruction : getInput().split("\n")) {
            Matcher matcher = discPattern.matcher(instruction.trim());

            if (matcher.find()) {
                discs.add(
                    new Disc(
                        Integer.parseInt(matcher.group(1)),
                        Integer.parseInt(matcher.group(2)),
                        Integer.parseInt(matcher.group(3))
                    )
                );
            }
        }

        return IntStream.range(0, Integer.MAX_VALUE)
            .filter(
                time -> !discs.stream()
                    .anyMatch(
                        disc -> !disc.isOpenAtTime(time + disc.id)
                    )
            )
            .findFirst()
            .getAsInt();
    }
}
