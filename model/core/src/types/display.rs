pub trait FixByteSlice2Display {
    fn to_string(&self) -> String;
}
impl FixByteSlice2Display for &[u8] {
    fn to_string(&self) -> String {
        String::from_utf8_lossy(self)
            // .replace('\x01', "|")
            // .replace('\x01', "❗")
            .replace('\x01', "🔸") // start of heading  - not show in terminal 🔸  shown in terminal
            .replace('\x09', ".") // horizontal tab
            .replace('\x0A', ".") // line feed
            .replace('\x0B', ".") // vertical tab
            .replace('\x0C', ".") // form feed
            .replace('\x0D', ".") // carriage return
            .to_string()
    }
}
impl FixByteSlice2Display for Vec<u8> {
    fn to_string(&self) -> String {
        format!("{}", self.as_slice().to_string())
        // let v = self.as_slice(); // TODO why does this not work
        // v.to_string()
    }
}
impl FixByteSlice2Display for Option<&[u8]> {
    fn to_string(&self) -> String {
        match self {
            Some(slice) => format!("Some({})", slice.to_string()),
            None => "None".to_owned(),
        }
    }
}
impl<const N: usize> FixByteSlice2Display for [u8; N] {
    fn to_string(&self) -> String {
        self.as_ref().to_string()
    }
}
