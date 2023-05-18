(* Interprets a list as a 2-tuple and returns whether the first element is less than the second. *)
let is_increasing list = List.nth list 0 < List.nth list 1
let lines = Util.input_lines "day_01/input.txt" |> List.map int_of_string
let part1 = lines |> Util.windows 2 |> List.filter is_increasing |> List.length

let part2 =
  lines
  |> Util.windows 3
  |> List.map Util.sum
  |> Util.windows 2
  |> List.filter is_increasing
  |> List.length
;;

print_string "part1 = ";
Util.println_int part1;
print_string "part2 = ";
Util.println_int part2
