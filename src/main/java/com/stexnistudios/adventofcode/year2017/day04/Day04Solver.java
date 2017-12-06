package com.stexnistudios.adventofcode.year2017.day04;

import com.stexnistudios.adventofcode.Solver;

import java.util.Arrays;
import java.util.Collection;
import java.util.HashSet;
import java.util.Set;

public class Day04Solver extends Solver {

    private Collection<String> lines;

    public Day04Solver(String input) {
        super(input);

        lines = Arrays.asList(input.split("\n"));
    }

    @Override
    public Object solveA() {
        int validCount = 0;
        for (String phrase : lines) {
            String[] words = phrase.split(" ");
            Set<String> uniques = new HashSet<>(Arrays.asList(words));
            if (uniques.size() == words.length) {
                ++validCount;
            }
        }

        return validCount;
    }

    @Override
    public Object solveB() {
        int validCount = 0;
        for (String phrase : lines) {
            String[] words = phrase.split(" ");
            Set<String> uniques = new HashSet<>(Arrays.asList(words));

            if (uniques.size() == words.length) {
                Set<Integer> hashes = new HashSet<>();

                for (String word : words) {
                    char[] letters = word.toCharArray();
                    Arrays.sort(letters);
                    hashes.add(Arrays.hashCode(letters));
                }

                if (hashes.size() == words.length) {
                    ++validCount;
                }
            }
        }

        return validCount;
    }
}
