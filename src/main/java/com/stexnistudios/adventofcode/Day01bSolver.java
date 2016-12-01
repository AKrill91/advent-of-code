package com.stexnistudios.adventofcode;

import com.stexnistudios.adventofcode.util.Point;

import java.util.HashSet;
import java.util.Set;

public class Day01bSolver extends Day01Solver {

    public Day01bSolver(String input) {
        super(input);
    }

    @Override
    public Object call() throws Exception {
        logger.info("I've got {} instructions", instructions.size());
        Point position = new Point();
        int direction = 1;
        Set<Point> intersections = new HashSet<>();
        intersections.add(position);

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
                    movement = new Point(1, 0);
                    break;
                case 1:
                    movement = new Point(0, 1);
                    break;
                case 2:
                    movement = new Point(-1, 0);
                    break;
                case 3:
                    movement = new Point(0, -1);
                    break;
            }

            for (int i = 0; i < length; ++i) {
                position = position.add(movement);
                if (intersections.contains(position)) {
                    return position;
                }
                intersections.add(position);
            }
        }

        return null;
    }
}
