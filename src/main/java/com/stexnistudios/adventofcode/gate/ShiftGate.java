package com.stexnistudios.adventofcode.gate;

/**
 * Created by akrill on 12/14/15.
 */
public abstract class ShiftGate extends UnaryGate {

    protected char amount;

    public ShiftGate(IGate gate1, char amount) {
        super(gate1);
        this.amount = amount;
    }
}
