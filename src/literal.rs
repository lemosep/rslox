/// Since Rust doesn't allow multiple type
/// variables (for obvious reasons), this enum
/// gives the possible values for each EXISTABLE value.
#[derive(Clone, Debug)]
pub enum Value {
    Str{ v: String },
    Int{ v: i32 },
    Fl{ v: f64 }
}