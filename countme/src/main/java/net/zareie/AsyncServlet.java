package net.zareie;

import javax.servlet.*;
import javax.servlet.http.HttpServlet;
import javax.servlet.http.HttpServletRequest;
import javax.servlet.http.HttpServletResponse;
import java.io.IOException;
import java.nio.ByteBuffer;
import java.nio.charset.StandardCharsets;
import java.util.Arrays;
import java.util.concurrent.atomic.AtomicInteger;

public class AsyncServlet extends HttpServlet {
    private static final AtomicInteger counter = new AtomicInteger(0);

    private static ByteBuffer POST_RESPONSE = ByteBuffer.wrap("".getBytes(StandardCharsets.UTF_8));

    private static void incCounter(int value) {
        synchronized (counter) {
            counter.set(counter.get() + value);
        }
    }


    protected void doGet(HttpServletRequest request, HttpServletResponse response)
            throws ServletException, IOException {

        String counterString = String.format("%s", counter.get());
        ByteBuffer GET_RESPONSE = ByteBuffer.wrap(counterString.getBytes(StandardCharsets.UTF_8));
        AsyncContext async = request.startAsync();
        ServletOutputStream out = response.getOutputStream();
        out.setWriteListener(new WriteListener() {
            @Override
            public void onWritePossible() throws IOException {
                while (out.isReady()) {
                    if (!GET_RESPONSE.hasRemaining()) {
                        response.setStatus(200);
                        async.complete();
                        return;
                    }
                    out.write(GET_RESPONSE.get());
                }
            }

            @Override
            public void onError(Throwable t) {
                getServletContext().log("Async Error", t);
                async.complete();
            }
        });


    }

    protected void doPost(HttpServletRequest request, HttpServletResponse response)
            throws ServletException, IOException {
        AsyncContext async = request.startAsync();
        ServletOutputStream out = response.getOutputStream();
        out.setWriteListener(new WriteListener() {
            @Override
            public void onWritePossible() throws IOException {
                while (out.isReady()) {
                    if (!POST_RESPONSE.hasRemaining()) {
                        response.setStatus(200);
                        async.complete();
                        return;
                    }
                    out.write(POST_RESPONSE.get());
                }
            }

            @Override
            public void onError(Throwable t) {
                getServletContext().log("Async Error", t);
                async.complete();
            }
        });
        ServletInputStream input = request.getInputStream();
        input.setReadListener(new ReadListener() {
            @Override
            public void onDataAvailable() throws IOException {
                byte[] buffer = new byte[1024];
                int size = request.getInputStream().read(buffer);
                incCounter(Integer.parseInt(new String(Arrays.copyOfRange(buffer, 0, size))));

            }

            @Override
            public void onAllDataRead() throws IOException {


            }

            @Override
            public void onError(Throwable throwable) {
                System.out.println(request);

            }
        });
        Integer value = 0;
        incCounter(value);
    }
}