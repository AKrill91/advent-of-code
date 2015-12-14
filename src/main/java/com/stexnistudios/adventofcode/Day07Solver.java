package com.stexnistudios.adventofcode;

import java.io.IOException;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class Day07Solver extends Solver {

    public interface IGate {
        short getValue();
    }

    private static class ValueGate implements IGate {
        short value;

        public ValueGate(short value) {
            this.value = value;
        }

        @Override
        public short getValue() {
            return value;
        }
    }

    private abstract class UnaryGate implements IGate {
        protected String input;

        public UnaryGate(String input) {
            this.input = input;
        }

        public String getInput() {
            return input;
        }
    }

    private abstract class BinaryGate extends UnaryGate {
        protected String input2;

        public BinaryGate(String input1, String input2) {
            super(input1);
            this.input2 = input2;
        }

        public String getInput2() {
            return input2;
        }
    }

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
