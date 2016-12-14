package com.stexnistudios.adventofcode.util;

import java.util.LinkedHashMap;
import java.util.Map;

public class SizeLimitedMap {
    public static <K, V> Map<K, V> create(int maxEntries) {
        return new LinkedHashMap<K, V>(maxEntries*10/7, 0.7f, true) {
            @Override
            protected boolean removeEldestEntry(Map.Entry<K, V> eldest) {
                return size() > maxEntries;
            }
        };
    }
}
