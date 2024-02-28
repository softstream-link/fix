use crate::schema::{
    quickfix::model::{
        component_def::{PartsCounter, QFComponentDef},
        field_def::QFFieldDef,
        message_def::qf_fld_ref_2_r_msg_member_plain_or_data,
        refs::{QFComponentRef, QFFieldRef},
        root::{QFModel, PAD},
    },
    rust::model::{
        field::{RFldDef, RFldDefRepGroup},
        message::{MessageCategory, RFMessageDef},
        message_member::{RMessageMember, RMessageMembers},
        repeating_group::RRepGrpMessageDef,
    },
};
use serde::{Deserialize, Serialize};
use std::vec;

use super::message_def::msg_embeded_group_name_to_rep_grp_name;

/// # Represents a group definition in a QuickFix message
/// ```xml
/// <group name='NoPartyIDs' required='N'>
///     <field name='PartyID' required='N' />
///     <field name='PartyIDSource' required='N' />
///     <field name='PartyRole' required='N' />
///     <component name='PtysSubGrp' required='N' />
/// </group>
/// ```

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename = "group")] // only visible in the ocs
pub struct QFGroupDef {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@required")]
    pub required: String,
    #[serde(rename = "$value")]
    pub parts: Vec<QFGroupPart>,
}

impl QFGroupDef {
    /// interprets GroupName as Field name so that Group called NoPartyIDs is treated as a field for look up purposes
    pub fn as_field_ref(&self) -> QFFieldRef {
        QFFieldRef {
            name: self.name.clone(),
            required: self.required.clone(),
        }
    }
    pub fn details(&self, fld_defs: &Vec<QFFieldDef>, cmp_defs: &Vec<QFComponentDef>, pad: usize) -> String {
        let mut f = String::new();
        use std::fmt::Write;
        writeln!(f, "{}|->GrpDef: {}@{}", PAD.repeat(pad), self.required, self.name).unwrap();
        for part in &self.parts {
            match part {
                QFGroupPart::FieldRef(fld_ref) => {
                    write!(f, "{}", fld_ref.details(fld_defs, pad + 1)).unwrap();
                }
                QFGroupPart::GroupDef(grp) => {
                    write!(f, "{}", grp.details(fld_defs, cmp_defs, pad + 1)).unwrap();
                }
                QFGroupPart::ComponentRef(cmp_ref) => {
                    write!(f, "{}", cmp_ref.details(fld_defs, cmp_defs, pad + 1)).unwrap();
                }
            }
        }
        f
    }
    pub fn parts_count(&self) -> PartsCounter {
        let sum = self
            .parts
            .iter()
            .map(|g| match g {
                QFGroupPart::FieldRef(_) => PartsCounter { grp: 0, fld: 1, cmp: 0 },
                QFGroupPart::GroupDef(_) => PartsCounter { grp: 1, fld: 0, cmp: 0 },
                QFGroupPart::ComponentRef(_) => PartsCounter { grp: 0, fld: 0, cmp: 1 },
            })
            .sum::<PartsCounter>();
        sum
    }
    pub fn extract_rep_grp_defs(&self, rep_grp_name: String) -> Vec<QFRepGroupDef> {
        let mut rep_grp_defs = Vec::new();
        rep_grp_defs.push(QFRepGroupDef {
            rep_grp_name: rep_grp_name.clone(),
            grp_def: self,
            xml: quick_xml::se::to_string(&self).unwrap(),
        });

        for part in &self.parts {
            match part {
                QFGroupPart::GroupDef(grp_def) => {
                    let rep_grp_name = rep_grp_name.to_owned() + &msg_embeded_group_name_to_rep_grp_name(&grp_def.name);
                    rep_grp_defs.extend(QFGroupDef::extract_rep_grp_defs(grp_def, rep_grp_name));
                }
                _ => {}
            }
        }
        rep_grp_defs
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum QFGroupPart {
    #[serde(rename = "field")]
    FieldRef(QFFieldRef),
    #[serde(rename = "component")]
    ComponentRef(QFComponentRef),
    #[serde(rename = "group")]
    GroupDef(QFGroupDef), // only used prior to fix 4.4
}

#[derive(Debug)]
pub struct QFRepGroupDef<'a> {
    pub rep_grp_name: String,
    pub grp_def: &'a QFGroupDef,
    pub xml: String,
}

impl From<(&QFRepGroupDef<'_>, &QFModel)> for RRepGrpMessageDef {
    fn from(value: (&QFRepGroupDef, &QFModel)) -> RRepGrpMessageDef {
        let (qf_rep_grp_def, qf_model) = value;

        let mut errors = Vec::new();

        let name = qf_rep_grp_def.rep_grp_name.to_owned();
        let xml = qf_rep_grp_def.xml.to_owned();

        let members = qf_rep_grp_def
            .grp_def
            .parts
            .iter()
            .map(|p| match p {
                // nested <component_def/single_group_def/field_ref>
                QFGroupPart::FieldRef(qf_fld_ref) => {
                    match qf_fld_ref_2_r_msg_member_plain_or_data(qf_fld_ref, qf_model) {
                        // returns Some member for eitehr plain field or len field of the data pairs
                        Ok(Some(member)) => vec![member],
                        // data field is returned as none because is already mapped by len field from the len/data pair
                        Ok(None) => vec![],
                        Err(e) => {
                            errors.push(e);
                            vec![]
                        }
                    }
                }
                // nested <component_def/single_group_def/component_ref/..>
                QFGroupPart::ComponentRef(qf_cmp_ref) => {
                    let qf_cmp_def_sub = qf_model.cmp_def(qf_cmp_ref).unwrap();
                    let members = RMessageMembers::from((qf_cmp_def_sub, qf_model));
                    errors.extend(members.errors);
                    members.members
                }
                QFGroupPart::GroupDef(grp_def) => {
                    let generic_info = qf_model.grp_def_generic_types(grp_def);
                    let qf_fld_def = qf_model.fld_def(&grp_def.as_field_ref()).unwrap();
                    let member = RMessageMember {
                        member: RFldDef::RepGroup(RFldDefRepGroup {
                            name: qf_rep_grp_def.rep_grp_name.to_owned() + &msg_embeded_group_name_to_rep_grp_name(&grp_def.name),
                            tag: qf_fld_def.number.parse().unwrap(),
                            generic_info: generic_info,
                        }),
                        required: grp_def.required == "Y", // note that we use grp required flag
                    };
                    vec![member]
                    // panic!("QFComponentDef/SingleGroupDef/SubGroupDef is not supported {}", xml);
                }
            })
            .flatten()
            .collect::<Vec<_>>();

        RRepGrpMessageDef(RFMessageDef {
            name,
            msg_type: "na".to_string(),
            msg_category: MessageCategory::RepGrp,
            members,
            errors,
            xml,
        })
    }
}
