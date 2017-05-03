pub trait StringIO {
    type N;
    fn from_char(input: char) -> Result<Self::N, String>;
    fn to_char(&self) -> char;
}
