/**
 * A C expression can be assigned to if it is a non-const lvalue with an object
 * type.
 *
 * lvalues, for the most part, designate non-temporary values. These include
 * variables, the result of dereferences (thus, array members), and the fields
 * of structs.
 *
 * An object type is any type that is not a function type.
 *
 * https://en.cppreference.com/w/c/language/value_category
 * https://en.cppreference.com/w/c/language/type#Type_groups
 */
#include <assert.h>
#include <stdio.h>

void local_var(void) {
  int x = 1;
  const int y = 2;

  x = x + y;

  assert(x == 3);
}

void do_assign_var(int *r, int v) { *r = v; }

void func_var(void) {
  int x = 1;
  const int y = 2;

  do_assign_var(&x, x + y);

  assert(x == 3);
}

void local_array(void) {
  int x[2] = {1, 1};
  const int y = 2;

  x[0] = x[0] + y;

  assert(x[0] == 3);
  assert(x[1] == 1);
}

/**
 * Due to how array access works in C, do_assign_var works exactly the same as
 * do_assign_array. For clarity, I write them with different syntactic sugar.
 */
void do_assign_array(int *r, int v) { r[0] = v; }

void func_array(void) {
  int x[2] = {1, 1};
  const int y = 2;

  do_assign_array(x, x[0] + y);

  assert(x[0] == 3);
  assert(x[1] == 1);
}

struct S {
  int i;
  char b;
};

void local_struct(void) {
  struct S x = {.i = 1, .b = 1};
  const int y = 2;

  x.i = x.i + y;

  assert(x.i == 3);
  assert(x.b == 1);
}

void do_assign_struct(struct S *r, int v) { r->i = v; }

void func_struct(void) {
  struct S x = {.i = 1, .b = 1};
  const int y = 2;

  do_assign_struct(&x, x.i + y);

  assert(x.i == 3);
  assert(x.b == 1);
}
/**
 * Strangely, compound literals and string literals are both considered lvalues.
 * Thus, the following constructs are fine and seem to compile without warnings
 * (though the behavior is probably undefined).
 */

void anomalies(void) {
  (struct S){.i = 3} = (struct S){.i = 2};
  (struct { int x; }){.x = 3}.x = 2;

  /* (struct S){} = (struct S){}; // forbidden by ISO C */

  /* *"yes" = '\0'; // segfaults at runtime */
}

int main(void) {
  local_var();
  func_var();
  local_array();
  func_array();
  local_struct();
  func_struct();

  anomalies();

  printf("Tests passed!\n");
  return 0;
}
