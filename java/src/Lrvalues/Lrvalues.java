/**
 * In Java, there isn't a notion of an l-value vs r-value. There are simply
 * variables and values. The left operand of an assignment must be a variable.
 *
 * Variables may be those that are explicitly declared as such, or computed
 * variables, which result from array element and object member accesses.
 *
 * A variable declared final cannot be assigned to because it is treated as
 * a value in expressions. However, computed variables from final-declared
 * variables are still assignable.
 *
 * https://docs.oracle.com/javase/specs/jls/se8/html/jls-15.html
 * https://docs.oracle.com/javase/specs/jls/se8/html/jls-15.html#jls-AssignmentExpression
 * https://docs.oracle.com/javase/specs/jls/se8/html/jls-4.html#jls-4.12.4
 */

class LocalVar {
  public static void example() {
    int x = 1;
    final int y = 2;
    x = x + y;
    assert x == 3;
  }
}

/** Also provided in Java standard library */
class Box<T> {
  T i;

  Box(T i) { this.i = i; }

  void set(T i) { this.i = i; }
  T get() { return i; }
}

class FuncVar {
  static void doAssign(Box<Integer> r, int v) {
    r.i = v;
    // alternatively, r.set(v);
  }

  public static void example() {
    Box<Integer> x = new Box<Integer>(1);
    final int y = 2;
    FuncVar.doAssign(x, x.get() + y);
    assert x.get() == 3;
  }
}

class LocalArray {
  public static void example() {
    int[] x = new int[] {1, 1};
    // final int[] x = new int[]{1, 1}; // also works
    final int y = 2;

    x[0] = x[0] + y;

    assert x[0] == 3;
    assert x[1] == 1;
  }
}

class FuncArray {
  static void doAssign(int[] r, int v) { r[0] = v; }

  public static void example() {
    int[] x = new int[] {1};
    final int y = 2;

    FuncArray.doAssign(x, x[0] + y);

    assert x[0] == 3;
    assert x[1] == 1;
  }
}

class S {
  int i;
  boolean b;

  S(int i, boolean b) {
    this.i = i;
    this.b = b;
  }
}

class LocalStruct {
  public static void example() {
    S x = new S(1, true);
    final int y = 2;

    x.i = x.i + y;

    assert x.i == 3;
    assert x.b == true;
  }
}

class FuncStruct {
  static void doAssign(S r, int v) { r.i = v; }

  public static void example() {
    S x = new S(1, true);
    final int y = 2;

    doAssign(x, x.i + y);

    assert x.i == 3;
    assert x.b == true;
  }
}

class Anomalies {
  public static void example() {
    new int[]{1, 1}[0] = 3;

    new S(1, true).i = 3;

    int x = new int[]{1, 1}[0];
  }
}

public class Lrvalues {
  public static void main(String[] args) {
    LocalVar.example();
    FuncVar.example();
    LocalArray.example();
    FuncArray.example();
    LocalStruct.example();
    FuncStruct.example();
    System.out.println("Passed all tests!");
  }
}
