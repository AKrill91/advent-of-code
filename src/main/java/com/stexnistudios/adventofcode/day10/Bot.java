package com.stexnistudios.adventofcode.day10;

import java.util.function.Consumer;

public class Bot implements ChipConsumer {
    private final int id;
    private final BotCompareObserver observer;

    private Integer chipA;
    private Integer chipB;

    private Consumer<Integer> lowConsumer;
    private Consumer<Integer> highConsumer;

    public Bot(int id, BotCompareObserver observer) {
        this.id = id;
        this.observer = observer;
    }

    @Override
    public void accept(Integer chip) {
        if (chipA == null) {
            chipA = chip;
        } else {
            chipB = chip;
        }

        if (chipA != null && chipB != null) {
            int low = Math.min(chipA, chipB);
            int high = Math.max(chipA, chip);

            observer.notify(id, low, high);

            lowConsumer.accept(low);
            highConsumer.accept(high);

            chipA = null;
            chipB = null;
        }
    }

    public void setLowConsumer(Consumer<Integer> lowConsumer) {
        this.lowConsumer = lowConsumer;
    }

    public void setHighConsumer(Consumer<Integer> highConsumer) {
        this.highConsumer = highConsumer;
    }

    @Override
    public String toString() {
        return String.format("bot-%d", id);
    }
}
