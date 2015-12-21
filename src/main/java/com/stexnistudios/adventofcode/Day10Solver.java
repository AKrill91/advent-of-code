package com.stexnistudios.adventofcode;

import java.io.IOException;
import java.util.List;

public class Day10Solver extends Solver {

    private String result;

    public Day10Solver(List<String> input) {
        super(input);
    }

    public Day10Solver(String fileName) throws IOException {
        super(fileName);
    }

    @Override
    public void run() {
        String in = input.get(0);
        int times = Integer.parseInt(input.get(1));

        for (int i = 0; i < times; ++i) {
            in = process(in);
        }

        result = in;

        logger.info(
            "Result after {} times has a length of {}",
            times,
            result.length()
        );
    }

    private String process(String str) {
        StringBuilder sb = new StringBuilder();

        char repeat = str.charAt(0);
        str = str.substring(1) + " ";
        int times = 1;

        for (char next : str.toCharArray()) {
            if (next != repeat) {
                sb.append(times);
                sb.append(repeat);
                times = 1;
                repeat = next;
            } else {
                times += 1;
            }
        }

        return sb.toString();
    }

    public String getResult() {
        return result;
    }
}
