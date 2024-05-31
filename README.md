# Financial Information Exchange
## Introduction
* The project is a simple implementation of the Financial Information Exchange (FIX) protocol and contains the following utilities.
  
  * [fix_model_core](./model/core/) - contains utilities such as [Ascii](./model/core/src/types/asciistring.rs), [&asc](./model/core/src/types/asciistr.rs), [aschar](./model/core/src/types/asciichar.rs) types which can be used in place of `rust` native `String`, `&str`, `char` types.
  
  * [fix_model_generator](./model/generator/) - contains rust code generator that auto-creates `structs` with [serde](https://serde.rs) that can be serialized into `FIX` & `JSON` standard formats. Generator is using [QuickFix](https://github.com/quickfix/quickfix/tree/master/spec) specification files as a input format.
  
  * [fix_model_v42](./model/v42/) & [fix_model_v44](./model/v44/) - module contains the boiler plate structs created by above generator.
  
  * [fix_serde](./serde/) - contains `serde` serialization & deserialization implementation for `FIX` messages.

## Basic Usage
 * reffer to [fix_model_v44/examples](./model/v44/examples/model_example.rs) for advanced usage and `FIX message` creation

  ```rust
use fix_model_core::prelude::*;
use fix_serde::prelude::*;
use fix_model_v44::*;

pub fn from_fix<'de, T: serde::Deserialize<'de>>(slice: &'de [u8]) -> fix_serde::prelude::Result<T> {
    fix_serde::prelude::from_slice_with_schema::<_, Fix44Schema>(slice)
}
pub fn to_fix<T: serde::Serialize>(value: &T, capacity: Option<usize>) -> fix_serde::prelude::Result<Serializer<BytesWrite, Fix44Schema>> {
    fix_serde::prelude::to_bytes_with_schema::<_, Fix44Schema>(value, capacity)
}

fn main() {
    // allocated Ascii & Data
    let msg_inp = NewOrderSingle::<Ascii, aschar, Data> {
        cl_ord_id: "cl_ord_id".to_owned().try_into().unwrap(),
        side: Side::Buy,
        order_qty: Some(100_f64.into()),
        price: Some(99.99.into()),
        ..Default::default()
    };
    let fix = to_fix(&msg_inp, 1024.into()).unwrap();
    println!("fix: {}", fix);

    // borrowed Ascii & Data
    let msg_out: NewOrderSingle<&asc, aschar, &dat> = from_fix(&fix).unwrap();
    let msg_out = msg_out.to_owned_inner_if_ref();
    assert_eq!(msg_inp, msg_out);

    let json = serde_json::to_string(&msg_inp).unwrap();
    println!("json: {}", json);
}

  ```

  ```json
  fix: len: 68, capacity: 1024, bytes: "11=cl_ord_id🔸54=1🔸60=TransactTime:60@Default🔸38=100.0🔸40=1🔸44=99.99🔸"
  json: {"ClOrdID":"cl_ord_id","Side":"1","TransactTime":"TransactTime:60@Default","OrderQty":100.0,"OrdType":"1","Price":99.99}
  ```