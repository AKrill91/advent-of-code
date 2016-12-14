package com.stexnistudios.adventofcode.day14;

import com.stexnistudios.adventofcode.Solver;
import com.stexnistudios.adventofcode.util.SizeLimitedMap;

import javax.xml.bind.DatatypeConverter;
import java.security.MessageDigest;
import java.util.*;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

public class Day14aSolver extends Solver {

    private final int stretch;
    private final List<Integer> keyIndices;
    private final Map<Integer, String> cache;
    private final MessageDigest digest;
    private final Pattern triplet;
    private final Map<Character, Pattern> quintuplets;

    public Day14aSolver(String input) throws Exception {
        this(input, 0);
    }

    public Day14aSolver(String input, int stretch) throws Exception {
        super(input);

        keyIndices = new ArrayList<>(64);
        cache = SizeLimitedMap.create(8000);
        digest = MessageDigest.getInstance("MD5");
        triplet = Pattern.compile("([0-9a-f])\\1{2,}");
        quintuplets = new HashMap<>();
        quintuplets.put('0', Pattern.compile("0{5}"));
        quintuplets.put('1', Pattern.compile("1{5}"));
        quintuplets.put('2', Pattern.compile("2{5}"));
        quintuplets.put('3', Pattern.compile("3{5}"));
        quintuplets.put('4', Pattern.compile("4{5}"));
        quintuplets.put('5', Pattern.compile("5{5}"));
        quintuplets.put('6', Pattern.compile("6{5}"));
        quintuplets.put('7', Pattern.compile("7{5}"));
        quintuplets.put('8', Pattern.compile("8{5}"));
        quintuplets.put('9', Pattern.compile("9{5}"));
        quintuplets.put('a', Pattern.compile("a{5}"));
        quintuplets.put('b', Pattern.compile("b{5}"));
        quintuplets.put('c', Pattern.compile("c{5}"));
        quintuplets.put('d', Pattern.compile("d{5}"));
        quintuplets.put('e', Pattern.compile("e{5}"));
        quintuplets.put('f', Pattern.compile("f{5}"));

        this.stretch = stretch;
    }

    @Override
    public Integer call() throws Exception {
        int hashCount = 0;
        int index = 0;

        while (hashCount < 64) {
            String md5 = getMd5(index);

            Matcher tripletMatcher = triplet.matcher(md5);
            Character digit = null;

            if (tripletMatcher.find()) {
                digit = tripletMatcher.group(1).charAt(0);
            }

            if (digit != null) {
                boolean found = false;
                Pattern quintuplet = quintuplets.get(digit);
                for (int i = 1; i <= 1000 && !found; ++i) {
                    md5 = getMd5(index + i);
                    Matcher quintupletMatcher = quintuplet.matcher(md5);

                    if (quintupletMatcher.find()) {
                        found = true;
                        keyIndices.add(index);
                        ++hashCount;
                    }
                }
            }

            ++index;
        }

        return keyIndices.get(keyIndices.size() - 1);
    }

    private String getMd5(int index) {
        String output;
        if (cache.containsKey(index)) {
            output = cache.get(index);
        } else {
            output = DatatypeConverter.printHexBinary(
                digest.digest((getInput() + index).getBytes())
            )
                .toLowerCase();

            for (int i = 0; i < stretch; ++i) {
                output = DatatypeConverter.printHexBinary(
                    digest.digest(output.getBytes())
                )
                    .toLowerCase();
            }

            cache.put(index, output);
        }

        return output;
    }
}
