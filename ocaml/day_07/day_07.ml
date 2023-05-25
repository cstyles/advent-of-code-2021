(* the input: all of the starting positions of the crabs *)
let positions =
  let line = open_in "day_07/input.txt" |> In_channel.input_line in
  match line with
  | None -> failwith "no input"
  | Some line -> line |> String.split_on_char ',' |> List.map int_of_string

(* returns the minimum or maximum integer from a list *)
let min_of list = List.fold_left Int.min Int.max_int list
let max_of list = List.fold_left Int.max Int.min_int list
let abs_diff a b = Int.abs (a - b)

(* generate a list of integers from a to b, inclusive *)
let rec upto a b = if a > b then [] else a :: upto (a + 1) b

(* calculate total fuel needed to move to a particular target *)
let total_fuel_part1 target =
  let abs_diff' = abs_diff target in
  List.map abs_diff' positions |> List.fold_left ( + ) 0

let min = min_of positions
let max = max_of positions
let range = upto min max
let part1 = List.map total_fuel_part1 range |> min_of

(* fuel needed to move between two positions in part 2 *)
let fuel_part2 position target =
  let x = abs_diff position target in
  (BatInt.pow x 2 + x) / 2

(* calculate total fuel needed to move to a particular target *)
let total_fuel_part2 target =
  let hm = fuel_part2 target in
  List.map hm positions |> List.fold_left ( + ) 0

let part2 = List.map total_fuel_part2 range |> min_of;;

print_int part1;
print_newline ();
print_int part2;
print_newline ()
