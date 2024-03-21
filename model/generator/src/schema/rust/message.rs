#[macro_export]
macro_rules! fix_message {
    // generic & lifetime match lifted from https://stackoverflow.com/questions/41603424/rust-macro-accepting-type-with-generic-parameters
    ($NAME:ident $(< $( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+ >)?, $($FIELD:ident: $TYPE:ty),+) => {
        #[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Clone)]
        #[serde(rename_all = "PascalCase")]
        struct $NAME$(< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? {
            $(pub $FIELD: $TYPE,)+
        }

        impl $(< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $NAME$(< $( $lt ),+ >)? {
            #[inline(always)]
            pub fn to_owned(&self) -> $NAME<String> {
                $NAME {
                    $($FIELD: self.$FIELD.to_owned(),)+
                }
            }
            // pub fn compare(&self, other: &$NAME<String>) -> bool {
            //     $(*self.$FIELD.value() == *other.$FIELD.value())&&+
            //     // true
            // }
        }

        impl $(< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? fix_model_core::prelude::Serialize for $NAME$(< $( $lt ),+ >)? {
            #[inline(always)]
            fn serialize(&self, ser: &mut impl fix_model_core::prelude::Serializer){
                $(self.$FIELD.serialize(ser);)+
            }
        }
    };
}
