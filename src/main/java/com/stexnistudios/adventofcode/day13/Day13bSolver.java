package com.stexnistudios.adventofcode.day13;

import com.stexnistudios.adventofcode.Solver;
import com.stexnistudios.adventofcode.util.Point;

import java.util.*;
import java.util.stream.Collectors;

public class Day13bSolver extends Day13aSolver {
    public Day13bSolver(String input) {
        this(input, new Point(1, 1), new Point(31, 39));
    }

    public Day13bSolver(String input, Point start, Point end) {
        super(input, start, end);
    }

    @Override
    public Integer call() {
        int favorite = Integer.parseInt(getInput());
        setScore(start, 0);
        int score = 1;
        Set<Point> explore = getNeighbors(start, favorite);
        Set<Point> nextSet;

        while (score < 51) {
            nextSet = new HashSet<>(explore.size() * 3);
            for (Point point : explore) {
                setScore(point, score);
                nextSet.addAll(getNeighbors(point, favorite));
            }

            explore = nextSet;
            ++score;
        }

        return explored.size();
    }
}
