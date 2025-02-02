use crate::any::{
    AnyArgumentBuffer, AnyArguments, AnyColumn, AnyConnection, AnyQueryResult, AnyRow,
    AnyStatement, AnyTransactionManager, AnyTypeInfo, AnyValue, AnyValueRef,
};
use crate::database::{Database, HasArguments, HasStatement, HasStatementCache, HasValueRef};
use std::marker::PhantomData;

/// Opaque database driver. Capable of being used in place of any SQLx database driver. The actual
/// driver used will be selected at runtime, from the connection url.
#[derive(Debug)]
pub struct Any;

impl Database for Any {
    type Connection = AnyConnection;

    type TransactionManager = AnyTransactionManager;

    type Row = AnyRow;

    type QueryResult = AnyQueryResult;

    type Column = AnyColumn;

    type TypeInfo = AnyTypeInfo;

    type Value = AnyValue;
    const NAME: &'static str = "Any";

    const URL_SCHEMES: &'static [&'static str] = &[];
}

impl<'r> HasValueRef<'r> for Any {
    type Database = Any;

    type ValueRef = AnyValueRef<'r>;
}

impl<'q> HasStatement<'q> for Any {
    type Database = Any;

    type Statement = AnyStatement<'q>;
}

impl<'q> HasArguments<'q> for Any {
    type Database = Any;

    type Arguments = AnyArguments<'q>;

    type ArgumentBuffer = AnyArgumentBuffer<'q>;
}

// This _may_ be true, depending on the selected database
impl HasStatementCache for Any {}
