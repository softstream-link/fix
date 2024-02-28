pub trait MsgType {
    const MSG_TYPE: &'static str;
    #[inline]
    fn msg_type(&self) -> &'static str{
        Self::MSG_TYPE
    }
}



#[cfg(test)]
mod tests{
    use fix_model_test::unittest::setup;
    use log::info;

    use super::*;
    #[test]
    fn test_msg_type(){
        struct Blah{

        }
        impl MsgType for Blah{
            const MSG_TYPE: &'static str = "BLAH";
        }
        setup::log::configure();
        let blah = Blah{};
        info!("msg_type: {:?}", blah.msg_type());
        info!("msg_type: {:?}", Blah::MSG_TYPE);
        assert_eq!(blah.msg_type(), "BLAH");

    }
}