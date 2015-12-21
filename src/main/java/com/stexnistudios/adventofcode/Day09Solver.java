package com.stexnistudios.adventofcode;

import com.google.common.collect.Collections2;

import java.io.IOException;
import java.util.*;

public class Day09Solver extends Solver {

    static class Route {
        final String start;
        final String end;

        public Route(String start, String end) {
            int ordering = start.compareTo(end);
            this.start = ordering < 0 ? start : end;
            this.end = ordering < 0 ? end : start;
        }

        @Override
        public boolean equals(Object o) {
            boolean eql = false;

            if (o instanceof Route) {
                Route other = (Route) o;
                eql = start.equals(other.start) && end.equals(other.end);
            }

            return eql;
        }

        @Override
        public int hashCode() {
            return Objects.hash(start, end);
        }
    }

    long shortestRoute = Long.MAX_VALUE;
    long longestRoute = Long.MIN_VALUE;

    Map<Route, Integer> routes;
    Collection<String> cities;
    Collection<String> shortest;
    Collection<String> longest;

    public Day09Solver(List<String> input) {
        super(input);
        routes = new HashMap<>();
        cities = new HashSet<>();
    }

    public Day09Solver(String fileName) throws IOException {
        this(loadInputFromFile(fileName));
    }

    @Override
    public void run() {
        parseInput();

        Collections2.permutations(cities)
            .parallelStream()
            .forEach(visits -> {
                int tripLength = 0;
                for (int i = 0; i < visits.size() - 1; ++i) {
                    tripLength += routes.get(new Route(
                        visits.get(i),
                        visits.get(i + 1)
                    ));
                }

                if (tripLength < shortestRoute) {
                    shortestRoute = tripLength;
                    shortest = visits;
                } else if (tripLength > longestRoute) {
                    longestRoute = tripLength;
                    longest = visits;
                }
            });

        logger.info(
            "Shortest route is {} with distance {}",
            String.join(", ", shortest),
            shortestRoute
        );

        logger.info(
            "Longest route is {} with distance {}",
            String.join(", ", longest),
            longestRoute
        );
    }

    private void parseInput() {
        input.forEach(route -> {
            String[] parts = route.split(" ");
            String firstCity = parts[0];
            String secondCity = parts[2];
            int distance = Integer.parseInt(parts[4]);

            cities.add(firstCity);
            cities.add(secondCity);

            routes.put(new Route(firstCity, secondCity), distance);
        });

        logger.info(
            "There are {} cities and {} routes.",
            cities.size(),
            routes.size()
        );
    }

    public long getShortestRoute() {
        return shortestRoute;
    }
}
