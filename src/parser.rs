use crate::table::create_syntactic_table;
use prettytable::{Table, row};

pub fn parse(input_str: &str) {
    let mut input: Vec<&str> = input_str.split_whitespace().collect();
    let syntactic_table = create_syntactic_table();

    // Extrair lista de não terminais da tabela sintática
    let non_terminals: Vec<&str> = syntactic_table.keys().cloned().collect();

    fn is_terminal(token: &str, non_terminals: &Vec<&str>) -> bool {
        !non_terminals.contains(&token)
    }

    fn print_table(table: &Vec<TableRow>) {
        let mut table_output = Table::new();
        table_output.add_row(row!["Matched", "Stack", "Input", "Action"]);

        for row in table {
            table_output.add_row(row![
            &row.matched,
            &row.stack,
            &row.input,
            &row.action,
        ]);
        }

        table_output.printstd();
    }


    let mut stack = vec!["MAIN", "$"];

    let mut X = stack[0];
    let mut a = input[0];
    let mut matched: Vec<&str> = Vec::new();

    #[derive(Debug)]
    struct TableRow {
        matched: String,
        stack: String,
        input: String,
        action: String,
    }

    fn generate_printable_row(matched: &Vec<&str>, stack: &Vec<&str>, input: &Vec<&str>, action: &str) -> TableRow {
        TableRow {
            matched: matched.join(" "),
            stack: stack.join(" "),
            input: input.join(" "),
            action: action.to_string(),
        }
    }

    let mut table_rows = vec![generate_printable_row(&matched, &stack, &input, "")];

    while X != "$" {
        if X == a {
            table_rows.push(generate_printable_row(&matched, &stack, &input, &format!("match {}", X)));
            matched.push(a);
            stack.remove(0);
            input.remove(0);
            a = input[0];
            X = stack[0];
        } else if is_terminal(X, &non_terminals) {
            print_table(&table_rows);
            panic!("⚠️  Syntax Error: {} is a terminal without a match", X);
        } else if !syntactic_table[X].contains_key(a) {
            print_table(&table_rows);
            panic!("⚠️  Syntax Error: {}:{} not in parse table", X, a);
        } else {
            let output = syntactic_table[X][a];
            table_rows.push(generate_printable_row(&matched, &stack, &input, &format!("{} -> {}", X, output)));
            stack.remove(0);
            if !output.is_empty() {
                for sym in output.split_whitespace().rev() {
                    stack.insert(0, sym);
                }
            }
            X = stack[0];
        }
    }

    print_table(&table_rows);

    if a == "$" {
        println!("✅ Parsed successfully!\n");
        println!("Matches: \"{}\"\n", matched.join(" "));
    } else {
        println!("⚠️ Parsed partially...");
        println!("Matches: \"{}\"", matched.join(" "));
        println!("Rest: \"{}\"\n", input.join(" "));
    }
}