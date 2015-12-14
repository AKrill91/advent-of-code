package com.stexnistudios.adventofcode;

import com.google.common.base.Strings;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;

public class Application {
    public static void main(String[] args) throws IOException {
        Logger logger = LoggerFactory.getLogger(Application.class);

        int day = 0;
        if (args.length > 0) {
            day = Integer.parseInt(args[0]);
        }

        String dayStr = Strings.padStart(String.valueOf(day), 2, '0');
        Runnable solver = null;
        String fileName = "day" + dayStr + ".txt";

        switch (day) {
            case 1:
                solver = new Day01Solver(fileName);
                break;
            case 2:
                solver = new Day02Solver(fileName);
                break;
            case 3:
                solver = new MultiSolver(
                    new Day03Solver(fileName),
                    new Day03Solver(fileName, 2)
                );
                break;
            case 6:
                solver = new Day06Solver(fileName);
                break;
            case 7:
                solver = new Day07Solver(fileName);
                break;
            default:
                logger.info("No solver found for day {}", dayStr);
                break;
        }

        if (solver != null) {
            logger.info("Running solver for day {}", dayStr);
            solver.run();
        }
    }
}
