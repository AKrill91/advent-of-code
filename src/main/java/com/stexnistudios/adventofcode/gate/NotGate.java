package com.stexnistudios.adventofcode.gate;

/**
 * Created by akrill on 12/14/15.
 */
public class NotGate extends UnaryGate {
    public NotGate(IGate gate1) {
        super(gate1);
    }

    @Override
    public char getValue() {
        return (char)~gate1.getValue();
    }
}
