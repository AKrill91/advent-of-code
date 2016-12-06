package com.stexnistudios.adventofcode;

import java.util.Arrays;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class Day06aSolver extends Solver {
    public Day06aSolver(String input) {
        super(input);
    }

    @Override
    public String call() {
        String msg = "";
        List<String> messages = Arrays.asList(getInput().split("\n"));

        Map<Integer, Map<Character, Integer>> indexCharacterCounts = new HashMap<>();

        for (String message : messages) {
            for (int i = 0; i < message.length(); ++i) {
                if (!indexCharacterCounts.containsKey(i)) {
                    indexCharacterCounts.put(i, new HashMap<>());
                }

                Map<Character, Integer> characterCounts = indexCharacterCounts.get(
                    i);

                Character c = message.charAt(i);

                if (characterCounts.containsKey(c)) {
                    characterCounts.put(c, characterCounts.get(c) + 1);
                } else {
                    characterCounts.put(c, 1);
                }
            }
        }

        for (int i = 0; i < indexCharacterCounts.size(); ++i) {
            Map<Character, Integer> characterCounts = indexCharacterCounts.get(i);

            Map.Entry<Character, Integer> max = null;

            for (Map.Entry<Character, Integer> entry : characterCounts.entrySet()) {
                if (
                    max == null ||
                        entry.getValue().compareTo(max.getValue()) > 0
                    ) {
                    max = entry;
                }
            }

            msg += max.getKey();
        }

        return msg;
    }
}
