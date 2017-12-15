package com.stexnistudios.adventofcode.year2017.day11;

import com.stexnistudios.adventofcode.Solver;

public class Day11Solver extends Solver {
    public Day11Solver(String input) {
        super(input);
    }

    @Override
    public Integer solveA() {
        String[] instructions = getInput().split(",");
        HexTile position = HexTile.ORIGIN;

        for (String instruction : instructions) {
            position = getNewPosition(position, instruction);
        }

        return position.distanceFromOrigin();
    }

    @Override
    public Object solveB() {
        String[] instructions = getInput().split(",");
        HexTile position = HexTile.ORIGIN;
        int maxDistance = 0;

        for (String instruction : instructions) {
            position = getNewPosition(position, instruction);
            maxDistance = Math.max(maxDistance, position.distanceFromOrigin());
        }

        return maxDistance;
    }

    private HexTile getNewPosition(HexTile position, String instruction) {
        HexTile output = HexTile.ORIGIN;

        switch (instruction) {
            case "n":
                output = new HexTile(position.q, position.r + 1, position.s - 1);
                break;
            case "ne":
                output = new HexTile(position.q + 1, position.r, position.s - 1);
                break;
            case "se":
                output = new HexTile(position.q + 1, position.r - 1, position.s);
                break;
            case "s":
                output = new HexTile(position.q, position.r - 1, position.s + 1);
                break;
            case "sw":
                output = new HexTile(position.q - 1, position.r, position.s + 1);
                break;
            case "nw":
                output = new HexTile(position.q - 1, position.r + 1, position.s);
                break;
        }

        return output;
    }

    private static class HexTile {
        private static final HexTile ORIGIN = new HexTile(0, 0, 0);

        private final int q;
        private final int r;
        private final int s;

        private HexTile(int q, int r, int s) {
            this.q = q;
            this.r = r;
            this.s = s;

            assert 0 == q + r + s;
        }

        public int distanceFromOrigin() {
            return distance(this, ORIGIN);
        }

        public static int distance(HexTile start, HexTile end) {
            return Math.max(
                Math.max(
                    Math.abs(start.q - end.q),
                    Math.abs(start.r - end.r)
                ),
                Math.abs(start.s - end.s)
            );
        }
    }
}
