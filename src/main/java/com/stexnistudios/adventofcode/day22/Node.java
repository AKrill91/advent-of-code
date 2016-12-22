package com.stexnistudios.adventofcode.day22;

import com.stexnistudios.adventofcode.util.Point;

public class Node {
    private final Point position;
    private final int size;
    private final int used;

    public Node(Point position, int size, int used) {
        this.position = position;
        this.size = size;
        this.used = used;
    }

    public Point getPosition() {
        return position;
    }

    public int getSize() {
        return size;
    }

    public int getUsed() {
        return used;
    }

    public int getAvail() {
        return size - used;
    }

    @Override
    public String toString() {
        return "Node{" +
            "position=" + position +
            ", size=" + size +
            ", used=" + used +
            '}';
    }
}
