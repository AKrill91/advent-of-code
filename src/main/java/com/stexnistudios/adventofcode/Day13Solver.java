package com.stexnistudios.adventofcode;

import com.google.common.collect.Collections2;

import java.io.IOException;
import java.util.*;

/**
 * Created by akrill on 12/21/15.
 */
public class Day13Solver extends Solver {

    static class Arrangement {
        final String left;
        final String right;

        public Arrangement(String left, String right) {
            this.left = left;
            this.right = right;
        }

        @Override
        public boolean equals(Object o) {
            if (this == o) {
                return true;
            }
            if (o == null || getClass() != o.getClass()) {
                return false;
            }
            Arrangement that = (Arrangement) o;
            return Objects.equals(left, that.left) &&
                Objects.equals(right, that.right);
        }

        @Override
        public int hashCode() {
            return Objects.hash(left, right);
        }
    }

    Collection<String> people;
    Map<Arrangement, Integer> happinessMap;

    int minHappiness = Integer.MAX_VALUE;
    int maxHappiness = Integer.MIN_VALUE;

    List<String> saddestArrangement;
    List<String> happiestArrangement;

    public Day13Solver(List<String> input) {
        super(input);
        people = new HashSet<>();
        happinessMap = new HashMap<>();
    }

    public Day13Solver(String fileName) throws IOException {
        this(loadInputFromFile(fileName));
    }

    @Override
    public void run() {
        parseInput();

        Collections2.permutations(people)
            .parallelStream()
            .forEach(order -> {
                int happiness = 0;

                for (int i = 0; i < order.size(); ++i) {
                    String current = order.get(i);
                    String next = order.get(i == order.size() - 1 ? 0 : i + 1);

                    happiness += happinessMap.get(new Arrangement(
                        current,
                        next
                    ));

                    happiness += happinessMap.get(
                        new Arrangement(next, current)
                    );
                }

                if (happiness > maxHappiness) {
                    maxHappiness = happiness;
                    happiestArrangement = order;
                } else if (happiness < minHappiness) {
                    minHappiness = happiness;
                    saddestArrangement = order;
                }
            });

        logger.info(
            "The happiest arrangement is {} with a happiness of {}",
            String.join(", ", happiestArrangement),
            maxHappiness
        );

        logger.info(
            "The saddest arrangement is {} with a happiness of {}",
            String.join(", ", saddestArrangement),
            minHappiness
        );
    }

    public void parseInput() {
        input.forEach(str -> {
            String[] parts = str.split(" ");

            String left = parts[0];
            String right = parts[parts.length - 1];
            right = right.substring(0, right.length() - 1);

            int sign = parts[2].equals("gain") ? 1 : -1;
            int amount = sign * Integer.parseInt(parts[3]);

            people.add(left);
            people.add(right);
            happinessMap.put(new Arrangement(left, right), amount);
        });
    }

    public int getMinHappiness() {
        return minHappiness;
    }

    public int getMaxHappiness() {
        return maxHappiness;
    }

    public List<String> getSaddestArrangement() {
        return saddestArrangement;
    }

    public List<String> getHappiestArrangement() {
        return happiestArrangement;
    }
}
