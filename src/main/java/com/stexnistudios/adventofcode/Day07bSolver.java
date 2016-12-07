package com.stexnistudios.adventofcode;

import org.javatuples.Pair;

import java.util.ArrayList;
import java.util.List;

public class Day07bSolver extends Day07Solver {
    public Day07bSolver(String input) {
        super(input);
    }

    protected boolean isInputValid(String input) {
        List<Pair<Character, Character>> abas = new ArrayList<>();
        List<Pair<Character, Character>> babs = new ArrayList<>();

        boolean inBracket = false;
        for (int i = 1; i < input.length() - 1; ++i) {
            char second = input.charAt(i);
            if (second == '[') {
                inBracket = true;
                continue;
            } else if (second == ']') {
                inBracket = false;
                continue;
            }

            char first = input.charAt(i - 1);
            char third = input.charAt(i + 1);

            if (first == third) {
                if (inBracket) {
                    babs.add(new Pair<>(first, second));
                } else {
                    abas.add(new Pair<>(first, second));
                }
            }
        }

        boolean found = false;

        for (Pair<Character, Character> pair : abas) {
            found = found || babs.contains(new Pair<>(pair.getValue1(), pair.getValue0()));
        }

        return found;
    }
}
