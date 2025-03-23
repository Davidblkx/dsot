/// Creates a new table definition for a redb table
macro_rules! create_table_def {
    ($name: expr) => {
        {
            let table_def: redb::TableDefinition<&[u8], &[u8]> = redb::TableDefinition::new(&$name);
            table_def
        }
    };
}

/// Creates an error Result for a table transaction error
macro_rules! to_trx_err {
    ($name: expr, $err:expr) => {
        |e| DsotError::TransactionError {
            bucket: $name.clone(),
            operation: $err,
            error: e.to_string()
        }
    };
}
