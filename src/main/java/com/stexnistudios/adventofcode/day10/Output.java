package com.stexnistudios.adventofcode.day10;

import java.util.ArrayList;
import java.util.List;
import java.util.function.Consumer;

public class Output implements ChipConsumer {
    private final int id;
    private final List<Integer> chips;

    public Output(int id) {
        this.id = id;
        chips = new ArrayList<>();
    }

    @Override
    public void accept(Integer chip) {
        chips.add(chip);
    }

    public List<Integer> getChips() {
        return chips;
    }

    @Override
    public String toString() {
        return String.format("output-%d", id);
    }
}
