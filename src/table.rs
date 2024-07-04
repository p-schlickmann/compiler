use std::collections::HashMap;

pub fn create_syntactic_table() -> HashMap<&'static str, HashMap<&'static str, &'static str>> {
    let mut syntactic_table: HashMap<&str, HashMap<&str, &str>> = HashMap::new();

    syntactic_table.insert("MAIN", {
        let mut m = HashMap::new();
        m.insert("def", "FLIST");
        m.insert("id", "STMT");
        m.insert("{", "STMT");
        m.insert("int", "STMT");
        m.insert(";", "STMT");
        m.insert("print", "STMT");
        m.insert("return", "STMT");
        m.insert("if", "STMT");
        m.insert("$", "");
        m
    });

    syntactic_table.insert("FLIST", {
        let mut m = HashMap::new();
        m.insert("def", "FDEF FLIST1");
        m
    });

    syntactic_table.insert("FLIST1", {
        let mut m = HashMap::new();
        m.insert("def", "FLIST");
        m.insert("$", "");
        m
    });

    syntactic_table.insert("FDEF", {
        let mut m = HashMap::new();
        m.insert("def", "def id ( PARLIST ) { STMTLIST }");
        m
    });

    syntactic_table.insert("PARLIST", {
        let mut m = HashMap::new();
        m.insert(")", "");
        m.insert("int", "int id PARLIST1");
        m
    });

    syntactic_table.insert("PARLIST1", {
        let mut m = HashMap::new();
        m.insert(")", "");
        m.insert(",", ", PARLIST");
        m
    });

    syntactic_table.insert("STMT", {
        let mut m = HashMap::new();
        m.insert("id", "ATRIBSTMT ;");
        m.insert("{", "{ STMTLIST }");
        m.insert("int", "int id ;");
        m.insert(";", ";");
        m.insert("print", "PRINTSTMT ;");
        m.insert("return", "RETURNSTMT ;");
        m.insert("if", "IFSTMT");
        m
    });

    syntactic_table.insert("ATRIBSTMT", {
        let mut m = HashMap::new();
        m.insert("id", "id ATRIBSTMT1");
        m
    });

    syntactic_table.insert("ATRIBSTMT1", {
        let mut m = HashMap::new();
        m.insert("(", "( PARLISTCALL )");
        m.insert("=", "= EXPR");
        m
    });

    syntactic_table.insert("PARLISTCALL", {
        let mut m = HashMap::new();
        m.insert("id", "id PARLISTCALL1");
        m.insert(")", "");
        m
    });

    syntactic_table.insert("PARLISTCALL1", {
        let mut m = HashMap::new();
        m.insert(")", "");
        m.insert(",", ", PARLISTCALL");
        m
    });

    syntactic_table.insert("PRINTSTMT", {
        let mut m = HashMap::new();
        m.insert("print", "print EXPR");
        m
    });

    syntactic_table.insert("RETURNSTMT", {
        let mut m = HashMap::new();
        m.insert("return", "return");
        m
    });

    syntactic_table.insert("IFSTMT", {
        let mut m = HashMap::new();
        m.insert("if", "if ( EXPR ) STMT IFSTMT1");
        m
    });

    syntactic_table.insert("IFSTMT1", {
        let mut m = HashMap::new();
        m.insert("id", "");
        m.insert("{", "");
        m.insert("}", "");
        m.insert("int", "");
        m.insert(";", "");
        m.insert("print", "");
        m.insert("return", "");
        m.insert("if", "");

        // not sure how this worked? cause the original syntax table had two possible productions for `IFSTMT1`+`else` (it had also epsilon)
        m.insert("else", "else STMT");

        m.insert("$", "");
        m
    });

    syntactic_table.insert("STMTLIST", {
        let mut m = HashMap::new();
        m.insert("id", "STMT STMTLIST1");
        m.insert("{", "STMT STMTLIST1");
        m.insert("int", "STMT STMTLIST1");
        m.insert(";", "STMT STMTLIST1");
        m.insert("print", "STMT STMTLIST1");
        m.insert("return", "STMT STMTLIST1");
        m.insert("if", "STMT STMTLIST1");
        m
    });

    syntactic_table.insert("STMTLIST1", {
        let mut m = HashMap::new();
        m.insert("id", "STMTLIST");
        m.insert("{", "STMTLIST");
        m.insert("}", "");
        m.insert("int", "STMTLIST");
        m.insert(";", "STMTLIST");
        m.insert("print", "STMTLIST");
        m.insert("return", "STMTLIST");
        m.insert("if", "STMTLIST");
        m
    });

    syntactic_table.insert("EXPR", {
        let mut m = HashMap::new();
        m.insert("id", "NUMEXPR EXPR1");
        m.insert("(", "NUMEXPR EXPR1");
        m.insert("num", "NUMEXPR EXPR1");
        m
    });

    syntactic_table.insert("EXPR1", {
        let mut m = HashMap::new();
        m.insert(")", "");
        m.insert(";", "");
        m.insert("<", "< NUMEXPR");
        m.insert(">", "> NUMEXPR");
        m.insert("==", "== NUMEXPR");
        m
    });

    syntactic_table.insert("NUMEXPR", {
        let mut m = HashMap::new();
        m.insert("id", "TERM NUMEXPR1");
        m.insert("(", "TERM NUMEXPR1");
        m.insert("num", "TERM NUMEXPR1");
        m
    });

    syntactic_table.insert("NUMEXPR1", {
        let mut m = HashMap::new();
        m.insert(")", "");
        m.insert(";", "");
        m.insert("<", "");
        m.insert(">", "");
        m.insert("==", "");
        m.insert("+", "+ TERM NUMEXPR1");
        m.insert("-", "- TERM NUMEXPR1");
        m
    });

    syntactic_table.insert("TERM", {
        let mut m = HashMap::new();
        m.insert("id", "FACTOR TERM1");
        m.insert("(", "FACTOR TERM1");
        m.insert("num", "FACTOR TERM1");
        m
    });

    syntactic_table.insert("TERM1", {
        let mut m = HashMap::new();
        m.insert(")", "");
        m.insert(";", "");
        m.insert("<", "");
        m.insert(">", "");
        m.insert("==", "");
        m.insert("+", "");
        m.insert("-", "");
        m.insert("*", "* FACTOR TERM1");
        m
    });

    syntactic_table.insert("FACTOR", {
        let mut m = HashMap::new();
        m.insert("id", "id");
        m.insert("(", "( NUMEXPR )");
        m.insert("num", "num");
        m
    });

    syntactic_table
}