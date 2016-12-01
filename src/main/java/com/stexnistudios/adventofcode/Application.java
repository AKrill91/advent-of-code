package com.stexnistudios.adventofcode;

import com.google.common.base.Strings;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;

public class Application implements Runnable {
    public static void main(String[] args) throws IOException {
        new Application().run();
    }

    private final Logger logger;

    public Application() {
        logger = LoggerFactory.getLogger(getClass());
    }


    @Override
    public void run() {
        logger.info("Hello, World!");
    }
}
