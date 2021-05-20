/**
 * In Scala, assigning to anything that isn't an identifier or an argument is
 * just syntactically invalid.
 *
 * With a plain mutable variable as the left operand, an assignment binds it to
 * the object/value produced in the right operand.
 *
 * Like Java, Scala cannot pass a direct reference to a primitive type to an
 * integer. However, it can pass a closure which can perform the assignment when
 * evaluated in a different context.
 *
 * Scala's arrays are indexed using parentheses rather than square brackets,
 * e.g., a(i) instead of a[i]. This desugars to the a's .apply method, i.e.,
 * a.apply(i). But to support assigning to arrays, this notation is desugared
 * differently in the left operand of =, e.g., a(i) = e, this is desugared to
 * a's update method, i.e., a.update(i, e).
 *
 * https://scala-lang.org/files/archive/spec/2.13/06-expressions.html#assignments
 */

/**
 * Declare mutable variable x and immutable variable y, assign x + y to x.
 */
object LocalVar {
  def example() = {
    var x: Int = 1
    val y: Int = 2
    x = x + y
    assert(x == 3)
  }
}

/**
 * Declare mutable variable x and immutable variable y, indirectly assign
 * x + y via a function.
 */
object FuncVar {
  def doAssign(r: Int => Unit, v: Int) = r(v)

  def example() = {
    var x: Int = 1
    val y: Int = 2
    doAssign(x=_, x + y)
    assert(x == 3)
  }
}

object LocalArray {
  def example() = {
    var x: Array[Int] = Array(1, 1)
    val y: Int = 2
    x(0) = x(0) + y

    assert(x(0) == 3)
    assert(x(1) == 1)
  }
}

object FuncArray {
  def doAssign(r: Array[Int], v: Int) = r(0) = v

  def example() = {
    var x: Array[Int] = Array(1, 1)
    val y: Int = 2
    doAssign(x, x(0) + y)
    assert(x(0) == 3)
    assert(x(1) == 1)
  }
}

class S(var i: Int, var b: Boolean) { }

object LocalStruct {
  def example() = {
    var x: S = new S(1, true)
    val y: Int = 2
    x.i = 1
    assert(x.i == 3)
    assert(x.b == true)
  }
}

object FuncStruct {
  def doAssign(r: S, v: Int) = r.i = v
  def example() = {
    var x: S = new S(1, true)
    val y: Int = 2
    doAssign(x, x.i + y)
    assert(x.i == 3)
    assert(x.b == true)
  }
}

object Lrvalues {
  def main(args: Array[String]) = {
    LocalVar.example()
    FuncVar.example()
    LocalArray.example()
    FuncArray.example()
    println("All tests passed!")
  }
}
