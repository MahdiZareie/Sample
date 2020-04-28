package net.zareie;

/**
 * Hello world!
 *
 */
public class Application
{
    public static void main( String[] args ) throws Exception {
        System.out.println( "Hello World!" );
        JettyServer js = new JettyServer();
        js.start();
    }
}
