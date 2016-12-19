package com.stexnistudios.adventofcode.day18;

import com.stexnistudios.adventofcode.Solver;

import java.util.ArrayList;
import java.util.List;

public class Day18aSolver extends Solver {
    private final List<List<Boolean>> tiles;
    private final int numRows;

    public Day18aSolver(String input) {
        this(input, 40);
    }

    public Day18aSolver(String input, int numRows) {
        super(input);
        this.numRows = numRows;
        tiles = new ArrayList<>(numRows);

        for (int i = 0; i < numRows; ++i) {
            tiles.add(new ArrayList<>());
        }
    }

    @Override
    public Integer call() {
        List<Boolean> firstRow = tiles.get(0);
        for (char c : getInput().trim().toCharArray()) {
            firstRow.add(c == '.');
        }

        for (int i = 1; i < tiles.size(); ++i) {
            List<Boolean> row = tiles.get(i);
            for (int j = 0; j < firstRow.size(); ++j) {
                row.add(isSafe(i, j));
            }
        }

        return (int) tiles.stream()
            .mapToLong(
                list -> list.stream()
                    .filter(Boolean::booleanValue)
                    .count()
            )
            .sum();
    }

    private boolean isSafe(int row, int column) {
        Boolean left = getLeft(row - 1, column);
        Boolean center = getCenter(row - 1, column);
        Boolean right = getRight(row - 1, column);

        boolean isTrap = (!left && !center && right) ||
            (left && !center && !right) ||
            (!left && center && right) ||
            (left && center && !right);

        return !isTrap;
    }

    private boolean getLeft(int row, int column) {
        return getTile(row, column - 1);
    }

    private boolean getCenter(int row, int column) {
        return getTile(row, column);
    }

    private boolean getRight(int row, int column) {
        return getTile(row, column + 1);
    }

    private boolean getTile(int row, int column) {
        List<Boolean> rowTiles = tiles.get(row);
        boolean output = true;
        if (column >= 0 && column < rowTiles.size()) {
            output = rowTiles.get(column);
        }

        return output;
    }
}
