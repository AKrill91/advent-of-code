package com.stexnistudios.adventofcode;

import javax.xml.bind.DatatypeConverter;
import java.security.MessageDigest;

public class Day05aSolver extends Solver {
    public Day05aSolver(String input) {
        super(input);
    }

    @Override
    public String call() throws Exception {
        MessageDigest hasher = MessageDigest.getInstance("MD5");
        int hashLength = 8;
        String password = "";

        int counter = 0;
        for (int i = 0; i < hashLength; ++i) {
            String hash = "";
            do {
                String key = getInput() + counter++;
                hash = DatatypeConverter.printHexBinary(hasher.digest(key.getBytes()));
            } while (!hash.substring(0, 5).equals("00000"));

            password += hash.charAt(5);
        }

        return password.toLowerCase();
    }
}
