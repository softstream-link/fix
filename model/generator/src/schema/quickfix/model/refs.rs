use super::{component_def::QFComponentDef, field_def::QFFieldDef};
use crate::schema::quickfix::model::root::PAD;
use serde::{Deserialize, Serialize};

/// # Represents a field reference in the QuickFIX schema
/// ```xml
/// <field name='IOINaturalFlag' required='N' />
/// ```
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct QFFieldRef {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@required")]
    pub required: String,
}
impl QFFieldRef {
    pub fn details(&self, fld_defs: &[QFFieldDef], pad: usize) -> String {
        use std::fmt::Write;
        let mut f = String::new();

        writeln!(
            f,
            "{}|->FldRef: {}@{:25} FldDef:{}",
            PAD.repeat(pad),
            self.required,
            self.name,
            fld_defs.iter().find(|f| f.name == self.name).unwrap()
        )
        .unwrap();

        f
    }
}

/// # Represents a single component reference in the QuickFIX schema
/// ```xml
/// <component name='Instrument' required='Y' />
/// ```
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct QFComponentRef {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@required")]
    pub required: String,
}
impl QFComponentRef {
    pub fn details(&self, fld_defs: &Vec<QFFieldDef>, cmp_defs: &Vec<QFComponentDef>, pad: usize) -> String {
        use std::fmt::Write;
        let mut f = String::new();
        let cmp_def = cmp_defs.iter().find(|cmp_def| cmp_def.name == self.name).unwrap();
        write!(
            f,
            "{}|*>CmpRef: {}\n{}",
            PAD.repeat(pad),
            self.name,
            cmp_def.details(fld_defs, cmp_defs, pad + 1)
        )
        .unwrap();

        f
    }
    // pub fn rf_members(&self, qf_model: &QFModel) -> Vec<RMessageMember> {
    //     let qf_cmp_def = qf_model.cmp_def(self).unwrap();
    //     let errors = Vec::<super::Error>::new();
    //     let members = qf_cmp_def.parts.iter().map(|part| match part{
    //         QFCompomentPart::FieldRef(fld_ref) => fld_ref.as_member(),
    //         QFCompomentPart::GroupDef(grp_def) => {
    //             let generic_info = qf_model.grp_def_generic_types(grp_def);
    //             let qf_fld_def = qf_model.fld_def(&grp_def.as_field_ref()).unwrap();

    //             vec![RMessageMember {
    //                 member: RFldDef::RepGroup(RFldDefRepGroup {
    //                     name: qf_cmp_ref_sub.name.to_owned(),
    //                     tag: qf_fld_def.number.parse().unwrap(),
    //                     generic_info: generic_info,
    //                 }),
    //                 required: qf_cmp_ref_sub.required == "Y",
    //             }]
    //         },
    //         QFCompomentPart::ComponentRef(cmp_ref) => cmp_ref.as_member(),
    //     }).collect()

    // }
}
