package com.stexnistudios.adventofcode;

import java.io.IOException;
import java.util.*;

public class Day07Solver extends Solver {

    private Map<String, Character> wires;

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

        Queue<String> queue = new LinkedList<>(input);

        while (!queue.isEmpty()) {
            String instruction = queue.poll();
            if (!processInstruction(instruction)) {
                queue.add(instruction);
            }
        }

        List<String> sorted = new ArrayList<>(wires.keySet());
        Collections.sort(sorted);

        sorted.stream().forEachOrdered(key -> {
            logger.info("Key: {}, Value: {}", key, (int)wires.get(key));
        });
    }

    private boolean processInstruction(String instructionStr) {
        String[] parts = instructionStr.split(" ");
        String id = parts[parts.length - 1];

        //Subtract two for the -> and the id.
        int argLength = parts.length - 2;
        boolean success = false;

        Character val = null;

        switch (argLength) {
            case 1:
                val = parseParts(parts[0]);
                break;
            case 2:
                val = parseParts(parts[0], parts[1]);
                break;
            case 3:
                val = parseParts(parts[0], parts[1], parts[2]);
                break;
            default:
                throw new RuntimeException(
                    "Unable to parse instruction: " + instructionStr);
        }

        if (val != null) {
            wires.put(id, val);
            success = true;
        }

        return success;
    }

    private Character parseParts(String arg) {
        Character val = null;

        try {
            int intVal = Integer.parseInt(arg);
            val = (char) intVal;
        } catch (Exception e) {
            if(wires.containsKey(arg)) {
                val = wires.get(arg);
            }
        }

        return val;
    }

    private Character parseParts(String arg1, String arg2) {
        Character val = null;

        if (wires.containsKey(arg2)) {
            switch (arg1) {
                case "NOT":
                    val = (char)~wires.get(arg2);
                    break;
                default:
                    throw new RuntimeException("Unknown Operation " + arg1);
            }
        }

        return val;
    }

    private Character parseParts(String arg1, String arg2, String arg3) {
        Character val = null;

        Character left;
        Character right;

        try {
            left = (char) Integer.parseInt(arg1);
        } catch (Exception e) {
            left = wires.get(arg1);
        }

        try {
            right = (char) Integer.parseInt(arg3);
        } catch (Exception e2) {
            right = wires.get(arg3);
        }

        if (left != null && right != null) {
            switch (arg2) {
                case "AND":
                    val = (char)(left & right);
                    break;
                case "OR":
                    val = (char)(left | right);
                    break;
                case "LSHIFT":
                    val = (char)(left << right);
                    break;
                case "RSHIFT":
                    val = (char)(left >>> right);
                    break;
                default:
                    throw new RuntimeException("Unknown operation " + arg2);
            }
        }

        return val;
    }

    public Map<String, Character> getWires() {
        return wires;
    }
}
