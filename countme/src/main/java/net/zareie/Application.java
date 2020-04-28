package net.zareie;


import fi.iki.elonen.NanoHTTPD;

import java.io.IOException;
import java.util.Arrays;
import java.util.concurrent.atomic.AtomicReference;

public class Application extends NanoHTTPD {

    private static final AtomicReference<Integer> count = new AtomicReference<>(0);

    private static Integer getCounter() {
        return count.get();
    }

    private static void incCounter(int value) {
        synchronized (count) {
            count.updateAndGet(v -> v + value);
        }
    }

    public Application() throws IOException {
        super(80);
        start(NanoHTTPD.SOCKET_READ_TIMEOUT, false);
    }

    public static void main(String[] args) {
        try {
            new Application();
        } catch (IOException ioe) {
            System.err.println("Couldn't start server:\n" + ioe);
        }
    }

    @Override
    public Response serve(IHTTPSession session) {
        if (session.getMethod() == Method.POST) {
            byte[] buffer = new byte[50];
            try {
                int size = session.getInputStream().read(buffer);
                incCounter(Integer.parseInt(new String(Arrays.copyOfRange(buffer, 0, size)).trim()));
            } catch (IOException e) {
                e.printStackTrace();
            }
            return newFixedLengthResponse("");
        }
        return newFixedLengthResponse(getCounter().toString());
    }
}
