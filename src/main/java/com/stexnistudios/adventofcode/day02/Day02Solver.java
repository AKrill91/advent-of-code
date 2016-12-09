package com.stexnistudios.adventofcode.day02;

import com.stexnistudios.adventofcode.Solver;
import com.stexnistudios.adventofcode.util.Point;

import java.util.Arrays;
import java.util.List;

public abstract class Day02Solver extends Solver {

    private final List<List<String>> grid;
    private final Point initial;

    public Day02Solver(String input, List<List<String>> grid, Point initial) {
        super(input);

        this.grid = grid;
        this.initial = initial;
    }

    @Override
    public String call() {
        String code = "";
        Point position = initial;

        List<String> instructions = Arrays.asList(getInput().split("\n"));

        logger.info("Code is {} long", instructions.size());

        for (String instruction : instructions) {
            instruction = instruction.trim();
            for (int j = 0; j < instruction.length(); ++j) {
                position = move(position, instruction.charAt(j));
            }

            code += getValueAtPosition(position);
        }

        return code;
    }

    protected String getValueAtPosition(Point position) {
        return grid.get(position.getY()).get(position.getX());
    }

    protected Point move(Point current, char direction) {
        int offsetX = 0;
        int offsetY = 0;
        switch (direction) {
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
        }

        return moveByOffset(current, new Point(offsetX, offsetY));
    }

    private Point moveByOffset(Point current, Point offset) {
        if (offset.equals(Point.ZERO)) {
            return current;
        }
        Point desired = current.add(offset);
        int desiredX = desired.getX();
        int desiredY = desired.getY();
        int actualX = current.getX();
        int actualY = current.getY();

        if (isValidX(desiredX) && isValidY(desiredY)) {
            String str = getValueAtPosition(desired);
            if (str != null) {
                actualX = desiredX;
                actualY = desiredY;
            }
        }

        return new Point(actualX, actualY);
    }

    private boolean isValidX(int x) {
        return x >= 0 && x < grid.get(0).size();
    }

    private boolean isValidY(int y) {
        return y >= 0 && y < grid.size();
    }
}
