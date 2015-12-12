package com.stexnistudios.adventofcode;

import com.google.common.base.Strings;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;

public class Application {
    public static void main(String[] args) throws IOException {
        Logger logger = LoggerFactory.getLogger(Application.class);

        int day = Integer.parseInt(args[0]);
        String dayStr = Strings.padStart(String.valueOf(day), 2, '0');

        logger.info("Running solver for day {}", dayStr);

        Solver solver = null;

        switch (day) {
            case 1:
                solver = new Day01Solver(dayStr);
                break;
        }

        if(solver != null) {
            solver.run();
        }
    }
}
