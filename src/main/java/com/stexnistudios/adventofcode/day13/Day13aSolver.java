package com.stexnistudios.adventofcode.day13;

import com.stexnistudios.adventofcode.Solver;
import com.stexnistudios.adventofcode.util.Point;

import java.util.*;
import java.util.stream.Collectors;

public class Day13aSolver extends Solver {
    private final Point start;
    private final Point end;
    private final HashMap<Point, Boolean> cache;
    private final Set<Point> explored;
    private final HashMap<Point, Integer> scores;

    public Day13aSolver(String input) {
        this(input, new Point(1, 1), new Point(31, 39));
    }

    public Day13aSolver(String input, Point start, Point end) {
        super(input);

        this.start = start;
        this.end = end;

        cache = new HashMap<>();
        explored = new HashSet<>();
        scores = new HashMap<>();
    }

    @Override
    public Integer call() {
        int favorite = Integer.parseInt(getInput());
        setScore(start, 0);
        int score = 1;
        Set<Point> explore = getNeighbors(start, favorite);
        Set<Point> nextSet;

        while (!scores.containsKey(end)) {
            nextSet = new HashSet<>(explore.size() * 3);
            for (Point point : explore) {
                setScore(point, score);
                nextSet.addAll(getNeighbors(point, favorite));
            }

            explore = nextSet;
            ++score;
        }

        return scores.get(end);
    }

    private void setScore(Point point, int score) {
        scores.put(point, score);
        explored.add(point);
    }

    private Set<Point> getNeighbors(Point current, int favorite) {
        long x = current.getX();
        long y = current.getY();
        List<Point> neighbors = new ArrayList<>(4);

        //North
        if (y - 1 >= 0) {
            neighbors.add(new Point(x, y - 1));
        }

        //East
        neighbors.add(new Point(x + 1, y));

        //South
        neighbors.add(new Point(x, y + 1));

        //West
        if (x - 1 >= 0) {
            neighbors.add(new Point(x - 1, y));
        }

        return neighbors.stream()
            .filter(point -> !explored.contains(point))
            .filter(point -> !isWall(point, favorite))
            .collect(Collectors.toSet());
    }

    private boolean isWall(Point point, int favorite) {
        if (cache.containsKey(point)) {
            return cache.get(point);
        }

        long x = point.getX();
        long y = point.getY();
        boolean isWall = true;

        if (x >= 0 && y >= 0) {
            long sum = x * x + 3 * x + 2 * x * y + y + y * y + favorite;

            isWall = bitCount(sum) % 2 == 1;
        }

        return isWall;
    }

    private int bitCount(long num) {
        int counter = 0;
        while (num > 0) {
            ++counter;
            num = num & (num - 1);
        }

        return counter;
    }
}
