package com.stexnistudios.adventofcode.day08;

import com.stexnistudios.adventofcode.Solver;

import java.util.Arrays;
import java.util.List;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

public class Day08aSolver extends Solver {

    private final boolean[][] grid;

    public Day08aSolver(String input) {
        this(input, 50, 6);
    }

    public Day08aSolver(String input, int width, int height) {
        super(input);
        grid = new boolean[height][width];
    }

    @Override
    public Integer call() throws Exception {
        List<String> instructions = Arrays.asList(getInput().split("\n"));

        instructions.forEach(this::handleInstruction);

        logger.info(gridToString());

        int activeCount = 0;
        for (int y = 0; y < grid.length; ++y) {
            for (int x = 0; x < grid[y].length; ++x) {
                activeCount += grid[y][x] ? 1 : 0;
            }
        }

        return activeCount;
    }

    private void handleInstruction(String instruction) {
        boolean valid = handleRect(instruction) ||
            handleRow(instruction) ||
            handleColumn(instruction);

        if (!valid) {
            logger.error("Invalid instruction found: {}", instruction);
        }
    }

    private boolean handleRect(String instruction) {
        Pattern pattern = Pattern.compile("^rect (\\d+)x(\\d+)$");
        Matcher matcher = pattern.matcher(instruction);
        if (!matcher.matches()) {
            return false;
        }

        int width = Integer.parseInt(matcher.group(1));
        int height = Integer.parseInt(matcher.group(2));

        for (int y = 0; y < height; ++y) {
            for (int x = 0; x < width; ++x) {
                grid[y][x] = true;
            }
        }

        return true;
    }

    private boolean handleRow(String instruction) {
        Pattern pattern = Pattern.compile("^rotate row y=(\\d+) by (\\d+)$");
        Matcher matcher = pattern.matcher(instruction);
        if (!matcher.matches()) {
            return false;
        }

        int row = Integer.parseInt(matcher.group(1));
        int amount = Integer.parseInt(matcher.group(2));
        int rowLen = grid[row].length;
        boolean[] newRow = new boolean[rowLen];

        for (int x = 0; x < rowLen; ++x) {
            newRow[x] = grid[row][(rowLen + x - amount) % rowLen];
        }

        grid[row] = newRow;

        return true;
    }

    private boolean handleColumn(String instruction) {
        Pattern pattern = Pattern.compile("^rotate column x=(\\d+) by (\\d+)$");
        Matcher matcher = pattern.matcher(instruction);
        if (!matcher.matches()) {
            return false;
        }

        int column = Integer.parseInt(matcher.group(1));
        int amount = Integer.parseInt(matcher.group(2));
        int columnLen = grid.length;
        boolean[] newColumn = new boolean[columnLen];

        for (int i = 0; i < columnLen; ++i) {
            try {
                newColumn[i] = grid[(columnLen + i - amount) % columnLen][column];
            } catch (ArrayIndexOutOfBoundsException e) {
                logger.error("{}", e.getMessage(), e);
            }
        }

        for (int y = 0; y < columnLen; ++y) {
            grid[y][column] = newColumn[y];
        }

        return true;
    }

    private String gridToString() {
        StringBuilder sb = new StringBuilder("\n");

        for (int y = 0; y < grid.length; ++y) {
            for (int x = 0; x < grid[y].length; ++x) {
                sb.append(grid[y][x] ? '#' : '-');
            }
            sb.append('\n');
        }

        return sb.toString();
    }
}
