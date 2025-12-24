# Part 1

Per risolverlo basta vedere se si dividono a metà le stringhe e se le metà combaciano.
Se lo fanno, allora hai trovato il numero, altrimenti si skippa.

In parse_range:

Dato che split_once restituisce un `Option<(&str, &str)>`, `ok_or_else(|| format!("Invalid range: {range}"))` la converte in un `Result<(&str, &str), String>`:

- Some(x) diventa Ok(x)

- None diventa Err("Invalid range: ...".to_string()) (il messaggio viene creato solo se serve, grazie alla closure || ...)

Il ? dice: “se è Ok, continua e dammi il valore; se è Err, ritorna subito dalla funzione con quell’errore”.

# Part 2

Bisogna trovare i numeri che hanno una ripetizione al loro interno, è il problema della substringa ripetuta, vedi (qui)[https://algo.monster/liteproblems/459].

La risoluzione si ottiene vedendo se, concatenando il numero per se stesso, si trova una ripetizione all'interno della nuova stringa generata. L'eventuale stringa non può uscire dai confini della strina e non si contano la prima e l'ultima occorrenza.
