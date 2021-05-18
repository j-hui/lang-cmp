/**
 * C++ has a much more lax policy than C for assignability, though its value
 * categories are also far more complex. This seems to be for accommodating
 * l-value reference types, overloaded assignment operators, and methods in
 * general.
 *
 * The cppreference article on value categories chronicles the history of value
 * categories from CPL to C++17.
 *
 * https://en.cppreference.com/w/cpp/language/value_category
 */
#include <cassert>
#include <iostream>

namespace local_var {
void example(void) {
  int x = 1;
  const int y = 2;

  x = x + y;

  assert(x == 3);
}
} // namespace local_var

namespace func_var {
void do_assign(int &r, int v) { r = v; }

void example(void) {
  int x = 1;
  const int y = 2;

  do_assign(x, x + y);

  assert(x == 3);
}
} // namespace func_var

namespace local_array {
void example(void) {
  int x[2] = {1, 1};
  const int y = 2;

  x[0] = x[0] + y;

  assert(x[0] == 3);
  assert(x[1] == 1);
}
} // namespace local_array

namespace func_array {
template <unsigned int N> void do_assign(int (&r)[N], int v) { r[0] = v; }
void example(void) {

  int x[2] = {1, 1};
  const int y = 2;

  do_assign(x, x[0] + y);

  assert(x[0] == 3);
  assert(x[1] == 1);
}
} // namespace func_array

namespace local_struct {
struct S {
  int i;
  bool b;
};
void example(void) {
  struct S x = {.i = 1, .b = true};
  const int y = 2;

  x.i = x.i + y;

  assert(x.i == 3);
  assert(x.b == true);
}
} // namespace local_struct

namespace func_struct {
struct S {
  int i;
  bool b;
};
void do_assign(struct S &r, int v) { r.i = v; }
void example(void) {
  struct S x = {.i = 1, .b = true};
  const int y = 2;

  do_assign(x, x.i + y);

  assert(x.i == 3);
  assert(x.b == true);
}
} // namespace func_struct

namespace anomalies {
namespace assign_to_func {
struct S {
  int x;
};
S f() { return S{3}; }
void example(void) {
  f() = S{2}; // this is assignable
              // f().x = 3;   // yet this is not
}
} // namespace assign_to_func
namespace assign_to_assign {
int example(void) {
  int x = 3;
  (x = 2) = 1;
  return x;
}
} // namespace assign_to_assign
} // namespace anomalies

int main(void) {
  local_var::example();
  func_var::example();
  local_array::example();
  func_array::example();
  local_struct::example();
  func_struct::example();
  printf("Tests passed!\n");
  return 0;
}
