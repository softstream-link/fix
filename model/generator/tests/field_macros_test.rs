use fix_model_core::{
    prelude::{asc, Ascii},
    types::{asciichar::aschar, dat::dat, dat_codec::dat_codec, data::Data},
};
use fix_model_generator::{
    fix_u8_fixed_length, fix_usize_fixed_length,
    prelude::{fix_ascii_char_enum, fix_bool, fix_char_any, fix_data, fix_string, fix_usize},
};
use fix_model_test::unittest::setup;
use fix_serde::unittest::{from_slice_unittest, to_bytes_unittest};
use log::info;

use serde_json::{from_str, to_string};

#[test]
fn test_data_dat_codec_dat() {
    setup::log::configure();

    fix_data!(RawDataLength, 95, RawData, 96);

    // &dat
    let _inp = RawData::new(dat::from_slice(b"\x00BIN"));
    let _inp = RawData::new(dat::from_slice(&vec![1, 2]));
    let _inp: RawData<&dat> = b"BIN".as_slice().into();
    let _inp: RawData<&dat> = b"BIN".into();
    let _own = _inp.to_owned_inner_if_ref();
    let _borrow = _inp.borrow_inner_if_allocated();
    let _ref = _inp.as_ref();
    let _dat = _inp.as_dat();
    let _inner = _inp.into_inner();

    let _inp = RawData::new(Data::from_slice(b"\x00BIN"));
    let _inp = RawData::new(Data::from_slice(&vec![1, 2]));
    let _inp = RawData::new(Data::from_vec(vec![1, 2]));
    let _inp: RawData<Data> = vec![1, 2].into();
    let _inp: RawData<Data> = b"BIN".into();
    let _own = _inp.to_owned();
    let _borrow = _inp.borrow_inner_if_allocated();
    let _ref = _inp.as_ref();
    let _dat = _inp.as_dat();
    let _inner = _inp.into_inner();

    let _inp = RawData::new(dat_codec::from_slice(b"\x00BIN"));
    let _own = _inp.to_owned_inner_if_ref();
    let _borrow = _inp.borrow_inner_if_allocated();
    let _dat = _inp.as_dat();
    let _inner = _inp.into_inner();

    let inp = RawData::new(Data::from_slice(b"\x00BIN"));

    let debug_ = format!("{:?}", inp);
    info!("debug_: {}", debug_);
    assert_eq!(debug_, r#"RawData:96([00, 42, 49, 4E])"#);
    let debug_p = format!("{:+?}", inp);
    info!("debug_p: {}", debug_p);
    assert_eq!(debug_p, r#"96([00, 42, 49, 4E])"#);
    let debug_m = format!("{:-?}", inp);
    info!("debug_m: {}", debug_m);
    assert_eq!(debug_m, r#"RawData([00, 42, 49, 4E])"#);

    let display_ = format!("{}", inp);
    info!("display_: {}", display_);
    assert_eq!(display_, "\0BIN");
    let display_p = format!("{:+}", inp);
    info!("display_p: {}", display_p);
    assert_eq!(display_p, "95=4|96=\0BIN");
    let display_m = format!("{:-}", inp);
    info!("display_m: {}", display_m);
    assert_eq!(display_m, "RawDataLength=4|RawData=\0BIN");
}

#[test]
fn test_fix_string_str_ascii_asc() {
    setup::log::configure();

    fix_string!(Account, 1);

    let _inp = Account::new("ABC");
    let _inp: Account<_> = "ABC".into();
    let _inp: Account<&str> = "ABC".into();
    let _inp: Account<&str> = Default::default();

    let _own = _inp.to_owned_inner_if_ref();
    let _borrow = _inp.borrow_inner_if_allocated();
    let _ref = _inp.as_ref();
    let _str = _inp.as_str();
    let _inner = _inp.into_inner();
    info!("Account<&str>::default '{}'", Account::<&str>::default());

    // String
    let _inp = Account::new("ABC".to_owned());
    let _inp: Account<String> = Account::new("ABC".into());
    let _inp: Account<String> = "ABC".to_owned().into();
    let _inp: Account<String> = Default::default();

    let _own = _inp.to_owned();
    let _borrow = _inp.borrow_inner_if_allocated();
    let _ref = _inp.as_ref();
    let _inner = _inp.into_inner();
    info!("Account<String>::default '{}'", Account::<String>::default());

    // &asc
    let _inp = Account::new(asc::try_from_str("ABC").unwrap());
    let _inp: Account<&asc> = Account::new("ABC".try_into().unwrap());
    let _inp: Account<&asc> = "ABC".try_into().unwrap();
    let _inp: Account<&asc> = b"ABC".try_into().unwrap();
    let _inp: Account<&asc> = Default::default();

    let _own = _inp.to_owned_inner_if_ref();
    let _borrow = _inp.borrow_inner_if_allocated();
    let _ref = _inp.as_ref();
    let _str = _inp.as_str();
    let _inner = _inp.into_inner();
    info!("Account<&asc>::default '{}'", Account::<&asc>::default());

    // Ascii
    let _inp: Account<Ascii> = Account::new(Ascii::try_from("ABC".to_owned()).unwrap());
    let _inp: Account<Ascii> = "ABC".to_owned().try_into().unwrap();
    let _inp: Account<Ascii> = Default::default();

    let _own = _inp.to_owned_inner_if_ref();
    let _borrow = _inp.borrow_inner_if_allocated();
    let _ref = _inp.as_ref();
    let _str = _inp.as_str();
    let _inner = _inp.into_inner();
    info!("Account<Ascii>::default '{}'", Account::<Ascii>::default());

    let inp = Account::new("ABC".to_owned());
    let debug_ = format!("{:?}", inp);
    info!("debug_: {}", debug_);
    assert_eq!(debug_, r#"Account:1("ABC")"#);
    let debug_p = format!("{:+?}", inp);
    info!("debug_p: {}", debug_p);
    assert_eq!(debug_p, r#"1("ABC")"#);
    let debug_m = format!("{:-?}", inp);
    info!("debug_m: {}", debug_m);
    assert_eq!(debug_m, r#"Account("ABC")"#);

    let display_ = format!("{}", inp);
    info!("display_: {}", display_);
    assert_eq!(display_, "ABC");
    let display_p = format!("{:+}", inp);
    info!("display_p: {}", display_p);
    assert_eq!(display_p, "1=ABC");
    let display_m = format!("{:-}", inp);
    info!("display_m: {}", display_m);
    assert_eq!(display_m, "Account=ABC");

    let fix_ser = to_bytes_unittest(&inp).unwrap();
    info!("fix_ser: {}", fix_ser);
    let jsn_ser = to_string(&inp).unwrap();
    info!("jsn_ser: {}", jsn_ser);

    info!("Account<String>");
    let fix_out = from_slice_unittest::<Account<String>>(&fix_ser).unwrap();
    let jsn_out = serde_json::from_str::<Account<String>>(&jsn_ser).unwrap();
    info!("jsn_out: {:?}", jsn_out);
    assert_eq!(jsn_out, fix_out);
    assert_eq!(fix_out.value(), "ABC");

    info!("Account<&str>");
    let fix_out = from_slice_unittest::<Account<&str>>(&fix_ser).unwrap();
    info!("fix_out: {:?}", fix_out);
    let jsn_out = from_str::<Account<&str>>(&jsn_ser).unwrap();
    info!("jsn_out: {:?}", jsn_out);
    assert_eq!(jsn_out, fix_out);
    assert_eq!(fix_out.value(), "ABC");

    info!("Account<Ascii>");
    let fix_out = from_slice_unittest::<Account<Ascii>>(&fix_ser).unwrap();
    info!("fix_out: {:?}", fix_out);
    let jsn_out = from_str::<Account<Ascii>>(&jsn_ser).unwrap();
    info!("jsn_out: {:?}", jsn_out);
    assert_eq!(jsn_out, fix_out);
    assert_eq!(fix_out.value(), asc::try_from_str("ABC").unwrap());

    info!("Account<&asc>");
    let fix_out = from_slice_unittest::<Account<&asc>>(&fix_ser).unwrap();
    info!("fix_out: {:?}", fix_out);
    let jsn_out = from_str::<Account<&asc>>(&jsn_ser).unwrap();
    info!("jsn_out: {:?}", jsn_out);
    assert_eq!(jsn_out, fix_out);
    assert_eq!(fix_out.value(), asc::try_from_slice(b"ABC").unwrap());
}

#[test]
fn test_char_any() {
    setup::log::configure();

    fix_char_any!(IOIOthSvc, 24);

    let _inp = IOIOthSvc::new('A');
    let _inp = IOIOthSvc::<char>::new('A');
    let _inp: IOIOthSvc<char> = 'A'.into();
    let _inner = _inp.into_inner();
    let _inp: IOIOthSvc<char> = Default::default();

    let _inp = IOIOthSvc::new(aschar::try_from('A').unwrap());
    let _inp = IOIOthSvc::<aschar>::new('A'.try_into().unwrap());
    let _inp: IOIOthSvc<aschar> = 'A'.try_into().unwrap();
    let _inp: IOIOthSvc<aschar> = b'A'.try_into().unwrap();
    let _inner = _inp.into_inner();
    let _inp: IOIOthSvc<aschar> = Default::default();

    let inp: IOIOthSvc<char> = 'A'.into();
    assert_eq!(inp.value(), 'A');

    let debug_ = format!("{:?}", inp);
    info!("debug_: {}", debug_);
    assert_eq!(debug_, "IOIOthSvc:24('A')");

    let debug_p = format!("{:+?}", inp);
    info!("debug_p: {}", debug_p);
    assert_eq!(debug_p, "24('A')");

    let debug_m = format!("{:-?}", inp);
    info!("debug_m: {}", debug_m);
    assert_eq!(debug_m, "IOIOthSvc('A')");

    let display_ = format!("{}", inp);
    info!("display_: {}", display_);
    assert_eq!(display_, "A");

    let display_p = format!("{:+}", inp);
    info!("display_p: {}", display_p);
    assert_eq!(display_p, "24=A");

    let display_m = format!("{:-}", inp);
    info!("display_m: {}", display_m);
    assert_eq!(display_m, "IOIOthSvc=A");

    let fix_ser = to_bytes_unittest(&inp).unwrap();
    info!("fix_ser: {}", fix_ser);
    let jsn_ser = serde_json::to_string(&inp).unwrap();
    info!("jsn_ser: {}", jsn_ser);

    let fix_out = from_slice_unittest::<IOIOthSvc<aschar>>(&fix_ser).unwrap();
    info!("fix_out: {:?}", fix_out);
    let jsn_out = serde_json::from_str::<IOIOthSvc<aschar>>(&jsn_ser).unwrap();
    info!("jsn_out: {:?}", jsn_out);
    assert_eq!(jsn_out, fix_out);
}

#[test]
fn test_char_enum() {
    setup::log::configure();

    fix_ascii_char_enum!(AdvSide, 4, Buy: "B", Sell: "S", Trade: "T", Cross: "X" );
    let inp = AdvSide::Buy;
    assert_eq!(inp.value(), 'B');

    let debug_ = format!("{:?}", inp);
    info!("debug_: {}", debug_);
    assert_eq!(debug_, "AdvSide:4(Buy:'B')");

    let debug_p = format!("{:+?}", inp);
    info!("debug_p: {}", debug_p);
    assert_eq!(debug_p, "4('B')");

    let debug_m = format!("{:-?}", inp);
    info!("debug_m: {}", debug_m);
    assert_eq!(debug_m, "AdvSide('Buy')");

    let display_ = format!("{}", inp);
    info!("display_: {}", display_);
    assert_eq!(display_, "B");

    let display_p = format!("{:+}", inp);
    info!("display_p: {}", display_p);
    assert_eq!(display_p, "4=B");

    let display_m = format!("{:-}", inp);
    info!("display_m: {}", display_m);
    assert_eq!(display_m, "AdvSide=B");

    let fix_ser = to_bytes_unittest(&inp).unwrap();
    info!("fix_ser: {}", fix_ser);
    let jsn_ser = serde_json::to_string(&inp).unwrap();
    info!("jsn_ser: {}", jsn_ser);

    let fix_out = from_slice_unittest::<AdvSide>(&fix_ser).unwrap();
    info!("fix_out: {:?}", fix_out);
    let jsn_out = serde_json::from_str::<AdvSide>(&jsn_ser).unwrap();
    info!("jsn_out: {:?}", jsn_out);
    assert_eq!(jsn_out, fix_out);
}

#[test]
fn test_fix_u8_fixed_len() {
    setup::log::configure();

    // 88::MAX 255 len 3
    fix_u8_fixed_length!(CheckSum, 10);
    let _inp = CheckSum::new(2);
    let inp: CheckSum = 2.into();

    let fix_ser = to_bytes_unittest(&inp).unwrap();
    info!("fix_ser: {}", fix_ser);
    let jsn_ser = to_string(&inp).unwrap();
    info!("jsn_ser: {}", jsn_ser);

    let fix_out = from_slice_unittest::<CheckSum>(&fix_ser).unwrap();
    let jsn_out = serde_json::from_str::<CheckSum>(&jsn_ser).unwrap();
    assert_eq!(jsn_out, fix_out);
    assert_eq!(jsn_out.value(), 2);
}
#[test]
fn test_fix_usize_fixed_len() {
    setup::log::configure();

    // usize::MAX 18446744073709551615 len 20
    fix_usize_fixed_length!(BodyLength, 9);
    let _inp = BodyLength::new(2);
    let inp: BodyLength = usize::MAX.into();
    assert_eq!(inp.value(), 18446744073709551615);

    let debug_ = format!("{:?}", inp);
    info!("debug_: {}", debug_);
    assert_eq!(debug_, "BodyLength:9(18446744073709551615)");

    let debug_p = format!("{:+?}", inp);
    info!("debug_p: {}", debug_p);
    assert_eq!(debug_p, "9(18446744073709551615)");

    let debug_m = format!("{:-?}", inp);
    info!("debug_m: {}", debug_m);
    assert_eq!(debug_m, "BodyLength(18446744073709551615)");

    let display_ = format!("{}", inp);
    info!("display_: {}", display_);
    assert_eq!(display_, "18446744073709551615");

    let display_p = format!("{:+}", inp);
    info!("display_p: {}", display_p);
    assert_eq!(display_p, "9=18446744073709551615");

    let display_m = format!("{:-}", inp);
    info!("display_m: {}", display_m);
    assert_eq!(display_m, "BodyLength=18446744073709551615");

    let fix_ser = to_bytes_unittest(&inp).unwrap();
    info!("fix_ser: {}", fix_ser);
    let jsn_ser = to_string(&inp).unwrap();
    info!("jsn_ser: {}", jsn_ser);

    let fix_out = from_slice_unittest::<BodyLength>(&fix_ser).unwrap();
    let jsn_out = serde_json::from_str::<BodyLength>(&jsn_ser).unwrap();
    info!("jsn_out: {:?}", jsn_out);
    assert_eq!(jsn_out, fix_out);
}

#[test]
fn test_fix_usize() {
    setup::log::configure();

    fix_usize!(BeginSeqNo, 7);
    let _inp = BeginSeqNo::new(2);
    let inp: BeginSeqNo = 2.into();
    assert_eq!(inp.value(), 2);

    let debug_ = format!("{:?}", inp);
    info!("debug_: {}", debug_);
    assert_eq!(debug_, "BeginSeqNo:7(2)");

    let debug_p = format!("{:+?}", inp);
    info!("debug_p: {}", debug_p);
    assert_eq!(debug_p, "7(2)");

    let debug_m = format!("{:-?}", inp);
    info!("debug_m: {}", debug_m);
    assert_eq!(debug_m, "BeginSeqNo(2)");

    let display_ = format!("{}", inp);
    info!("display_: {}", display_);
    assert_eq!(display_, "2");

    let display_p = format!("{:+}", inp);
    info!("display_p: {}", display_p);
    assert_eq!(display_p, "7=2");

    let display_m = format!("{:-}", inp);
    info!("display_m: {}", display_m);
    assert_eq!(display_m, "BeginSeqNo=2");

    let fix_ser = to_bytes_unittest(&inp).unwrap();
    info!("fix_ser: {}", fix_ser);
    let jsn_ser = to_string(&inp).unwrap();
    info!("jsn_ser: {}", jsn_ser);

    let fix_out = from_slice_unittest::<BeginSeqNo>(&fix_ser).unwrap();
    let jsn_out = serde_json::from_str::<BeginSeqNo>(&jsn_ser).unwrap();
    info!("jsn_out: {:?}", jsn_out);
    assert_eq!(jsn_out, fix_out);
}

#[test]
fn test_fix_bool() {
    setup::log::configure();

    fix_bool!(PossDupFlag, 43);

    let _inp = PossDupFlag::new(false);
    let inp: PossDupFlag = false.into();
    assert_eq!(inp.value(), false);

    let debug_ = format!("{:?}", inp);
    info!("debug_: {}", debug_);
    assert_eq!(debug_, "PossDupFlag:43(false)");

    let debug_p = format!("{:+?}", inp);
    info!("debug_p: {}", debug_p);
    assert_eq!(debug_p, "43(false)");

    let debug_m = format!("{:-?}", inp);
    info!("debug_m: {}", debug_m);
    assert_eq!(debug_m, "PossDupFlag(false)");

    let display_ = format!("{}", inp);
    info!("display_: {}", display_);
    assert_eq!(display_, "N");

    let display_p = format!("{:+}", inp);
    info!("display_p: {}", display_p);
    assert_eq!(display_p, "43=N");

    let display_m = format!("{:-}", inp);
    info!("display_m: {}", display_m);
    assert_eq!(display_m, "PossDupFlag=N");

    let fix_ser = to_bytes_unittest(&inp).unwrap();
    info!("fix_ser: {}", fix_ser);
    let jsn_ser = serde_json::to_string(&inp).unwrap();
    info!("jsn_ser: {}", jsn_ser);

    let fix_out = from_slice_unittest::<PossDupFlag>(&fix_ser).unwrap();
    info!("fix_out: {:?}", fix_out);
    let jsn_out = serde_json::from_str::<PossDupFlag>(&jsn_ser).unwrap();
    info!("jsn_out: {:?}", jsn_out);
    assert_eq!(jsn_out, fix_out);
}

#[test]
fn test_example_msg_to_owned() {
    fix_string!(Account, 1);
    fix_char_any!(IOIOthSvc, 24);
    fix_usize!(BeginSeqNo, 7);
    fix_data!(RawDataLength, 95, RawData, 96);

    #[derive(Debug, Clone)]
    struct RepGrp {
        begin_seq_no: BeginSeqNo,
    }
    impl RepGrp {
        fn to_owned_inner_if_ref(&self) -> RepGrp {
            RepGrp {
                begin_seq_no: self.begin_seq_no,
            }
        }
    }
    #[derive(Debug, Clone)]
    struct SomeMsg<S, C, D,> {
        account: Account<S>,
        account_optional: Option<Account<S>>,
        ioioth_svc: IOIOthSvc<C>,
        begin_seq_no: BeginSeqNo,
        raw_data: RawData<D>,
        raw_data_optional: Option<RawData<D>>,
        rep_grp: Vec<RepGrp>,
    }
    // impl<S, C, D> SomeMsg<S, C, D> {
    //     fn to_owned_inner_if_ref(&self) -> SomeMsg<String, char, Data> {
    impl SomeMsg<&str, char, &dat,> {
        fn to_owned_inner_if_ref(&self) -> SomeMsg<String, char, Data> {
            SomeMsg {
                account: self.account.to_owned_inner_if_ref(),
                account_optional: match &self.account_optional {
                    Some(account) => Some(account.to_owned_inner_if_ref()),
                    None => None,
                },
                ioioth_svc: self.ioioth_svc,
                begin_seq_no: self.begin_seq_no,
                raw_data: self.raw_data.to_owned_inner_if_ref(),
                raw_data_optional: match &self.raw_data_optional {
                    Some(raw_data) => Some(raw_data.to_owned_inner_if_ref()),
                    None => None,
                },
                rep_grp: self.rep_grp.iter().map(|rep_grp| rep_grp.to_owned_inner_if_ref()).collect(),
            }
        }
    }
    // impl SomeMsg<String, char, Data> {
    //     fn to_owned_inner_if_ref(&self) -> SomeMsg<String, char, Data> {
    //         SomeMsg {
    //             account: self.account.to_owned_inner_if_ref(),
    //             account_optional: match &self.account_optional {
    //                 Some(account) => Some(account.to_owned_inner_if_ref()),
    //                 None => None,
    //             },
    //             ioioth_svc: self.ioioth_svc,
    //             begin_seq_no: self.begin_seq_no,
    //             raw_data: self.raw_data.to_owned_inner_if_ref(),
    //             raw_data_optional: match &self.raw_data_optional {
    //                 Some(raw_data) => Some(raw_data.to_owned_inner_if_ref()),
    //                 None => None,
    //             },
    //             rep_grp: self.rep_grp.iter().map(|rep_grp| rep_grp.to_owned_inner_if_ref()).collect(),
    //         }
    //     }
    // }
    let _inp = SomeMsg {
        account: Account::new("ABC"),
        account_optional: Some(Account::new("ABC")),
        ioioth_svc: IOIOthSvc::new('A'),
        begin_seq_no: BeginSeqNo::new(2),
        raw_data: RawData::new(dat::from_slice(b"\x00BIN")),
        raw_data_optional: Some(RawData::new(&dat::from_slice(b"\x00BIN"))),
        rep_grp: vec![RepGrp {
            begin_seq_no: BeginSeqNo::new(2),
        }],
    };

    let _own = _inp.to_owned();
    let _own = _inp.clone();
    let _own = _inp.to_owned_inner_if_ref();

    let _inp = SomeMsg {
        account: Account::new("ABC".to_owned()),
        account_optional: Some(Account::new("ABC".to_owned())),
        ioioth_svc: IOIOthSvc::new('A'),
        begin_seq_no: BeginSeqNo::new(2),
        raw_data: RawData::new(Data::from_vec(b"\x00BIN".to_vec())),
        raw_data_optional: Some(RawData::new(Data::from_vec(b"\x00BIN".to_vec()))),
        rep_grp: vec![RepGrp {
            begin_seq_no: BeginSeqNo::new(2),
        }],
    };

    // let _own = _inp.to_owned_inner_if_ref();
    let _own = _inp.clone();
    let _own = _inp.to_owned();
}


// impl Advertisement < & fix_model_core :: prelude :: asc , fix_model_core :: prelude :: aschar , & fix_model_core :: prelude :: dat , > {
//      pub fn to_owned_inner_if_ref (& self) -> Advertisement < fix_model_core :: prelude :: Ascii , fix_model_core :: prelude :: aschar , fix_model_core :: prelude :: Data , > { 
//         Advertisement < fix_model_core :: prelude :: Ascii , fix_model_core :: prelude :: aschar , fix_model_core :: prelude :: Data , > {
//              r#adv_id : self . r#adv_id . to_owned_inner_if_ref () , r#adv_trans_type : self . r#adv_trans_type . to_owned_inner_if_ref () , r#adv_ref_id : match & self . r#adv_ref_id { Some (v) => Some (v . to_owned_inner_if_ref ()) , None => None , } , r#symbol : match & self . r#symbol { Some (v) => Some (v . to_owned_inner_if_ref ()) , None => None , } , r#symbol_sfx : match & self . r#symbol_sfx { Some (v) => Some (v . to_owned_inner_if_ref ()) , None => None , } , r#security_id : match & self . r#security_id { Some (v) => Some (v . to_owned_inner_if_ref ()) , None => None , } , r#security_id_source : match & self . r#security_id_source { Some (v) => Some (v . to_owned_inner_if_ref ()) , None => None , } , r#sec_alt_id_grp : match & self . r#sec_alt_id_grp { Some (v) => Some (v . iter () . map (| rep_grp | rep_grp . to_owned_inner_if_ref ()) . collect ()) , None => None , } , r#product : match & self . r#product { Some (v) => Some (v) , None => None , } , r#cfi_code : match & self . r#cfi_code { Some (v) => Some (v . to_owned_inner_if_ref ()) , None => None , } , r#security_type : match & self . r#security_type { Some (v) => Some (v . to_owned_inner_if_ref ()) , None => None , } , r#security_sub_type : match & self . r#security_sub_type { Some (v) => Some (v . to_owned_inner_if_ref ()) , None => None , } , r#maturity_month_year : match & self . r#maturity_month_year { Some (v) => Some (v . to_owned_inner_if_ref ()) , None => None , } , r#maturity_date : match & self . r#maturity_date { Some (v) => Some (v . to_owned_inner_if_ref ()) , None => None , } , r#put_or_call : match & self . r#put_or_call { Some (v) => Some (v) , None => None , } , r#coupon_payment_date : match & self . r#coupon_payment_date { Some (v) => Some (v . to_owned_inner_if_ref ()) , None => None , } , r#issue_date : match & self . r#issue_date { Some (v) => Some (v . to_owned_inner_if_ref ()) , None => None , } , r#repo_collateral_security_type : match & self . r#repo_collateral_security_type { Some (v) => Some (v . to_owned_inner_if_ref ()) , None => None , } , r#repurchase_term : match & self . r#repurchase_term { Some (v) => Some (v) , None => None , } , r#repurchase_rate : match & self . r#repurchase_rate { Some (v) => Some (v) , None => None , } , r#factor : match & self . r#factor { Some (v) => Some (v) , None => None , } , r#credit_rating : match & self . r#credit_rating { Some (v) => Some (v . to_owned_inner_if_ref ()) , None => None , } , r#instr_registry : match & self . r#instr_registry { Some (v) => Some (v . to_owned_inner_if_ref ()) , None => None , } , r#country_of_issue : match & self . r#country_of_issue { Some (v) => Some (v . to_owned_inner_if_ref ()) , None => None , } , r#state_or_province_of_issue : match & self . r#state_or_province_of_issue { Some (v) => Some (v . to_owned_inner_if_ref ()) , None => None , } , r#locale_of_issue : match & self . r#locale_of_issue { Some (v) => Some (v . to_owned_inner_if_ref ()) , None => None , } , r#redemption_date : match & self . r#redemption_date { Some (v) => Some (v . to_owned_inner_if_ref ()) , None => None , } , r#strike_price : match & self . r#strike_price { Some (v) => Some (v) , None => None , } , r#strike_currency : match & self . r#strike_currency { Some (v) => Some (v . to_owned_inner_if_ref ()) , None => None , } , r#opt_attribute : match & self . r#opt_attribute { Some (v) => Some (v) , None => None , } , r#contract_multiplier : match & self . r#contract_multiplier { Some (v) => Some (v) , None => None , } , r#coupon_rate : match & self . r#coupon_rate { Some (v) => Some (v) , None => None , } , r#security_exchange : match & self . r#security_exchange { Some (v) => Some (v . to_owned_inner_if_ref ()) , None => None , } , r#issuer : match & self . r#issuer { Some (v) => Some (v . to_owned_inner_if_ref ()) , None => None , } , r#encoded_issuer : match & self . r#encoded_issuer { Some (v) => Some (v . to_owned_inner_if_ref ()) , None => None , } , r#security_desc : match & self . r#security_desc { Some (v) => Some (v . to_owned_inner_if_ref ()) , None => None , } , r#encoded_security_desc : match & self . r#encoded_security_desc { Some (v) => Some (v . to_owned_inner_if_ref ()) , None => None , } , r#pool : match & self . r#pool { Some (v) => Some (v . to_owned_inner_if_ref ()) , None => None , } , r#contract_settl_month : match & self . r#contract_settl_month { Some (v) => Some (v . to_owned_inner_if_ref ()) , None => None , } , r#cp_program : match & self . r#cp_program { Some (v) => Some (v) , None => None , } , r#cp_reg_type : match & self . r#cp_reg_type { Some (v) => Some (v . to_owned_inner_if_ref ()) , None => None , } , r#evnt_grp : match & self . r#evnt_grp { Some (v) => Some (v . iter () . map (| rep_grp | rep_grp . to_owned_inner_if_ref ()) . collect ()) , None => None , } , r#dated_date : match & self . r#dated_date { Some (v) => Some (v . to_owned_inner_if_ref ()) , None => None , } , r#interest_accrual_date : match & self . r#interest_accrual_date { Some (v) => Some (v . to_owned_inner_if_ref ()) , None => None , } , r#instrmt_leg_grp : match & self . r#instrmt_leg_grp { Some (v) => Some (v . iter () . map (| rep_grp | rep_grp . to_owned_inner_if_ref ()) . collect ()) , None => None , } , r#und_instrmt_grp : match & self . r#und_instrmt_grp { Some (v) => Some (v . iter () . map (| rep_grp | rep_grp . to_owned_inner_if_ref ()) . collect ()) , None => None , } , r#adv_side : self . r#adv_side , r#quantity : self . r#quantity , r#qty_type : match & self . r#qty_type { Some (v) => Some (v) , None => None , } , r#price : match & self . r#price { Some (v) => Some (v) , None => None , } , r#currency : match & self . r#currency { Some (v) => Some (v . to_owned_inner_if_ref ()) , None => None , } , r#trade_date : match & self . r#trade_date { Some (v) => Some (v . to_owned_inner_if_ref ()) , None => None , } , r#transact_time : match & self . r#transact_time { Some (v) => Some (v . to_owned_inner_if_ref ()) , None => None , } , r#text : match & self . r#text { Some (v) => Some (v . to_owned_inner_if_ref ()) , None => None , } , r#encoded_text : match & self . r#encoded_text { Some (v) => Some (v . to_owned_inner_if_ref ()) , None => None , } , r#url_link : match & self . r#url_link { Some (v) => Some (v . to_owned_inner_if_ref ()) , None => None , } , r#last_mkt : match & self . r#last_mkt { Some (v) => Some (v . to_owned_inner_if_ref ()) , None => None , } , r#trading_session_id : match & self . r#trading_session_id { Some (v) => Some (v . to_owned_inner_if_ref ()) , None => None , } , r#trading_session_sub_id : match & self . r#trading_session_sub_id { Some (v) => Some (v . to_owned_inner_if_ref ()) , None => None , } , } } }