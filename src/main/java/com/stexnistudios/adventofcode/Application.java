package com.stexnistudios.adventofcode;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.InputStream;
import java.lang.reflect.Constructor;
import java.time.LocalDate;
import java.util.Scanner;

public class Application {
    public static void main(String[] args) throws Exception {
        Logger logger = LoggerFactory.getLogger(Application.class);

        if (args.length == 0) {
            logger.error("No day argument provided");
            System.exit(1);
        }

        String year = String.valueOf(LocalDate.now().getYear());
        String day;

        if (args.length == 1) {
            day = args[0];
        } else {
            year = args[0];
            day = args[1];
        }

        InputStream inStream = Application.class
            .getResourceAsStream(
                String.format("/year%s/day%s.txt", year, day)
            );

        String input = new Scanner(inStream).useDelimiter("\\A").next();

        String className = String.format(
            "com.stexnistudios.adventofcode.year%s.day%s.Day%sSolver",
            year,
            day,
            day
        );

        Class<?> solverClass = Class.forName(className);
        Constructor<?> constructor = solverClass.getConstructor(String.class);
        Object instance = constructor.newInstance(input);

        if (!(instance instanceof Solver)) {
            logger.error("Class is not an instance of solver");
            System.exit(2);
        }

        Solver solver = (Solver) instance;

        Object resultA = solver.solveA();
        Object resultB = solver.solveB();

        logger.info("Result A is {}", resultA);
        logger.info("Result B is {}", resultB);
    }
}
