exception InvalidInput

let non_empty string = String.length string > 0
let left (l, _) = l
let right (_, r) = r
let length_is len string = len = String.length string
let input = Util.input_lines "day_08/input.txt"

let part1 =
  input
  |> List.filter_map (Util.split_once " | ")
  |> List.map right
  |> List.map (String.split_on_char ' ')
  |> List.flatten
  |> List.map String.length
  |> List.filter (fun len -> len == 2 || len == 3 || len == 4 || len == 7)
  |> List.length

let deduce_a_and_cf one seven =
  let one_doesnt_contain c = String.contains one c |> not in
  let a, cf = Util.partition one_doesnt_contain (BatString.explode seven) in
  (a, cf)

(* Returns true if every char in `list` is contained in string *)
let char_list_contains list string =
  List.for_all (fun c -> String.contains string c) list

let deduce_zero_six_nine bd cf zero_six_nine =
  let zero = ref "" in
  let six = ref "" in
  let nine = ref "" in
  List.iter
    (fun number ->
      (* If a number doesn't contain both `b` and `d`, it must be 0 *)
      if not (char_list_contains bd number) then zero := number
        (* If a number contain both `c` and `f`, it must be 9 *)
      else if char_list_contains cf number then nine := number
      else six := number)
    zero_six_nine;
  (!zero, !six, !nine)

let deduce_two_three_five bd cf two_three_five =
  let two = ref "" in
  let three = ref "" in
  let five = ref "" in
  List.iter
    (fun number ->
      (* If a number contains both `b` and `d`, it must be 5 *)
      if char_list_contains bd number then five := number
        (* If a number contains both `c` and `f`, it must be 3 *)
      else if char_list_contains cf number then three := number
      else two := number)
    two_three_five;
  (!two, !three, !five)

let sort_str string =
  BatString.explode string |> List.sort Char.compare |> BatString.of_list

let enumerate list = List.mapi (fun i a -> (i, a)) list

let decode line =
  (* == Parse == *)
  let examples, output = Util.split_once " | " line |> Option.get in
  let examples = String.split_on_char ' ' examples in
  let one = List.find (length_is 2) examples in
  let four = List.find (length_is 4) examples in
  let seven = List.find (length_is 3) examples in
  let eight = List.find (length_is 7) examples in
  let zero_six_nine = List.filter (length_is 6) examples in
  let two_three_five = List.filter (length_is 5) examples in

  (* == Deduce == *)
  let _a, cf = deduce_a_and_cf one seven in
  let bd =
    List.filter (fun c -> not (String.contains one c)) (BatString.explode four)
  in
  let zero, six, nine = deduce_zero_six_nine bd cf zero_six_nine in
  let two, three, five = deduce_two_three_five bd cf two_three_five in

  (* == Decode == *)
  let value = ref 0 in
  let map = Hashtbl.create 10 in
  Hashtbl.add map (sort_str zero) 0;
  Hashtbl.add map (sort_str one) 1;
  Hashtbl.add map (sort_str two) 2;
  Hashtbl.add map (sort_str three) 3;
  Hashtbl.add map (sort_str four) 4;
  Hashtbl.add map (sort_str five) 5;
  Hashtbl.add map (sort_str six) 6;
  Hashtbl.add map (sort_str seven) 7;
  Hashtbl.add map (sort_str eight) 8;
  Hashtbl.add map (sort_str nine) 9;
  String.split_on_char ' ' output
  |> List.rev
  |> enumerate
  |> List.iter (fun (digit, number) ->
         let number = sort_str number in
         let order_of_magnitude = BatInt.pow 10 digit in
         value := !value + (order_of_magnitude * Hashtbl.find map number));
  !value

let part2 = List.map decode input |> Util.sum;;

print_string "part1 = ";
Util.println_int part1;
print_string "part2 = ";
Util.println_int part2
