let local_var () =
  let x = ref 1 in
  let y = 2 in
  x := !x + y;
  x

let%test _ = !(local_var ()) = 3

let func_var () =
  let do_assign r v () =
    r := v
  in
  let x = ref 1 in
  let y = 2 in
  do_assign x (!x + y) ();
  x

let%test _ = !(func_var ()) = 3

let local_array () =
  let x = [| 1; 1 |] in
  let y = 2 in
  x.(0) <- x.(0) + y;
  x

let%test _ =
  let a = local_array () in
  a.(0) = 3 && a.(1) = 1

let func_array () =
  let do_assign r v () =
    r.(0) <- v
  in
  let x = [| 1; 1 |] in
  let y = 2 in
  do_assign x (x.(0) + y) ();
  x

type s = {
  mutable i: int;
  mutable b: bool;
}

let local_struct () =
  let x = {i=1; b=true} in
  let y = 2 in
  x.i <- x.i + y;
  x

let%test _ =
  let s = local_struct () in
  s.i = 3 && s.b = true

let func_struct () =
  let do_assign r v () =
    r.i <- v
  in
  let x = {i=1; b=true} in
  let y = 2 in
  do_assign x (x.i + y) ();
  x

let%test _ =
  let s = func_struct () in
  s.i = 3 && s.b = true
