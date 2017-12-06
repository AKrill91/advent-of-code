package com.stexnistudios.adventofcode;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.concurrent.Callable;

public abstract class Solver {

    private final String input;

    protected final Logger logger;

    public Solver(String input) {
        logger = LoggerFactory.getLogger(getClass());
        this.input = input;
    }

    public String getInput() {
        return input;
    }

    public abstract Object solveA();
    public abstract Object solveB();
}
