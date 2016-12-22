package com.stexnistudios.adventofcode.day22;

import com.stexnistudios.adventofcode.Solver;
import com.stexnistudios.adventofcode.util.Point;

import java.util.ArrayList;
import java.util.List;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

public class Day22aSolver extends Solver {

    private final List<Node> nodes;

    public Day22aSolver(String input) {
        super(input);

        nodes = new ArrayList<>();
    }

    @Override
    public Integer call() {
        Pattern pattern = Pattern.compile(
            "\\/dev\\/grid\\/node-x(\\d+)-y(\\d+)\\s+(\\d+)T\\s+(\\d+)T\\s+\\d+T"
        );

        for (String line : getInput().split("\n")) {
            Matcher matcher = pattern.matcher(line);

            if (matcher.find()) {
                int x = Integer.parseInt(matcher.group(1));
                int y = Integer.parseInt(matcher.group(2));
                int size = Integer.parseInt(matcher.group(3));
                int used = Integer.parseInt(matcher.group(4));

                nodes.add(
                    new Node(
                        new Point(x, y),
                        size,
                        used
                    )
                );

            } else {
                logger.error("Unable to parse node from input: {}", line);
            }
        }

        return nodes.stream()
            .mapToInt(node -> {
                int out = 0;
                if (node.getUsed() != 0) {
                    out = (int) nodes.stream()
                        .filter(
                            other ->
                                other != node && other.getAvail() >= node.getUsed()
                        )
                        .count();
                }

                return out;
            })
            .sum();
    }
}
