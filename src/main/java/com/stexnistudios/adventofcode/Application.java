package com.stexnistudios.adventofcode;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.io.InputStream;
import java.util.Scanner;

public class Application implements Runnable {
    public static void main(String[] args) throws IOException {
        InputStream inStream = Application.class
            .getResourceAsStream(
                "/day01.txt"
            );

        String input = new Scanner(inStream).useDelimiter("\\A").next();

        Solver solver = new Day01Solver(input);

        new Application(solver).run();
    }

    private final Solver solver;

    private final Logger logger;

    public Application(Solver solver) {
        logger = LoggerFactory.getLogger(getClass());

        this.solver = solver;
    }


    @Override
    public void run() {
        try {
            Object output = solver.call();
            logger.info("Output: {}", output);
        } catch (Exception e) {
            logger.error("Error solving problem: {}", e.getMessage(), e);
        }
    }
}
