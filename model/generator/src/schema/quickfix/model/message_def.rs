use crate::{
    prelude::Error,
    schema::{
        quickfix::model::{
            component_def::{QFCompomentPart, QFComponentDef},
            field_def::QFFieldDef,
            refs::{QFComponentRef, QFFieldRef},
            repgroup_def::QFGroupDef,
            root::QFModel,
        },
        rust::model::{
            field::{RFldDef, RFldDefData, RFldDefRepGroup},
            message::{MessageCategory, RFMessageDef},
            message_member::RMessageMember,
        },
    },
};
use serde::{Deserialize, Serialize};

use super::repgroup_def::QFRepGroupDef;

#[derive(Debug, Deserialize)]
pub struct QFMessageDefs {
    #[serde(rename = "$value")]
    pub(super) defs: Vec<QFMessageDef>,
}
impl QFMessageDefs {
    pub fn get(&self) -> &Vec<QFMessageDef> {
        &self.defs
    }
    pub fn extract_rep_grp_defs(&self) -> Vec<QFRepGroupDef> {
        self.defs.iter().flat_map(|msg_def| msg_def.extract_rep_grp_defs()).collect()
    }
}

#[derive(Debug, Deserialize, Serialize)] // <message name='Heartbeat' msgtype='0' msgcat='admin'>
#[serde(rename = "message")]
pub struct QFMessageDef {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@msgtype")]
    pub msg_type: String,
    #[serde(rename = "@msgcat")]
    pub msg_cat: String,

    #[serde(rename = "$value")]
    pub(super) parts: Option<Vec<QFMessagePart>>,
}
impl QFMessageDef {
    pub fn parts(&self) -> Vec<QFMessagePart> {
        match &self.parts {
            Some(atrb) => atrb.to_vec(),
            None => Vec::new(),
        }
    }
    pub fn details(&self, fld_defs: &Vec<QFFieldDef>, cmp_defs: &Vec<QFComponentDef>) -> String {
        let mut f = format!("MsgDef: {}-{}-{}\n", self.name, self.msg_type, self.msg_cat);
        use std::fmt::Write;
        //  there is a message with no parts in one of xmls, this is a workaroudn for this
        let msg_parts = match &self.parts {
            Some(msg_parts) => msg_parts.to_owned(),
            None => vec![],
        };

        let pad = 0;

        for part in msg_parts {
            match &part {
                QFMessagePart::FieldRef(fld_ref) => {
                    write!(f, "{}", fld_ref.details(fld_defs, pad + 1)).unwrap();
                }
                QFMessagePart::GroupDef(grp_def) => {
                    write!(f, "{}", grp_def.details(fld_defs, cmp_defs, pad + 1)).unwrap();
                }
                QFMessagePart::ComponentRef(cmp_ref) => {
                    write!(f, "{}", cmp_ref.details(fld_defs, cmp_defs, pad + 1)).unwrap();
                }
            }
        }
        f
    }
    pub fn extract_rep_grp_defs(&self) -> Vec<QFRepGroupDef> {
        let mut rep_grp_defs = Vec::new();
        match &self.parts {
            Some(parts) => {
                for part in parts {
                    if let QFMessagePart::GroupDef(grp_def) = part {
                        let rep_grp_name = self.name.to_owned() + &msg_embeded_group_name_to_rep_grp_name(&grp_def.name);
                        rep_grp_defs.extend(QFGroupDef::extract_rep_grp_defs(grp_def, rep_grp_name));
                    }
                }
            }
            None => (),
        }
        rep_grp_defs
    }
}

pub fn msg_embeded_group_name_to_rep_grp_name(name: &str) -> String {
    let n = name.to_owned();
    match (n.starts_with("No"), n.ends_with("Grp")) {
        (true, true) => n[2..n.len()].to_owned(),
        (true, false) => n[2..n.len()].to_owned() + "Grp",
        _ => n,
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub enum QFMessagePart {
    #[serde(rename = "field")]
    FieldRef(QFFieldRef), // <field name='IOINaturalFlag' required='N' />
    #[serde(rename = "group")]
    GroupDef(QFGroupDef), // <group name='NoIOIQualifiers' required='N'> <field name='IOIQualifier' required='N' /> .... <field ..../> </group>
    #[serde(rename = "component")]
    ComponentRef(QFComponentRef), // <component name='Instrument' required='Y' />
}
impl QFMessagePart {
    pub fn name(&self) -> &str {
        match self {
            QFMessagePart::FieldRef(fld) => &fld.name,
            QFMessagePart::GroupDef(grp) => &grp.name,
            QFMessagePart::ComponentRef(cmp) => &cmp.name,
        }
    }
    pub fn required(&self) -> &str {
        match self {
            QFMessagePart::FieldRef(fld) => &fld.required,
            QFMessagePart::GroupDef(grp) => &grp.required,
            QFMessagePart::ComponentRef(cmp) => &cmp.required,
        }
    }
}

impl From<(&QFMessageDef, &QFModel)> for RFMessageDef {
    fn from(args: (&QFMessageDef, &QFModel)) -> Self {
        let (qf_msg_def, qf_model) = args;
        let name = qf_msg_def.name.clone();
        let msg_type = qf_msg_def.msg_type.clone();
        let msg_category = match qf_msg_def.msg_cat.as_str() {
            "admin" => MessageCategory::Admin,
            "app" => MessageCategory::App,
            "header" => MessageCategory::Header,
            "trailer" => MessageCategory::Trailer,
            "tag_value" => MessageCategory::TagValue,

            _ => {
                panic!("{}", Error::QuickFixMessageCategoryNotMapped(qf_msg_def.msg_cat.to_string()));
            }
        };

        let mut errors = Vec::new();
        let members = qf_msg_def
            .parts()
            .iter()
            .flat_map(|qf_msg_parts| {
                match qf_msg_parts {
                    QFMessagePart::FieldRef(qf_fld_ref) => {
                        match qf_fld_ref_2_r_msg_member_plain_or_data(&qf_msg_def.name, qf_fld_ref, qf_model) {
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
                    QFMessagePart::ComponentRef(qf_cmp_ref) => {
                        let name = format!("Msg: {}", qf_msg_def.name);
                        match qf_cmp_ref_2_r_msg_members(&name, qf_cmp_ref, qf_model) {
                            Ok(members) => members,
                            Err(e) => {
                                errors.push(e);
                                vec![]
                            }
                        }
                    }
                    QFMessagePart::GroupDef(grp_def) => {
                        // !("this is only relevant to some admin messages but not app messages as all its groups are defined via component")
                        // log::warn!("'{}' has group: '{}' will be excluded", qf_msg_def.name, grp_def.name);

                        let generic_info = qf_model.grp_def_generic_types(grp_def);
                        let qf_fld_def = qf_model.fld_def(&grp_def.as_field_ref()).unwrap();
                        let member = RMessageMember {
                            member: RFldDef::RepGroup(RFldDefRepGroup {
                                name: qf_msg_def.name.to_owned() + &msg_embeded_group_name_to_rep_grp_name(&grp_def.name),
                                tag: qf_fld_def.number.parse().unwrap(),
                                generic_info,
                            }),
                            required: grp_def.required == "Y", // note that we use grp required flag
                        };
                        // log::warn!("{:?}", member);

                        vec![member]
                    }
                }
            })
            .collect::<Vec<_>>();
        let xml = quick_xml::se::to_string(qf_msg_def).unwrap();
        Self {
            name,
            msg_type,
            msg_category,
            members,
            errors,
            xml,
        }
    }
}

pub fn qf_fld_ref_2_r_msg_member_plain_or_data(msg_name: &str, qf_fld_ref: &QFFieldRef, qf_model: &QFModel) -> Result<Option<RMessageMember>, Error> {
    let qf_fld_def = qf_model.fld_def(qf_fld_ref).unwrap();

    // plain
    if qf_fld_def.is_type_plain() {
        match qf_fld_def.try_into() {
            Ok(f) => Ok(Some(RMessageMember {
                member: RFldDef::Plain(f),
                required: qf_fld_ref.required == "Y",
            })),
            Err(_) => {
                // error will be logged when all fields are created
                Ok(None)
            }
        }

    // len data pair
    } else if qf_fld_def.is_type_length() {
        let qf_fld_def_len_dat = qf_model.field_def_len_data(qf_fld_ref).unwrap();
        return Ok(Some(RMessageMember {
            member: RFldDef::Data(RFldDefData {
                len_name: qf_fld_def_len_dat.0.name.clone(),
                len_tag: qf_fld_def_len_dat.0.number.parse().unwrap(),
                data_name: qf_fld_def_len_dat.1.name.clone(),
                data_tag: qf_fld_def_len_dat.1.number.parse().unwrap(),
            }),
            required: qf_fld_ref.required == "Y",
        }));
    // skip data // Skip Data Filed because it is mapped by Len field already, don't log error
    } else if qf_fld_def.is_type_data() {
        Ok(None)
    } else {
        Err(Error::QuickFixMessageMissingPart {
            msg: msg_name.to_owned(),
            name: qf_fld_ref.name.clone(),
        })
    }
}

fn qf_cmp_ref_2_r_msg_members(msg_name_log: &str, qf_cmp_ref: &QFComponentRef, qf_model: &QFModel) -> Result<Vec<RMessageMember>, Error> {
    let qf_cmp_def = qf_model.cmp_def(qf_cmp_ref).unwrap();
    let mut members = vec![];

    for part in &qf_cmp_def.parts {
        match part {
            #[allow(clippy::single_match)]
            QFCompomentPart::FieldRef(qf_fld_ref) => {
                match qf_fld_ref_2_r_msg_member_plain_or_data(msg_name_log, qf_fld_ref, qf_model)? {
                    Some(member) => members.push(member),
                    None => (), // data is already mapped via len
                };
            }
            QFCompomentPart::GroupDef(grp_def) => {
                let generic_info = qf_model.grp_def_generic_types(grp_def);
                let qf_fld_def = qf_model.fld_def(&grp_def.as_field_ref()).unwrap();
                let member = RMessageMember {
                    member: RFldDef::RepGroup(RFldDefRepGroup {
                        name: qf_cmp_ref.name.clone(),
                        tag: qf_fld_def.number.parse().unwrap(),
                        generic_info,
                    }),
                    required: qf_cmp_ref.required == "Y", // note that we use comp required flag
                };
                members.push(member);
            }
            QFCompomentPart::ComponentRef(qf_cmp_ref) => {
                // recursive
                members.extend(qf_cmp_ref_2_r_msg_members(msg_name_log, qf_cmp_ref, qf_model)?)
            }
        }
    }
    Ok(members)
}
