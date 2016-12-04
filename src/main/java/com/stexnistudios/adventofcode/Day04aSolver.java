package com.stexnistudios.adventofcode;

import java.text.StringCharacterIterator;
import java.util.HashMap;
import java.util.Iterator;
import java.util.LinkedHashMap;
import java.util.Map;
import java.util.stream.Collectors;

public class Day04aSolver extends Solver {
    public Day04aSolver(String input) {
        super(input);
    }

    @Override
    public Integer call() {
        String[] lines = getInput().split("\n");
        int sectionIdSum = 0;

        for (String line : lines) {
            Map<Character, Integer> characterCounts = new HashMap<>();

            boolean inSectionId = false;
            boolean inChecksum = false;
            char checksumStart = '[';
            char checksumEnd = ']';
            String sectionId = "";
            String checksum = "";

            for (char c : line.trim().toCharArray()) {
                if (inSectionId) {
                    if (c == checksumStart) {
                        inChecksum = true;
                        inSectionId = false;
                    } else {
                        sectionId += c;
                    }
                } else if (inChecksum) {
                    if (c == checksumEnd) {
                        break;
                    }
                    checksum += c;
                } else {
                    if (Character.isDigit(c)) {
                        inSectionId = true;
                        sectionId += c;
                    } else if (c != '-') {
                        if (!characterCounts.containsKey(c)) {
                            characterCounts.put(c, 1);
                        } else {
                            characterCounts.put(c, characterCounts.get(c) + 1);
                        }
                    }
                }
            }

            if(isValidChecksum(checksum, characterCounts)) {
                sectionIdSum += Integer.parseInt(sectionId);
            }
        }

        return sectionIdSum;
    }

    private boolean isValidChecksum(String checksum, Map<Character, Integer> characterCounts) {
        Map<Character, Integer> orderedMap = characterCounts.entrySet()
            .stream()
            .sorted((e1, e2) -> {
                //Descending
                int compare = e2.getValue().compareTo(e1.getValue());

                if(compare == 0) {
                    //Ascending
                    compare = e1.getKey().compareTo(e2.getKey());
                }

                return compare;
            })
            .collect(
                Collectors.toMap(
                    Map.Entry::getKey,
                    Map.Entry::getValue,
                    (e1, e2) -> e1,
                    LinkedHashMap::new
                )
            );

        boolean isValid = true;

        Iterator<Character> charIter = orderedMap.keySet().iterator();

        for(int i = 0; i < checksum.length() && isValid; ++i) {
            isValid = charIter.hasNext() && charIter.next().equals(checksum.charAt(i));
        }

        return isValid;
    }
}
