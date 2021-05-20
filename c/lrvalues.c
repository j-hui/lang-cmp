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
 *
 * A set of deductive rules to describe the value category system:
 *
 * x : L-Value     y : R-value
 * --------------------------- STORE
 *      x = y : R-value
 *
 * e : L-value
 * ------------ ADDRESS_OF
 * &e : R-value
 *
 * x : L-value
 * ----------- LOAD
 * x : R-value
 *
 * x \in Variables
 * --------------- VARIABLE
 * x : L-value
 *
 * l \in IntLiterals
 * ----------------- INT_LITERAL
 * e : R-value
 *
 * e : R-value
 * --------------- FIELD
 * e . f : L-value
 *
 * e : R-value
 * ------------ DEREFERENCE
 * *e : L-value
 *
 * ------------------- CALL
 * f ( ... ) : R-value
 *
 * The rules are similar to type coercions in C. By default, we interpret things
 * as l-values, but will coerce them r-values as needed (this is the LOAD rule).
 * Coercion "requests" start from the RHS of an assignment and an ampersand.
 *
 * After playing with the Godbolt compiler explorer (with optimizations turned
 * off), using various version of GCC, the code generator for r-values passes
 * the l-value in as an argument: the address of where the result should be
 * placed. This address comes from the code generator for l-values.
 *
 *    void gen_rhs(expr *e, address *lvalue_location);
 *    address *gen_lhs(expr *e);
 *
 * This is the example we put into Godbolt:
 *
 *  struct S {
 *    int i;
 *    char b;
 *    int x, y , z ;
 *  };
 *  void foo(void) {
 *    (struct S){.b = 2, .i = 3}
 *    = (struct S) { .x = 3 }
 *    ;
 *  }
 *
 * GCC generates the following x86 assembly (seemingly version-independent):
 *
 *  foo:
 *          push    rbp
 *          mov     rbp, rsp
 *          ; (struct S){.b = 2, .i = 3}
 *          mov     QWORD PTR [rbp-32], 0
 *          mov     QWORD PTR [rbp-24], 0
 *          mov     DWORD PTR [rbp-16], 0
 *          mov     DWORD PTR [rbp-32], 3
 *          mov     BYTE PTR [rbp-28], 2
 *          ; = (struct S) { .x = 3 }
 *          mov     QWORD PTR [rbp-32], 0
 *          mov     QWORD PTR [rbp-24], 0
 *          mov     DWORD PTR [rbp-16], 0
 *          mov     DWORD PTR [rbp-24], 3
 *          nop
 *          pop     rbp
 *          ret
 *
 * Note that the new literal (struct S) {.x = 3} directly overwrites the
 * struct literal (struct S) {.b = 2, .i = 3}.
 *
 * Meanwhile, Clang generates the following:
 *
 *  foo:                                    # @foo
 *          push    rbp
 *          mov     rbp, rsp
 *          sub     rsp, 48
 *          ; (struct S) {.b = 2, .i = 3}
 *          lea     rdi, [rbp - 24]
 *          xor     esi, esi
 *          mov     edx, 20
 *          call    memset@PLT
 *          mov     dword ptr [rbp - 24], 3
 *          mov     byte ptr [rbp - 20], 2
 *          ; = (struct S) {.x = 3}
 *          lea     rdi, [rbp - 48]
 *          xor     esi, esi
 *          mov     edx, 20
 *          call    memset@PLT
 *          mov     dword ptr [rbp - 40], 3
 *          ; assignment begins
 *          mov     rax, qword ptr [rbp - 48]
 *          mov     qword ptr [rbp - 24], rax
 *          mov     rax, qword ptr [rbp - 40]
 *          mov     qword ptr [rbp - 16], rax
 *          mov     eax, dword ptr [rbp - 32]
 *          mov     dword ptr [rbp - 8], eax
 *          add     rsp, 48
 *          pop     rbp
 *          ret
 *
 * The second struct literal is also given space on the stack, initialized, and
 * then copied over to the first struct literal. Note that instead of writing
 * quadwords, Clang calls memset for initialization.
 *
 *    address *gen_value(expr *e);
 *    void gen_assignment(address *lhs, address *rhs) {
 *      gen_copy(lhs, rhs);
 *    }
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
