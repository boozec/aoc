let input_file = "input.txt"

let read_file f =
    let ic = open_in f in
    try
        let lines = really_input_string ic (in_channel_length ic) in
        let lines = String.trim lines in
        close_in ic ;
        lines
    with e ->
        close_in_noerr ic;
        raise e

let rec parse ?(result=0) body =
    match body with
    | [] -> result
    | x :: tail -> 
        let plays = String.split_on_char ':' x
        |> List.rev
        |> List.hd
        |> String.split_on_char ';'
        |> List.map (String.split_on_char ',')
        |> List.map (List.map @@ String.trim)
        |> List.map (List.map (fun x -> (List.hd (String.split_on_char ' ' x) |> int_of_string, String.get (List.nth (String.split_on_char ' ' x) 1 ) 0)))
        in 


        let points = List.map (List.map (fun (p,c) -> if (c = 'r' && p > 12) || (c = 'g' && p > 13) || (c = 'b' && p > 14) then 0 else 1)) plays in
        let add = if List.length (List.filter (fun x -> List.length x > 0) @@ List.map (List.filter (fun x -> x = 0)) points) = 0 then
            String.split_on_char ':' x |> List.hd |> String.split_on_char ' ' |> List.rev |> List.hd |> int_of_string
        else
            0 in

        parse tail ~result:(result+add)


let () = 
    let result = parse (String.split_on_char '\n' (read_file input_file)) in
    Printf.printf "%d" result
