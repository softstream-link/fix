pub trait MsgTypeCode {
    const MSG_TYPE_CODE: &'static str;
    #[inline]
    fn msg_type(&self) -> &'static str {
        Self::MSG_TYPE_CODE
    }
    fn is_app(&self) -> bool;
    #[inline]
    fn is_adm(&self) -> bool {
        !self.is_app()
    }
}
pub trait Header {}

