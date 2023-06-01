module Set = Set.Make (struct
  type t = string

  let compare = compare
end)

module Map = Map.Make (struct
  type t = string

  let compare = compare
end)

let input = Util.input_lines "day_12/input.txt"

let insert key value map =
  match Map.find_opt key map with
  | None -> Map.add key [ value ] map
  | Some l -> Map.add key (value :: l) map

(* A Map representing the graph of caves *)
let graph =
  List.fold_left
    (fun map line ->
      let left, right = Util.split_once "-" line |> Option.get in
      let map = insert left right map in
      let map = insert right left map in
      map)
    Map.empty input

let is_lowercase str = str = String.lowercase_ascii str

let try_path part_func target visited =
  let visited = Set.add target visited in
  let paths = Map.find target graph in
  List.fold_left (fun count path -> count + part_func visited path) 0 paths

let rec try_path_part1 visited target =
  if target = "end"
  then 1
  else if is_lowercase target && Set.mem target visited
  then 0
  else try_path try_path_part1 target visited

let rec try_path_part2 visited_small_twice visited target =
  if target = "end"
  then 1
  else if target = "start"
  then 0
  else if is_lowercase target && Set.mem target visited
  then
    if visited_small_twice
    then 0
    else try_path (try_path_part2 true) target visited
  else try_path (try_path_part2 visited_small_twice) target visited
;;

Map.find "start" graph
|> List.map (try_path_part1 (Set.add "start" Set.empty))
|> Util.sum
|> Util.println_int
;;

Map.find "start" graph
|> List.map (try_path_part2 false (Set.add "start" Set.empty))
|> Util.sum
|> Util.println_int
