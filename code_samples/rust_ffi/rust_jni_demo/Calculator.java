public class Calculator {
    static {
        System.loadLibrary("rust_jni_demo");
    }

    public native long factorial(int n);

    public static void main(String[] args) {
        Calculator calc = new Calculator();
        System.out.println(calc.factorial(10)); // Output: 3628800
    }
}