exception InvalidInput

type command = Forward of int | Down of int | Up of int

let parse_command string =
  let split = String.split_on_char ' ' string in
  match split with
  | [ "forward"; n ] -> Forward (int_of_string n)
  | [ "down"; n ] -> Down (int_of_string n)
  | [ "up"; n ] -> Up (int_of_string n)
  | _ -> raise InvalidInput

let move (x, y) cmd =
  match cmd with
  | Forward n -> (x + n, y)
  | Down n -> (x, y + n)
  | Up n -> (x, y - n)

let aim_and_move ((x, y), aim) cmd =
  match cmd with
  | Forward n -> ((x + n, y + (n * aim)), aim)
  | Down n -> ((x, y), aim + n)
  | Up n -> ((x, y), aim - n)

let commands = Util.input_lines "day_02/input.txt" |> List.map parse_command
let start = (0, 0)
let x, y = List.fold_left move start commands;;

print_string "part1 = ";;
print_endline (string_of_int (x * y))

let aim = 0
let (x, y), _aim = List.fold_left aim_and_move (start, aim) commands;;

print_string "part2 = ";;
print_endline (string_of_int (x * y))
