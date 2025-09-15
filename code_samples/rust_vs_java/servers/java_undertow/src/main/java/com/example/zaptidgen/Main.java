package com.example.zaptidgen;

import io.undertow.Undertow;
import io.undertow.server.RoutingHandler;
import io.undertow.util.Headers;
import java.util.concurrent.ThreadLocalRandom;
import java.util.concurrent.atomic.AtomicLong;

public class Main {
    private static final AtomicLong COUNTER =
            new AtomicLong(ThreadLocalRandom.current().nextLong(1, Long.MAX_VALUE));

    public static void main(String[] args) {
        RoutingHandler routes = new RoutingHandler()
            .get("/nextid", exchange -> {
                long id = COUNTER.incrementAndGet();
                String json = "{\"error\":false,\"id\":" + Long.toUnsignedString(id) + "}";
                exchange.getResponseHeaders().put(Headers.CONTENT_TYPE, "application/json");
                exchange.setStatusCode(200);
                exchange.getResponseSender().send(json);
            })
            .setFallbackHandler(exchange -> {
                exchange.setStatusCode(404);
                exchange.getResponseSender().send("Unsupported path");
            });

        Undertow server = Undertow.builder()
            .addHttpListener(8888, "0.0.0.0")
            .setHandler(routes)
            .build();

        System.out.println("INFO: Java Undertow listening on :8888");
        server.start();
    }
}
