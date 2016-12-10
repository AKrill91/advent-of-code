package com.stexnistudios.adventofcode.day01;

import com.stexnistudios.adventofcode.util.Point;

import java.util.ArrayList;
import java.util.List;

public class Day01aSolver extends Day01Solver {
    public Day01aSolver(String input) {
        super(input);

    }

    @Override
    public Point call() throws Exception {
        logger.info("I've got {} instructions", instructions.size());
        Point position = new Point();
        int direction = 1;
        List<String> steps = new ArrayList<>();
        steps.add("start: " + position);

        for (String instruction : instructions) {
            String dir = instruction.substring(0, 1);
            int length = Integer.parseInt(instruction.substring(1));
            if (dir.equals("L")) {
                direction += 1;
            } else {
                direction -= 1;
            }
            direction = (direction + 4) % 4;

            Point movement = new Point();

            switch (direction) {
                case 0:
                    movement = new Point(length, 0);
                    break;
                case 1:
                    movement = new Point(0, length);
                    break;
                case 2:
                    movement = new Point(-length, 0);
                    break;
                case 3:
                    movement = new Point(0, -length);
                    break;
            }

            position = position.add(movement);
            steps.add(instruction + ": " + position);
        }
        int stepCounter = 0;
        for (String step : steps) {
            logger.info(
                "Step {}: {}",
                String.format("%03d", stepCounter++),
                step
            );
        }

        return position;
    }
}
