package com.stexnistudios.adventofcode.day17;

import com.stexnistudios.adventofcode.Solver;
import com.stexnistudios.adventofcode.util.Point;

import javax.xml.bind.DatatypeConverter;
import java.security.MessageDigest;
import java.util.HashMap;
import java.util.Map;

public class Day17aSolver extends Solver {

    private final MessageDigest digest;

    public Day17aSolver(String input) throws Exception {
        super(input);

        digest = MessageDigest.getInstance("MD5");
    }

    @Override
    public String call() {
        Point start = new Point();
        Point end = new Point(3, 3);

        Map<String, Point> currentSteps = new HashMap<>();
        Map<String, Point> nextSteps = new HashMap<>();
        currentSteps.put("", start);

        while (true) {
            for (Map.Entry<String, Point> entry : currentSteps.entrySet()) {
                String directions = entry.getKey();
                Point pos = entry.getValue();

                if (pos.equals(end)) {
                    return directions;
                }

                Map<String, Point> neighbors = getNeighbors(pos, directions, getHash(getInput() + directions));
                nextSteps.putAll(neighbors);
            }

            currentSteps = nextSteps;
            nextSteps = new HashMap<>();
        }
    }

    private String getHash(String in) {
        return DatatypeConverter.printHexBinary(digest.digest(in.getBytes()))
            .substring(0, 4);
    }

    private Map<String, Point> getNeighbors(Point p, String directions, String hash) {
        boolean canGoUp = p.getY() > 0 && hash.charAt(0) > 'A';
        boolean canGoDown = p.getY() < 3 && hash.charAt(1) > 'A';
        boolean canGoLeft = p.getX() > 0 && hash.charAt(2) > 'A';
        boolean canGoRight = p.getX() < 3 && hash.charAt(3) > 'A';

        Map<String, Point> neighbors = new HashMap<>();

        if (canGoUp) {
            neighbors.put(directions + "U", new Point(p.getX(), p.getY() - 1));
        }

        if (canGoDown) {
            neighbors.put(directions + "D", new Point(p.getX(), p.getY() + 1));
        }

        if (canGoLeft) {
            neighbors.put(directions + "L", new Point(p.getX() - 1, p.getY()));
        }

        if (canGoRight) {
            neighbors.put(directions + "R", new Point(p.getX() + 1, p.getY()));
        }

        return neighbors;
    }
}
