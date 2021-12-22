pub enum Value {
    Boolean(bool),
    Byte(u8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    Word16(u16),
    Word32(u32),
    Word64(u64),
    Double(f64),
    Str(String),
    Vec(Vec<Value>)
}