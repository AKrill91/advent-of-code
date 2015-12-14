package com.stexnistudios.adventofcode;

import java.io.IOException;
import java.util.List;

public class Day08Solver extends Solver {

    private int codeCount = 0;
    private int literalCount = 0;

    public Day08Solver(List<String> input) {
        super(input);
    }

    public Day08Solver(String fileName) throws IOException {
        super(fileName);
    }

    @Override
    public void run() {

    }

    public int getCodeCount() {
        return codeCount;
    }

    public int getLiteralCount() {
        return literalCount;
    }
}
