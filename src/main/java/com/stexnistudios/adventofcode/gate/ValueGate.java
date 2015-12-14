package com.stexnistudios.adventofcode.gate;

/**
 * Created by akrill on 12/14/15.
 */
public class ValueGate implements IGate {

    private char value;

    public ValueGate(char value) {
        this.value = value;
    }

    @Override
    public char getValue() {
        return value;
    }
}
