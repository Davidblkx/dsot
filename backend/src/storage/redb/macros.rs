macro_rules! create_table_def {
    ($name: expr) => {
        {
            let table_def: redb::TableDefinition<&[u8], &[u8]> = redb::TableDefinition::new($name);
            table_def
        }
    };
}

macro_rules! to_trx_err {
    ($table_name: expr, $err:expr) => {
        |e| DsotError::TableTransactionError {
            table: $table_name,
            operation: $err,
            error: e.to_string()
        }
    };
}
