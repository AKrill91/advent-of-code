package com.stexnistudios.adventofcode.util;

public class Range<T extends Comparable<T>> implements Comparable<Range<T>> {
    private final T minimum;
    private final T maximum;
    private final boolean inclusive;

    public Range(T minimum, T maximum) {
        this(minimum, maximum, true);
    }

    public Range(T minimum, T maximum, boolean inclusive) {
        this.minimum = minimum;
        this.maximum = maximum;
        this.inclusive = inclusive;
    }

    public T getMinimum() {
        return minimum;
    }

    public T getMaximum() {
        return maximum;
    }

    public boolean isInclusive() {
        return inclusive;
    }

    public boolean includes(T compare) {
        int minCompare = compare.compareTo(minimum);
        int maxCompare = compare.compareTo(maximum);

        return (minCompare > 0 && maxCompare < 0) ||
            ((minCompare == 0 || maxCompare == 0) && inclusive);
    }

    @Override
    public int compareTo(Range<T> other) {
        return minimum.compareTo(other.minimum);
    }
}
