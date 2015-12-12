package com.stexnistudios.adventofcode;

import com.stexnistudios.adventofcode.util.Point;

import java.io.IOException;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class Day03Solver extends Solver {

    private final int santaCount;

    private static class Santa {
        Point currentPosition;

        public Santa() {
            currentPosition = new Point();
        }

        public void offset(long x, long y) {
            currentPosition = new Point(
                currentPosition.x + x,
                currentPosition.y + y
            );
        }
    }

    public Day03Solver(List<String> input) {
        this(input, 1);
    }

    public Day03Solver(String fileName) throws IOException {
        this(fileName, 1);
    }

    public Day03Solver(List<String> input, int santaCount) {
        super(input);
        this.santaCount = santaCount;
    }

    public Day03Solver(String fileName, int santaCount) throws IOException {
        super(fileName);
        this.santaCount = santaCount;
    }

    @Override
    public void run() {
        String steps = input.get(0);

        List<Santa> santas = new ArrayList<>();

        for(int i = 0; i < santaCount; ++i) {
            santas.add(new Santa());
        }

        Map<Point, Integer> houses = new HashMap<>();

        houses.put(santas.get(0).currentPosition, santas.size());

        for(int i = 0; i < steps.length(); ++i) {
            char c = steps.charAt(i);
            Santa current = santas.get(i % santas.size());

            switch(c) {
                case '<':
                    current.offset(-1, 0);
                    break;
                case '>':
                    current.offset(1, 0);
                    break;
                case '^':
                    current.offset(0, 1);
                    break;
                case 'v':
                    current.offset(0, -1);
                    break;
            }

            Point pos = current.currentPosition;

            if(!houses.containsKey(pos)) {
                houses.put(pos, 0);
            }

            houses.put(pos, houses.get(pos) + 1);
        }

        logger.info(
            "With {} santas, total of {} houses got presents.",
            santas.size(),
            houses.size()
        );
    }
}
