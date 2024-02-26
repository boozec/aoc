let input_file = "input.txt"

let read_file f =
  let ic = open_in f in
  try
    let lines = really_input_string ic (in_channel_length ic) |> String.trim in
    close_in ic;
    lines
  with e ->
    close_in_noerr ic;
    raise e

let nums_from_string str =
  String.split_on_char ' ' str
  |> List.filter (fun ch -> String.length ch > 0)
  |> List.map int_of_string

let rec parse ?(result = 0) lines =
  match lines with
  | [] -> result
  | x :: tail ->
      let numbers =
        List.nth (String.split_on_char ':' x) 1
        |> String.split_on_char '|' |> List.map String.trim
        |> List.map nums_from_string
      in

      let winnings = List.nth numbers 0 in
      let plays = List.nth numbers 1 in

      let k =
        Int.shift_left 1
          (List.length
             (List.map
                (fun win -> if List.mem win plays = true then 1 else 0)
                winnings
             |> List.filter (fun i -> i = 1))
          - 1)
      in

      parse tail ~result:(result + k)

let () =
  let result = read_file input_file |> String.split_on_char '\n' |> parse in
  Printf.printf "%d\n" result
