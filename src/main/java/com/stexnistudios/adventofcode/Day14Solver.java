package com.stexnistudios.adventofcode;

import java.io.IOException;
import java.util.ArrayList;
import java.util.Collection;
import java.util.List;
import java.util.stream.IntStream;

/**
 * Created by akrill on 12/23/15.
 */
public class Day14Solver extends Solver {

    static class Reindeer {
        public final String name;
        public final int velocity;
        public final int duration;
        public final int rest;

        private int distance;
        private int time;
        private boolean isResting;

        private int points;

        public Reindeer(String name, int velocity, int duration, int rest) {
            this.name = name;
            this.velocity = velocity;
            this.duration = duration;
            this.rest = rest;
            distance = 0;
            time = 0;
            isResting = false;
            points = 0;
        }

        public void tick() {
            ++time;
            if (!isResting) {
                distance += velocity;
                if (time >= duration) {
                    isResting = true;
                    time = 0;
                }
            } else {
                if (time >= rest) {
                    isResting = false;
                    time = 0;
                }
            }
        }

        public int getDistance() {
            return distance;
        }

        public int getPoints() {
            return points;
        }

        public void givePoint() {
            ++points;
        }
    }

    Collection<Reindeer> reindeers;
    int maxDistance;

    public Day14Solver(List<String> input) {
        super(input);
        reindeers = new ArrayList<>();
        maxDistance = Integer.MIN_VALUE;
    }

    public Day14Solver(String fileName) throws IOException {
        this(loadInputFromFile(fileName));
    }

    @Override
    public void run() {
        parseInput();

        IntStream.range(0, 2503)
            .forEach(num -> {
                reindeers.forEach(Reindeer::tick);
                int maxDistance = reindeers.stream()
                    .mapToInt(Reindeer::getDistance)
                    .max()
                    .getAsInt();

                reindeers.stream()
                    .filter(reindeer -> reindeer.getDistance() == maxDistance)
                    .findFirst()
                    .get()
                    .givePoint();
            });

        int maxDistance = reindeers.stream()
            .mapToInt(Reindeer::getDistance)
            .max()
            .getAsInt();

        int maxPoints = reindeers.stream()
            .mapToInt(Reindeer::getPoints)
            .max()
            .getAsInt();

        Reindeer winner = reindeers.stream()
            .filter(reindeer -> reindeer.getPoints() == maxPoints)
            .findFirst()
            .get();

        Reindeer farthest = reindeers.stream()
            .filter(reindeer -> reindeer.getDistance() == maxDistance)
            .findFirst()
            .get();

        logger.info(
            "The reindeer with the most points is {} with {}.",
            winner.name,
            winner.getPoints()
        );

        logger.info(
            "The reindeer that went the farthest is {} with a distance of {} km.",
            farthest.name,
            farthest.getDistance()
        );
    }

    public void parseInput() {
        input.forEach(str -> {
            String[] parts = str.split(" ");

            reindeers.add(
                new Reindeer(
                    parts[0],
                    Integer.parseInt(parts[3]),
                    Integer.parseInt(parts[6]),
                    Integer.parseInt(parts[13])
                )
            );
        });
    }

    public int getMaxDistance() {
        return maxDistance;
    }
}
