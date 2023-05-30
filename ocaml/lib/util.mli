val take : int -> int list -> int list

val take' : int -> int list -> int list

val windows : int -> 'a list -> 'a list list

val chunk : int -> int list -> int list * int list

val chunks : int -> int list -> int list list

val input_lines : string -> string list

val split_once : string -> string -> (string * string) option

val sum : int list -> int

val println_int : int -> unit

val partition : ('a -> bool) -> 'a list -> 'a list * 'a list
