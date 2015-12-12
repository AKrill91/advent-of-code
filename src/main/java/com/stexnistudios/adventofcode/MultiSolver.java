package com.stexnistudios.adventofcode;

import java.util.Arrays;
import java.util.List;

public class MultiSolver implements Runnable {
    private final List<Solver> children;

    public MultiSolver(Solver... solvers) {
        children = Arrays.asList(solvers);
    }

    @Override
    public void run() {
        children.parallelStream().forEach(Solver::run);
    }
}
