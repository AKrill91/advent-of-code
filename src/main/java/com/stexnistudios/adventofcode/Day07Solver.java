package com.stexnistudios.adventofcode;

import com.stexnistudios.adventofcode.gate.IGate;

import java.io.IOException;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class Day07Solver extends Solver {

    private Map<String, IGate> wires;

    public Day07Solver(List<String> input) {
        super(input);

        wires = new HashMap<>();
    }

    public Day07Solver(String fileName) throws IOException {
        this(loadInputFromFile(fileName));
    }

    @Override
    public void run() {
        logger.info("There are {} steps.", input.size());

        input.forEach(step -> {

        });
    }

    public Map<String, IGate> getWires() {
        return wires;
    }
}
