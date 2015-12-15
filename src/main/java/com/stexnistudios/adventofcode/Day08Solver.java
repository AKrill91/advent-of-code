package com.stexnistudios.adventofcode;

import java.io.IOException;
import java.util.Arrays;
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
        input.stream()
            .forEach(this::analyzeString);

        logger.info("Code Length: {}, Literal length: {}, Diff: {}", codeCount, literalCount, codeCount - literalCount);
    }

    private void analyzeString(String str) {
        codeCount += str.length();
        StringBuilder literal = new StringBuilder();

        boolean inEscaped = false;
        String asciiSequence = "";

        for (char ch : Arrays.copyOfRange(str.toCharArray(), 1, str.length() - 1)) {
            if (inEscaped) {
                if(ch == '"' || ch == '\\') {
                    literal.append(ch);
                    inEscaped = false;
                } else if (ch != 'x') {
                    asciiSequence += ch;
                    if(asciiSequence.length() == 2) {
                        literal.append((char)Integer.parseInt(asciiSequence, 16));
                        asciiSequence = "";
                        inEscaped = false;
                    }
                }
            } else if(ch == '\\') {
                inEscaped = true;
            } else {
                literal.append(ch);
            }
        }

        literalCount += literal.length();
    }

    public int getCodeCount() {
        return codeCount;
    }

    public int getLiteralCount() {
        return literalCount;
    }
}
