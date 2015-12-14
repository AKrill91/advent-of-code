package com.stexnistudios.adventofcode.gate;

/**
 * Created by akrill on 12/14/15.
 */
public class LeftShiftGate extends ShiftGate {
    public LeftShiftGate(IGate gate1, char amount) {
        super(gate1, amount);
    }

    @Override
    public char getValue() {
        return (char)(gate1.getValue() << amount);
    }
}
