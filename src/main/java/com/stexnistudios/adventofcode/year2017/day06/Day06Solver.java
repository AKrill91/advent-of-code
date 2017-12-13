package com.stexnistudios.adventofcode.year2017.day06;

import com.stexnistudios.adventofcode.Solver;

import java.util.*;

public class Day06Solver extends Solver {
    public Day06Solver(String input) {
        super(input);
    }

    private int[] parseBanks() {
        String[] parts = getInput().split("\t");
        int numBanks = parts.length;
        int[] banks = new int[numBanks];

        for (int i = 0; i < numBanks; ++i) {
            banks[i] = Integer.parseInt(parts[i]);
        }

        return banks;
    }

    @Override
    public Integer solveA() {
        List<Integer> hashes = new ArrayList<>();
        int[] banks = parseBanks();
        int numBanks = banks.length;
        int step = 0;

        int hash = Arrays.hashCode(banks);

        do {
            hashes.add(hash);

            ++step;

            int largestBankVal = banks[0];
            int largestBankIndex = 0;

            for (int i = 1; i < numBanks; ++i) {
                int val = banks[i];
                if (val > largestBankVal) {
                    largestBankVal = val;
                    largestBankIndex = i;
                }
            }

            int numToRedistribute = largestBankVal;
            int index = (largestBankIndex + 1) % numBanks;
            banks[largestBankIndex] = 0;

            while (numToRedistribute > 0) {
                banks[index] += 1;
                numToRedistribute -= 1;
                index = (index + 1) % numBanks;
            }

            hash = Arrays.hashCode(banks);

        } while (!hashes.contains(hash));

        return step;
    }

    @Override
    public Object solveB() {
        Map<Integer, Integer> hashes = new HashMap<>();
        int[] banks = parseBanks();
        int numBanks = banks.length;
        int step = 0;

        int hash = Arrays.hashCode(banks);

        do {
            hashes.put(hash, step);

            ++step;

            int largestBankVal = banks[0];
            int largestBankIndex = 0;

            for (int i = 1; i < numBanks; ++i) {
                int val = banks[i];
                if (val > largestBankVal) {
                    largestBankVal = val;
                    largestBankIndex = i;
                }
            }

            int numToRedistribute = largestBankVal;
            int index = (largestBankIndex + 1) % numBanks;
            banks[largestBankIndex] = 0;

            while (numToRedistribute > 0) {
                banks[index] += 1;
                numToRedistribute -= 1;
                index = (index + 1) % numBanks;
            }

            hash = Arrays.hashCode(banks);

        } while (!hashes.containsKey(hash));

        return step - hashes.get(hash);
    }
}
