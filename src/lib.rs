pub trait ToString {
    fn to_string(&self) -> String;
    fn to_str(&self) -> &'static str;
}