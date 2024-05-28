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

#[cfg(test)]
mod tests {
    use fix_model_test::unittest::setup;
    use log::info;

    use super::*;
    #[test]
    fn test_msg_type() {
        struct Blah {}
        impl MsgTypeCode for Blah {
            const MSG_TYPE_CODE: &'static str = "BLAH";
            fn is_app(&self) -> bool {
                false
            }
        }
        setup::log::configure();
        let blah = Blah {};
        info!("msg_type: {:?}", blah.msg_type());
        info!("msg_type: {:?}", Blah::MSG_TYPE_CODE);
        assert_eq!(blah.msg_type(), "BLAH");
    }
}
