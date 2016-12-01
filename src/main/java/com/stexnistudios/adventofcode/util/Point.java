package com.stexnistudios.adventofcode.util;

public class Point {
    private final int x;
    private final int y;

    public Point() {
        this(0, 0);
    }

    public Point(int x, int y) {
        this.x = x;
        this.y = y;
    }

    public int getX() {
        return x;
    }

    public int getY() {
        return y;
    }

    public Point add(Point other) {
        return new Point(getX() + other.getX(), getY() + other.getY());
    }

    @Override
    public String toString() {
        return "Point{" + "x=" + getX() + ", y=" + getY() + '}';
    }
}
