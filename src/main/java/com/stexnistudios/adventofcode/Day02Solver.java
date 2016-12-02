package com.stexnistudios.adventofcode;

import com.stexnistudios.adventofcode.util.Point;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

public class Day02Solver extends Solver {

    private final List<List<Integer>> grid;

    public Day02Solver(String input) {
        super(input);

        grid = new ArrayList<>();
        grid.add(Arrays.asList(1, 2, 3));
        grid.add(Arrays.asList(4, 5, 6));
        grid.add(Arrays.asList(7, 8, 9));
    }

    @Override
    public Integer call() throws Exception {
        int code = 0;
        Point position = new Point(1, 1);

        List<String> instructions = Arrays.asList(getInput().split("\n"));

        logger.info("Code is {} long", instructions.size());

        for (int i = 0; i < instructions.size(); ++i) {
            String instruction = instructions.get(i).trim();
            for (int j = 0; j < instruction.length(); ++j) {
                char c = instruction.charAt(j);
                int offsetX = 0;
                int offsetY = 0;
                switch (c) {
                    case 'U':
                        offsetY = -1;
                        break;
                    case 'D':
                        offsetY = 1;
                        break;
                    case 'L':
                        offsetX = -1;
                        break;
                    case 'R':
                        offsetX = 1;
                        break;
                    default:
                        logger.warn(
                            "Invalid character {} found at position {} of instruction {}",
                            c,
                            j,
                            i
                        );
                        break;
                }

                position = move(position, new Point(offsetX, offsetY));
            }

            int newValue = (int)Math.pow(
                10,
                (instructions.size() - i - 1)
            ) * getValueAtPosition(position);


            code += newValue;
        }

        return code;
    }

    private int getValueAtPosition(Point position) {
        return grid.get(position.getY()).get(position.getX());
    }

    private Point move(Point current, Point offset) {
        Point desired = current.add(offset);
        int newX = Math.max(
            0,
            Math.min(grid.get(0).size() - 1, desired.getX())
        );
        int newY = Math.max(0, Math.min(grid.size() - 1, desired.getY()));

        return new Point(
            newX,
            newY
        );
    }
}
