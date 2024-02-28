use fix_model_test::unittest::setup;
use fix_serde::{
    // de::rep_grp_deserializer::RepeatingGroupSeqAccess,
    unittest::{from_slice_unittest, to_bytes_unittest},
};
use log::info;
use serde::{Deserialize, Serialize};

fix_model_generator::prelude::fix_string!(RefMsgType, 372);
fix_model_generator::prelude::fix_ascii_char_enum!(
    MsgDirection, 385, Receive :  "R", Send :  "S",
);
fix_model_generator::prelude::fix_usize!(EncryptMethod, 98);
fix_model_generator::prelude::fix_usize!(HeartBtInt, 108);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct MsgType<T> {
    #[serde(rename = "372")]
    #[serde(alias = "RefMsgType")]
    pub msg_type: RefMsgType<T>,

    #[serde(rename = "385")]
    #[serde(alias = "MsgDirection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg_direction: Option<MsgDirection>,
}
impl<T> From<(T, Option<MsgDirection>)> for MsgType<T> {
    fn from(args: (T, Option<MsgDirection>)) -> Self {
        let (msg_type, msg_direction) = args;
        Self {
            msg_type: RefMsgType::new(msg_type),
            msg_direction,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct LogonMsg<T> {
    // basic fields
    #[serde(rename = "98")]
    pub encrypt_method: EncryptMethod,

    // optional repeating group
    #[serde(rename = "384")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_msg_types: Option<Vec<MsgType<T>>>,

    // basic fields
    #[serde(rename = "108")]
    pub heart_bt_int: HeartBtInt,
}

// A:Logon
//         F: Y -> (EncryptMethod-98-INT) enums: 0:NONE, 1:PKCS, 2:DES, 3:PKCS_DES, 4:PGP_DES ...+2
//         F: Y -> (HeartBtInt-108-INT)
//         F: N -> (RawDataLength-95-LENGTH)
//         F: N -> (RawData-96-DATA)
//         F: N -> (ResetSeqNumFlag-141-BOOLEAN) enums: N:NO, Y:YES
//         F: N -> (MaxMessageSize-383-INT)
//         G: N -> (NoMsgTypes-384-INT)
//                 F: N -> (RefMsgType-372-STRING)
//                 F: N -> (MsgDirection-385-CHAR) enums: R:RECEIVE, S:SEND

// order of fields info https://www.fixtrading.org/standards/tagvalue-online/
// Field sequence

// Except where noted, fields within a message can be defined in any sequence. (Relative position of a field within a message is inconsequential.) The exceptions to this rule are:
//  * General message format is composed of the standard header, followed by the body, followed by the standard trailer.
//  * The first three fields in the StandardHeader component must be BeginString(8), followed by BodyLength(9), followed by MsgType(35), in that sequence.
//  * The last field in the standard trailer must be CheckSum(10).
//  * Within a repeating group, field sequence is strictly defined by a group definition.
#[test]
fn test_msg_with_repgrp() {
    setup::log::configure_level(log::LevelFilter::Trace);

    let no_msg_types = vec![
        MsgType {
            msg_type: "LOGIN".into(),
            msg_direction: MsgDirection::Receive.into(),
        },
        MsgType {
            msg_type: RefMsgType::new("LOGOUT"),
            msg_direction: MsgDirection::Send.into(),
        },
        ("WITHOUT_DIRECTION", None).into(),
        ("OTHER", MsgDirection::Send.into()).into(),
    ];

    let inp = LogonMsg {
        encrypt_method: 999.into(),
        no_msg_types: no_msg_types.into(), // repeating group
        heart_bt_int: 999.into(),
    };
    info!("inp:      {:?}", inp);

    let json = serde_json::to_string(&inp).unwrap();
    info!("json: {}", json);

    let fix = to_bytes_unittest(&inp).unwrap();
    info!("fix: {}", fix);

    let out_fix: LogonMsg<&str> = from_slice_unittest(&fix).unwrap();
    info!("out_fix: {:?}", out_fix);
    assert_eq!(inp, out_fix);

    let out_json: LogonMsg<&str> = serde_json::from_str(&json).unwrap();
    info!("out_json: {:?}", out_json);
    assert_eq!(inp, out_json);
}

// ////////////////////////////////////////////////////////////////////////////////////////////////////////
// <message name='AllocationInstruction' msgtype='J' msgcat='app'>
//    <field name='AllocID' required='Y' />                 // <field number='70' name='AllocID' type='STRING' />
//    <field name='AllocTransType' required='Y' />          // <field number='71' name='AllocTransType' type='CHAR'>
//    <field name='AllocType' required='Y' />               // <field number='626' name='AllocType' type='INT'>
//    <field name='SecondaryAllocID' required='N' />        // <field number='793' name='SecondaryAllocID' type='STRING' />
//    ....
//    <component name='OrdAllocGrp' required='N' />
//    <component name='ExecAllocGrp' required='N' />
//    <field name='PreviouslyReported' required='N' />
//    <field name='ReversalIndicator' required='N' />
//    <field name='MatchType' required='N' />
//    <field name='Side' required='Y' />
fix_model_generator::prelude::fix_string!(AllocID, 70);
fix_model_generator::prelude::fix_ascii_char_enum!(
    AllocTransType, 71, New :  "0", Replace :  "1", Cancel :  "2",
);
fix_model_generator::prelude::fix_isize!(AllocType, 626);
fix_model_generator::prelude::fix_string!(SecondaryAllocID, 793);

// <component name='OrdAllocGrp'>
//      <group name='NoOrders' required='N'>                        // <field number='73' name='NoOrders' type='NUMINGROUP' />
//          <field name='ClOrdID' required='N' />                   // <field number='11' name='ClOrdID' type='STRING' />
//          <field name='OrderID' required='N' />                   // <field number='37' name='OrderID' type='STRING' />
//          <field name='SecondaryOrderID' required='N' />          // <field number='198' name='SecondaryOrderID' type='STRING' />
//          <field name='SecondaryClOrdID' required='N' />          // <field number='526' name='SecondaryClOrdID' type='STRING' />
//          <field name='ListID' required='N' />                    // <field number='66' name='ListID' type='STRING' />
//          <component name='NestedParties2' required='N' />
//          <field name='OrderQty' required='N' />                  // <field number='38' name='OrderQty' type='QTY' />
//          <field name='OrderAvgPx' required='N' />
//          <field name='OrderBookingQty' required='N' />
//      </group>
// </component>
fix_model_generator::prelude::fix_string!(ClOrdID, 11);
fix_model_generator::prelude::fix_string!(OrderID, 37);
fix_model_generator::prelude::fix_usize!(OrderQty, 38);

// <component name='NestedParties2'>
//      <group name='NoNested2PartyIDs' required='N'>               // <field number='756' name='NoNested2PartyIDs' type='NUMINGROUP' />
//          <field name='Nested2PartyID' required='N' />            // <field number='757' name='Nested2PartyID' type='STRING' />
//          <field name='Nested2PartyIDSource' required='N' />      // <field number='758' name='Nested2PartyIDSource' type='CHAR' />
//          <field name='Nested2PartyRole' required='N' />          // <field number='759' name='Nested2PartyRole' type='INT' />
//          <component name='NstdPtys2SubGrp' required='N' />
//      </group>
// </component>
fix_model_generator::prelude::fix_string!(Nested2PartyID, 757);
fix_model_generator::prelude::fix_string!(Nested2PartyIDSource, 758);

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
struct OrdAllocGrp<S>
where
    Nested2PartyIDSource<S>: Default,
{
    #[serde(rename = "11")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clt_ord_id: Option<ClOrdID<S>>,

    #[serde(rename = "756")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nested2_party_ids: Option<Vec<NestedParties2<S>>>,

    #[serde(rename = "38")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_qty: Option<OrderQty>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
struct NestedParties2<S>
where
    Nested2PartyIDSource<S>: Default,
{
    #[serde(rename = "757")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nested2_party_id: Option<Nested2PartyID<S>>,

    #[serde(rename = "758")]
    // #[serde(skip_serializing_if = "Option::is_none")]
    pub nested2_party_id_source: Nested2PartyIDSource<S>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
struct AllocationInstruction<S>
where
    AllocID<S>: Default,
    Nested2PartyIDSource<S>: Default,
{
    #[serde(rename = "70")]
    pub alloc_id: AllocID<S>,
    // #[serde(rename = "71")]
    // pub alloc_trans_type: AllocTransType,
    // #[serde(rename = "626")]
    // pub alloc_type: AllocType,
    #[serde(rename = "793")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_alloc_id: Option<SecondaryAllocID<S>>,

    #[serde(rename = "73")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ord_alloc_grp: Option<Vec<OrdAllocGrp<S>>>,
}

#[test]
fn test_msg_with_nested_repgrp() {
    setup::log::configure_level(log::LevelFilter::Info);

    let ord_alloc_grp: Vec<OrdAllocGrp<&str>> = vec![
        OrdAllocGrp {
            clt_ord_id: Some("1".into()),
            nested2_party_ids: Some(vec![NestedParties2 {
                nested2_party_id: Some("1.1".into()),
                nested2_party_id_source: "1.1.1".into(),
            }]),
            order_qty: Some(1.into()),

            ..Default::default()
        },
        OrdAllocGrp {
            clt_ord_id: Some("2".into()),
            nested2_party_ids: Some(vec![NestedParties2 {
                // nested2_party_id: Some("2.1".into()),
                nested2_party_id_source: "2.1.1".into(),
                ..Default::default()
            }]),
            order_qty: Some(2.into()),
            ..Default::default()
        },
    ];
    let alloc_inst = AllocationInstruction::<&str> {
        ord_alloc_grp: Some(ord_alloc_grp),
        ..Default::default()
    };
    // info!("alloc_inst: {:?}", alloc_inst);

    let json = serde_json::to_string(&alloc_inst).unwrap();
    info!("json: {}", json);

    let fix = to_bytes_unittest(&alloc_inst).unwrap();
    info!("fix: {}", fix);

    let json_out: AllocationInstruction<&str> = serde_json::from_str(&json).unwrap();
    // info!("json_out: {:?}", json_out);

    assert_eq!(alloc_inst, json_out);

    let out_fix: AllocationInstruction<&str> = from_slice_unittest(&fix).unwrap();
    // info!("out_fix: {:?}", out_fix);
    assert_eq!(alloc_inst, out_fix);

    info!("alloc_inst: {:?}", alloc_inst);
}
