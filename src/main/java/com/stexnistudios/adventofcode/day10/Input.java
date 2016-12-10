package com.stexnistudios.adventofcode.day10;

import java.util.ArrayList;
import java.util.List;

public class Input {
    private final int id;
    private final List<ChipConsumer> consumers;

    public Input(int id) {
        this.id = id;
        consumers = new ArrayList<>();
    }

    public void addConsumer(ChipConsumer consumer) {
        consumers.add(consumer);
    }

    public void supply() {
        consumers.forEach(chipConsumer -> chipConsumer.accept(id));
    }

    @Override
    public String toString() {
        return String.format("input-%d", id);
    }
}
