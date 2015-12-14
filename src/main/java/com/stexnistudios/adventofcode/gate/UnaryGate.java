package com.stexnistudios.adventofcode.gate;

public abstract class UnaryGate implements IGate {
    protected IGate gate1;

    public UnaryGate(IGate gate1) {
        this.gate1 = gate1;
    }
}
