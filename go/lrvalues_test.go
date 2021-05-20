package lrvalues

import "testing"

func assertEq(t *testing.T, a interface{}, b interface{}) {
	if a != b {
		t.Fatal()
	}
}

func TestLocalVar(t *testing.T) {
	x := 1
	const y = 2

	x = x + y

	assertEq(t, x, 3)
}

func doAssignVar(r *int, v int) {
	*r = v
}

func TestFuncVar(t *testing.T) {
	x := 1
	const y = 2

	doAssignVar(&x, x+y)

	assertEq(t, x, 3)
}

func TestLocalArray(t *testing.T) {
	x := []int{1, 1}
	const y = 2

	x[0] = x[0] + y

	assertEq(t, x[0], 3)
	assertEq(t, x[1], 1)
}

func doAssignArray(r []int, v int) {
	r[0] = v
}

func TestFuncArray(t *testing.T) {
	x := []int{1, 1}
	const y = 2

	doAssignArray(x, x[0]+y)

	assertEq(t, x[0], 3)
	assertEq(t, x[1], 1)
}

type S struct {
	i int
	b bool
}

func TestLocalStruct(t *testing.T) {
	x := S{i: 1, b: true}
	const y = 2

	x.i = x.i + y

	assertEq(t, x.i, 3)
	assertEq(t, x.b, true)
}

func doAssignStruct(r *S, v int) {
	r.i = v
}

func TestFuncStruct(t *testing.T) {
	x := S{i: 1, b: true}
	const y = 2

	doAssignStruct(&x, x.i+y)

	assertEq(t, x.i, 3)
	assertEq(t, x.b, true)
}

func TestAnomalies(t *testing.T) {
	[]int{1, 1}[0] = 3 // this is allowed
	// S{i: 1, b: true}.i = 3 // but this is forbidden?
}
