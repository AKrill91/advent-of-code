package com.stexnistudios.adventofcode;

import com.stexnistudios.adventofcode.util.Point;

import java.io.IOException;
import java.util.*;

public class Day06Solver extends Solver {

    private static class Light {
        private boolean isOn = false;
        private int brightness = 0;

        public void turnOn() {
            isOn = true;
            brightness++;
        }

        public void turnOff() {
            isOn = false;
            brightness = Integer.max(0, brightness - 1);
        }

        public void toggle() {
            isOn = !isOn;
            brightness += 2;
        }
    }

    private final int width = 1000;
    private final int height = 1000;
    private Map<Point, Light> lights;

    public Day06Solver(List<String> input) {
        super(input);
        lights = new HashMap<>();

        for(int y = 0; y < height; ++y) {
            for (int x = 0; x < width; ++x) {
                lights.put(new Point(x, y), new Light());
            }
        }
    }

    public Day06Solver(String fileName) throws IOException {
        this(loadInputFromFile(fileName));
    }

    @Override
    public void run() {

        for (String step : input) {
            Queue<String> components = new LinkedList<>(
                Arrays.asList(step.split(" "))
            );

            String instruction = components.poll();

            if (instruction.equals("turn")) {
                instruction = components.poll();
            }

            Point start = fromString(components.poll());
            components.poll();
            Point end = fromString(components.poll());

            processStep(instruction, start, end);
        }

        logger.info(
            "{} lights are on, with a combined brighness of {}",
            getNumOn(),
            getBrightness()
        );
    }

    private Point fromString(String str) {
        String[] parts = str.split(",");

        return new Point(
            Integer.parseInt(parts[0]),
            Integer.parseInt(parts[1])
        );
    }

    private void processStep(String instruction, Point start, Point end) {
        for(long y = start.y; y <= end.y; ++y) {
            for (long x = start.x; x <= end.x; ++x) {
                Light l =lights.get(new Point(x, y));

                switch (instruction) {
                    case "on":
                        l.turnOn();
                        break;
                    case "off":
                        l.turnOff();
                        break;
                    case "toggle":
                        l.toggle();
                        break;
                }
            }
        }
    }

    public long getNumOn() {
        return lights.values()
            .parallelStream()
            .filter(light -> light.isOn)
            .count();
    }

    public long getBrightness() {
        return lights.values()
            .parallelStream()
            .mapToInt(light -> light.brightness)
            .sum();
    }
}
