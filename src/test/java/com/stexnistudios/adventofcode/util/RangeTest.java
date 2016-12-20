package com.stexnistudios.adventofcode.util;

import org.junit.Assert;
import org.junit.Test;
import org.junit.runner.RunWith;
import org.junit.runners.Parameterized;

import java.util.Arrays;
import java.util.Collection;

@RunWith(Parameterized.class)
public class RangeTest {

    @Parameterized.Parameters
    public static Collection<Object[]> data() {
        return Arrays.asList(
            new Object[][]{
                {0, 10, false, 7, true},
                {0, 10, false, -1, false},
                {0, 10, true, 0, true},
                {0, 10, false, 0, false},
                {0, 10, false, 12, false}
            }
        );
    }

    private final int minimum;
    private final int maximum;
    private final boolean inclusive;
    private final int toTest;
    private final boolean expected;

    public RangeTest(
        int minimum,
        int maximum,
        boolean inclusive,
        int toTest,
        boolean expected
    ) {
        this.minimum = minimum;
        this.maximum = maximum;
        this.inclusive = inclusive;
        this.toTest = toTest;
        this.expected = expected;
    }

    @Test
    public void test() {
        Assert.assertEquals(
            expected,
            new Range<>(minimum, maximum, inclusive).includes(toTest)
        );
    }
}