package com.stexnistudios.adventofcode;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.util.ArrayList;
import java.util.Collections;
import java.util.List;

public abstract class Solver implements Runnable {

    protected final List<String> input;
    protected final Logger logger;

    public Solver(List<String> input) {
        this.input = Collections.unmodifiableList(input);
        this.logger = LoggerFactory.getLogger(this.getClass());
    }

    public Solver(String fileName) throws IOException {
        this(loadInputFromFile(fileName));
    }

    protected static List<String> loadInputFromFile(String fileName) throws IOException {
        List<String> lines = new ArrayList<>();

        try (
            BufferedReader reader = new BufferedReader(
                new InputStreamReader(
                    Thread.currentThread()
                        .getContextClassLoader()
                        .getResourceAsStream(fileName)
                )
            )
        ) {
            String line = "";

            while ((line = reader.readLine()) != null) {
                lines.add(line);
            }
        }

        return lines;
    }
}
