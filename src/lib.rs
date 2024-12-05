use tokio_postgres::{types::Type, Column};

fn test1(column: Column) {
    match *column.type_() {
        Type::TEXT => {}
        Type::INT4 => {}
        _ => {}
    }
}

fn test2(column: Column) {
    match column.type_() {
        &Type::TEXT => {}
        &Type::INT4 => {}
        _t => {}
    }
}
