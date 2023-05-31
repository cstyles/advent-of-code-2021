type validity = Valid | Incomplete of char list | Corrupt of char

let input = Util.input_lines "day_10/input.txt"

let rec check_validity stack line =
  match (line, stack) with
  (* No more characters and stack is empty *)
  | [], [] -> Valid
  (* No more characters but stack still has unclosed delimiters *)
  | [], stack -> Incomplete stack
  (* Opening characters -> push on stack and keep going *)
  | '(' :: rest_of_line, _ -> check_validity ('(' :: stack) rest_of_line
  | '[' :: rest_of_line, _ -> check_validity ('[' :: stack) rest_of_line
  | '{' :: rest_of_line, _ -> check_validity ('{' :: stack) rest_of_line
  | '<' :: rest_of_line, _ -> check_validity ('<' :: stack) rest_of_line
  (* Closing characters and top of stack matches -> keep going *)
  | ')' :: line, '(' :: rest_of_stack -> check_validity rest_of_stack line
  | ']' :: line, '[' :: stack -> check_validity stack line
  | '}' :: line, '{' :: stack -> check_validity stack line
  | '>' :: line, '<' :: stack -> check_validity stack line
  (* Anything else (e.g., bad character, closing delimiter didn't match top
     of stack, etc.) -> Corrupt *)
  | c :: _, _ -> Corrupt c

let check line = check_validity [] (BatString.explode line)
let extract_corrupt = function Corrupt c -> Some c | _ -> None
let extract_incomplete = function Incomplete c -> Some c | _ -> None

let score_part1 = function
  | ')' -> 3
  | ']' -> 57
  | '}' -> 1_197
  | '>' -> 25_137
  | _ -> failwith "unreachable"

let validities = List.map check input

let part1 =
  validities
  |> List.filter_map extract_corrupt
  |> List.map score_part1
  |> Util.sum

let part2_score_lookup = function
  | '(' -> 1
  | '[' -> 2
  | '{' -> 3
  | '<' -> 4
  | _ -> failwith "unreachable"

let score_part2 list =
  List.fold_left (fun acc elm -> (acc * 5) + part2_score_lookup elm) 0 list

let part2_scores =
  validities
  |> List.filter_map extract_incomplete
  |> List.map score_part2
  |> BatList.fast_sort compare

let part2 = List.nth part2_scores (List.length part2_scores / 2);;

print_string "part1 = ";
Util.println_int part1;
print_string "part2 = ";
Util.println_int part2
