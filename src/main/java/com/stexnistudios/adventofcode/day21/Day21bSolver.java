package com.stexnistudios.adventofcode.day21;

import com.google.common.collect.Collections2;
import com.stexnistudios.adventofcode.Solver;

import java.util.ArrayList;
import java.util.Collection;
import java.util.List;
import java.util.stream.Collectors;

public class Day21bSolver extends Solver {

    private final String target;
    private final List<String> permutations;

    public Day21bSolver(String input) {
        this(input, "fbgdceah");
    }

    public Day21bSolver(String input, String target) {
        super(input);

        this.target = target;
        Collection<Character> characters = new ArrayList<>();
        for (Character c : target.toCharArray()) {
            characters.add(c);
        }

        permutations = new ArrayList<>();

        Collection<List<Character>> perms = Collections2.orderedPermutations(characters);

        perms.forEach(chars -> {
            StringBuilder sb = new StringBuilder();

            chars.forEach(sb::append);

            permutations.add(sb.toString());
        });

    }

    @Override
    public String call() {
        List<String> matches = permutations.stream()
            .filter(s -> new Day21aSolver(getInput(), s).call().equals(target))
            .collect(Collectors.toList());

        logger.info("{} matches found: {}", matches.size(), matches);

        return matches.get(0);
    }
}
