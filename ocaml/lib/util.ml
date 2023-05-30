(* Returns a list containing the first n items of a list. *)
let rec take n list =
  match (n, list) with
  | 0, _ -> []
  | _, [] -> []
  | n, h :: t -> h :: take (n - 1) t

(* Returns a list containing the first n items of a list (tail recursive). *)
let take' n list =
  let rec take_inner n acc list =
    match (n, list) with
    | 0, _ -> acc
    | _, [] -> acc
    | n, h :: tail -> take_inner (n - 1) (h :: acc) tail
  in
  take_inner n [] list |> List.rev

(* Returns a list of lists. Each sub-list is an N-item overlapping slice from the original list. *)
let rec windows n list =
  match (n > List.length list, list) with
  | true, _ -> []
  | false, [] -> []
  | false, _ :: t -> take n list :: windows n t

(* Returns the first n items of a list along with the rest. *)
let chunk n list =
  let rec chunk_inner n acc list =
    match (n, list) with
    | 0, _ -> (List.rev acc, list)
    | _, [] -> (List.rev acc, list)
    | n, h :: tail -> chunk_inner (n - 1) (h :: acc) tail
  in
  chunk_inner n [] list

(* Takes a list and returns a list of lists of consecutive elements from the
   original list. Each sub-list contains at most n elements.. *)
let rec chunks n list =
  match chunk n list with
  | [], _ -> []
  | c, [] -> [ c ]
  | c, rest -> c :: chunks n rest

(* Reads a file and split it into lines. *)
let input_lines filename =
  open_in filename
  |> In_channel.input_all
  |> String.trim
  |> String.split_on_char '\n'

(* Splits a string by a pattern and return both sides or None. *)
let split_once pattern string =
  try Some (BatString.split ~by:pattern string) with Not_found -> None

(* Returns the sum of a list of integers. *)
let sum list = List.fold_left ( + ) 0 list

(* Prints an integer followed by a newline. *)
let println_int i =
  print_int i;
  print_newline ()

(* Splits a list in two, returing a list of the items from the original list
   that, when passed to `pred`, return `true` and another list containing items
   that return `false`. *)
let partition pred list =
  let rec partition' yes no pred list =
    match list with
    | [] -> (List.rev yes, List.rev no)
    | item :: rest ->
        if pred item
        then partition' (item :: yes) no pred rest
        else partition' yes (item :: no) pred rest
  in
  partition' [] [] pred list
