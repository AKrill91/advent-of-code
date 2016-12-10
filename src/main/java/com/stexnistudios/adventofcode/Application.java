package com.stexnistudios.adventofcode;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.InputStream;
import java.lang.reflect.Constructor;
import java.util.Scanner;

public class Application implements Runnable {
    public static void main(String[] args) throws Exception {
        Logger logger = LoggerFactory.getLogger(Application.class);

        if (args.length == 0) {
            logger.error("No day argument provided");
            System.exit(1);
        }


        String day = args[0];

        InputStream inStream = Application.class
            .getResourceAsStream(
                "/day" + day.substring(0, day.length() - 1) + ".txt"
            );

        String input = new Scanner(inStream).useDelimiter("\\A").next();

        String className = String.format(
            "com.stexnistudios.adventofcode.day%s.Day%sSolver",
            day.substring(0, day.length() - 1),
            day
        );


        Class<?> solverClass = Class.forName(className);
        Constructor<?> constructor = solverClass.getConstructor(String.class);
        Object instance = constructor.newInstance(input);

        if (!(instance instanceof Solver)) {
            logger.error("Class is not an instance of solver");
            System.exit(2);
        }

        new Application((Solver) instance).run();
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
