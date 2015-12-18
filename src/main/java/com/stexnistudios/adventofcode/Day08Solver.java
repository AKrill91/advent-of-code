package com.stexnistudios.adventofcode;

import java.io.IOException;
import java.util.Arrays;
import java.util.List;

public class Day08Solver extends Solver {

    private int codeCount;
    private int literalCount;
    private int encodedCount;

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

        logger.info(
            "Code Length: {}, Literal length: {}, Diff: {}",
            codeCount,
            literalCount,
            codeCount - literalCount
        );

        logger.info(
            "Code Length: {}, Encoded Length: {}, Diff: {}",
            codeCount,
            encodedCount,
            encodedCount - codeCount
        );
    }

    private void analyzeString(String str) {
        codeCount += str.length();
        StringBuilder literal = new StringBuilder();
        StringBuilder encoded = new StringBuilder("\"");

        boolean inEscaped = false;
        String hexSequence = "";

        for(int i = 0; i < str.length(); ++i) {
            char c = str.charAt(i);

            if(c == '"') {
                if(i != 0 && i != str.length() -1) {
                    literal.append(c);
                }
                encoded.append('\\');
                if(inEscaped) {
                    inEscaped = false;
                }
            } else if (c == '\\') {
                if(inEscaped) {
                    literal.append(c);
                }
                inEscaped = !inEscaped;
                encoded.append(c);
            } else if (c == 'x') {
                if(!inEscaped) {
                    literal.append(c);
                } else {
                    hexSequence = str.charAt(++i) + "" + str.charAt(++i);
                    literal.append(
                        (char) Integer.parseInt(hexSequence, 16)
                    );
                    inEscaped = false;
                }
            } else {
                literal.append(c);
            }

            encoded.append(c);

            if(!hexSequence.equals("")) {
                encoded.append(hexSequence);
                hexSequence = "";
            }
        }

        encoded.append('"');

        literalCount += literal.length();
        encodedCount += encoded.length();
    }

    public int getCodeCount() {
        return codeCount;
    }

    public int getLiteralCount() {
        return literalCount;
    }

    public int getEncodedCount() {
        return encodedCount;
    }
}
