package com.stexnistudios.adventofcode.year2017.day03;

import com.stexnistudios.adventofcode.Solver;
import com.stexnistudios.adventofcode.util.Point;

import java.util.Arrays;
import java.util.Collection;
import java.util.HashMap;
import java.util.Map;

public class Day03Solver extends Solver {
    public Day03Solver(String input) {
        super(input);
    }

    @Override
    public Long solveA() {
        int index = Integer.parseInt(getInput());

        return Point.manhattanDistance(Point.ZERO, getPosition(index));
    }

    @Override
    public Long solveB() {
        int goal = Integer.parseInt(getInput());
        Map<Point, Long> grid = new HashMap<>();

        int index = 2;
        long value = 1;
        grid.put(Point.ZERO, 1L);

        while (value <= goal) {
            Point p = getPosition(index);
            value = getNeighborSums(grid, p);
            grid.put(p, value);

            ++index;
        }

        return value;
    }

    private int getRing(int index) {
        int ring = 1;

        while (Math.pow((ring * 2) - 1, 2) < index) {
            ++ring;
        }

        return ring;
    }

    private Point getPosition(int index) {
        if (index == 1) {
            return Point.ZERO;
        }

        int ring = getRing(index);

        int start = (int) Math.pow(((ring - 1) * 2) - 1, 2); //exclusive
        int end = (int) Math.pow((ring * 2) - 1, 2); //inclusive
        int ringSize = end - start;
        int legLength = ringSize / 4;
        int quadrant = (index - (start + 1)) / legLength;
        int quadrantPos = (index - (start + 1)) % legLength;
        int xMult = quadrant % 3 == 0 ? 1 : -1;
        int yMult = quadrant >= 2 ? -1 : 1;
        Point quadrantEnd = new Point((ring - 1) * xMult, (ring - 1) * yMult);
        int xDir = quadrant % 2 == 1 ? -xMult * (legLength - (quadrantPos + 1)) : 0;
        int yDir = quadrant % 2 == 0 ? -yMult * (legLength - (quadrantPos + 1)) : 0;

        return new Point(quadrantEnd.getX() + xDir, quadrantEnd.getY() + yDir);
    }

    private long getNeighborSums(Map<Point, Long> grid, Point current) {
        return getNeighbors(current)
            .stream()
            .filter(grid::containsKey)
            .mapToLong(grid::get)
            .sum();
    }

    private Collection<Point> getNeighbors(Point p) {
        return Arrays.asList(
            new Point(p.getX() + 1, p.getY()), //E
            new Point(p.getX() + 1, p.getY() + 1), //NE
            new Point(p.getX(), p.getY() + 1), //N
            new Point(p.getX() - 1, p.getY() + 1), //NW
            new Point(p.getX() - 1, p.getY()), //W
            new Point(p.getX() - 1, p.getY() - 1), //SW
            new Point(p.getX(), p.getY() - 1), //S
            new Point(p.getX() + 1, p.getY() - 1) //SE
        );
    }
}
