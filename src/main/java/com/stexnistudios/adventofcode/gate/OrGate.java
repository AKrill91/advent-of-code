package com.stexnistudios.adventofcode.gate;

/**
 * Created by akrill on 12/14/15.
 */
public class OrGate extends BinaryGate {
    public OrGate(IGate gate1, IGate gate2) {
        super(gate1, gate2);
    }

    @Override
    public char getValue() {
        return (char)(gate1.getValue() | gate2.getValue());
    }
}
