package com.stexnistudios.adventofcode.gate;

public abstract class BinaryGate extends UnaryGate {

    protected IGate gate2;

    public BinaryGate(IGate gate1, IGate gate2) {
        super(gate1);
        this.gate2 = gate2;
    }
}
