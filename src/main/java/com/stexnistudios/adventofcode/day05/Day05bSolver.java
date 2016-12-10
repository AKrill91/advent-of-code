package com.stexnistudios.adventofcode.day05;

import com.stexnistudios.adventofcode.Solver;

import javax.xml.bind.DatatypeConverter;
import java.security.MessageDigest;

public class Day05bSolver extends Solver {
    public Day05bSolver(String input) {
        super(input);
    }

    @Override
    public String call() throws Exception {
        MessageDigest hasher = MessageDigest.getInstance("MD5");
        int hashLength = 8;
        Character[] characters = new Character[hashLength];

        int counter = 0;
        for (int i = 0; i < hashLength; ++i) {
            String hash;
            int index;
            do {
                String key = getInput() + counter++;
                hash = DatatypeConverter.printHexBinary(hasher.digest(key.getBytes()));
                index = hash.charAt(5) - 48;
            } while (
                !(
                    hash.substring(0, 5).equals("00000") &&
                        index < 8 &&
                        characters[index] == null
                ));

            characters[index] = hash.charAt(6);
        }

        StringBuilder sb = new StringBuilder();

        for (Character character : characters) {
            sb.append(character);
        }

        return sb.toString().toLowerCase();
    }
}
