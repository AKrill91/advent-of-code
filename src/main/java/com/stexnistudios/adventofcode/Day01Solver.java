package com.stexnistudios.adventofcode;

import java.io.IOException;
import java.util.List;

public class Day01Solver extends Solver {
    public Day01Solver(List<String> input) {
        super(input);
    }

    public Day01Solver(String fileName) throws IOException {
        super(fileName);
    }

    @Override
    public void run() {
        int floor = 0;
        int firstBasement = Integer.MAX_VALUE;

        String steps = input.get(0);

        for (int i = 0; i < steps.length(); ++i) {
            char c = steps.charAt(i);

            switch (c) {
                case '(':
                    ++floor;
                    break;
                case ')':
                    --floor;
                    if (floor == -1) {
                        firstBasement = Integer.min(i, firstBasement);
                    }
                    break;
            }
        }

        logger.info(
            "Santa ended up on floor {}, he went to the basement on step {}",
            floor,
            firstBasement + 1
        );
    }
}
