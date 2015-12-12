package com.stexnistudios.adventofcode;

import java.io.IOException;
import java.util.ArrayList;
import java.util.Collections;
import java.util.IntSummaryStatistics;
import java.util.List;
import java.util.stream.IntStream;
import java.util.stream.StreamSupport;

/**
 * Created by Austin Krill on 12/11/2015.
 */
public class Day02Solver extends Solver {

    private static class Box {
        public final int length;
        public final int width;
        public final int height;
        public final int area;
        public final int slack;
        public final int ribbonLength;

        public Box(int l, int w, int h) {
            length = l;
            width = w;
            height = h;

            List<Integer> sides = new ArrayList<>();
            sides.add(length);
            sides.add(width);
            sides.add(height);
            Collections.sort(sides);
            slack = sides.get(0) * sides.get(1);
            area = 2 * slack + 2 * sides.get(0) * sides.get(2) + 2 * sides.get(1) * sides.get(2);
            ribbonLength = length * width * height + 2 * sides.get(0) + 2 * sides.get(1);
        }

        public static Box fromString(String str) {
            String[] parts = str.split("x");

            return new Box(
                Integer.parseInt(parts[0]),
                Integer.parseInt(parts[1]),
                Integer.parseInt(parts[2])
            );
        }
    }

    public Day02Solver(String fileName) throws IOException {
        super(fileName);
    }

    public Day02Solver(List<String> input) {
        super(input);
    }

    @Override
    public void run() {
        int totalArea = 0;
        int totalRibbon = 0;

        for (String boxStr : input) {
            Box b = Box.fromString(boxStr);

            totalArea += b.area + b.slack;
            totalRibbon += b.ribbonLength;
        }

        logger.info(
            "Need a total area of {}, and ribbon length of {}",
            totalArea,
            totalRibbon
        );
    }
}
