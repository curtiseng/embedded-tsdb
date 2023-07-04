pub struct Database {
    name: String,
    tenant: String,
    user: User,
}

pub struct User {
    username: String,
    password: String,
}

// refs https://www.skyzh.dev/posts/articles/2022-01-24-rust-type-exercise-in-database-executors-middle/
pub struct TableScheme {
    db: Database,
    name: String,
    columns: Vec<TableColumn>,
    tags: Vec<TableTag>,
    timestamp: TimeUnit,
}

pub struct TableColumn {
    pub id: u32,
    pub name: String,
    pub column_type: ValueType,
}

pub struct TableTag {
    name: String,
    value: String,
}

pub enum ValueType {
    Unknown,
    Float,
    Integer,
    Unsigned,
    Boolean,
    String,
}

pub enum TimeUnit {
    /// Time in seconds.
    Second,
    /// Time in milliseconds.
    Millisecond,
    /// Time in microseconds.
    Microsecond,
    /// Time in nanoseconds.
    Nanosecond,
}
