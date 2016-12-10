package com.stexnistudios.adventofcode.day07;

public class Day07aSolver extends Day07Solver {
    public Day07aSolver(String input) {
        super(input);
    }

    protected boolean isInputValid(String input) {
        boolean found = false;
        boolean inBracket = false;
        for (int i = 1; i < input.length() - 2; ++i) {
            char second = input.charAt(i);
            if(second == '[') {
                inBracket = true;
                continue;
            } else if(second == ']') {
                inBracket = false;
                continue;
            }

            char first = input.charAt(i - 1);
            if(first == second) {
                continue;
            }
            char third = input.charAt(i + 1);
            char fourth = input.charAt(i + 2);

            if (first == fourth && second == third) {
                if(inBracket) {
                    found = false;
                    break;
                } else  {
                    found = true;
                }
            }
        }

        return found;
    }
}
