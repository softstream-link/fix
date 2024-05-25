use super::component_def::{QFCompomentPart, QFComponentDef, QFComponentDefs};
use super::field_def::{QFFieldDef, QFFieldDefs};
use super::header_trailer_def::{QFHeaderDef, QFTrailerDef};
use super::message_def::{QFMessageDef, QFMessageDefs, QFMessagePart};
use super::refs::{QFComponentRef, QFFieldRef};
use super::repgroup_def::{QFGroupDef, QFGroupPart, QFRepGroupDef};
use crate::prelude::RFModel;
use crate::prelude::{Error, RFldDefPlain};
use crate::schema::rust::model::field::{RFldDef, RFldDefData};
use crate::schema::rust::model::message::RFMessageDef;
use crate::schema::rust::model::repeating_group::RRepGrpMessageDef;
use crate::schema::rust::model::GenericTypeInfo;
use core::panic;
use serde::Deserialize;
use std::collections::HashSet;
use std::vec;

pub const PAD: &str = "  ";
#[derive(Debug, Deserialize)]
pub struct QFModel {
    #[serde(rename = "@type")]
    pub protocol_type: String,
    #[serde(rename = "@major")]
    pub major: String,
    #[serde(rename = "@minor")]
    pub minor: String,
    #[serde(rename = "@servicepack")]
    pub servicepack: String,

    #[serde(rename = "fields")] // name fields must match <fields> in quickfix xml
    fld_defs: QFFieldDefs,
    #[serde(rename = "messages")] // name messages must match <messages> in quickfix xml
    msg_defs: QFMessageDefs,

    #[serde(rename = "components")] // name messages must match <messages> in quickfix xml
    cmp_defs: QFComponentDefs,

    #[serde(rename = "header")]
    header_def: QFHeaderDef,

    #[serde(rename = "trailer")]
    trailer_def: QFTrailerDef,

    #[serde(skip_deserializing)]
    pub fld_defs_plain: Vec<QFFieldDef>,

    #[serde(skip_deserializing)]
    pub fld_defs_len_data: Vec<(QFFieldDef, QFFieldDef)>,
}

impl QFModel {
    fn complete_preparation(&mut self) {
        (&mut self.fld_defs).sort();
        (&mut self.cmp_defs).sort();

        {
            // add header to messages so that thier rust structs are created
            self.msg_defs.defs.push(QFMessageDef {
                name: "HeaderFull".to_string(),
                msg_type: "N/A".to_string(),
                msg_cat: "header".to_string(),
                parts: Some(self.header_def.parts.clone()),
            });
            const HEADER_PART1: &[&str] = &["BeginString", "BodyLength"];
            self.msg_defs.defs.push(QFMessageDef {
                name: "Header1".to_string(),
                msg_type: "N/A".to_string(),
                msg_cat: "header".to_string(),
                parts: {
                    Some(
                        self.header_def
                            .parts
                            .iter()
                            .filter_map(|f| match HEADER_PART1.contains(&f.name()) {
                                true => Some(f.clone()),
                                false => None,
                            })
                            .collect::<Vec<_>>(),
                    )
                },
            });
            const HEADER_PART2: &[&str] = &["MsgType", "SenderCompID", "TargetCompID"];
            self.msg_defs.defs.push(QFMessageDef {
                name: "Header2".to_string(),
                msg_type: "N/A".to_string(),
                msg_cat: "header".to_string(),
                parts: {
                    Some(
                        self.header_def
                            .parts
                            .iter()
                            .filter_map(|f| match HEADER_PART2.contains(&f.name()) {
                                true => Some(f.clone()),
                                false => None,
                            })
                            .collect::<Vec<_>>(),
                    )
                },
            });

            self.msg_defs.defs.push(QFMessageDef {
                name: "Header3".to_string(),
                msg_type: "N/A".to_string(),
                msg_cat: "header".to_string(),
                parts: {
                    Some(
                        self.header_def
                            .parts
                            .iter()
                            .filter_map(|f| match !HEADER_PART1.contains(&f.name()) && !HEADER_PART2.contains(&f.name()) {
                                true => Some(f.clone()),
                                false => None,
                            })
                            .collect::<Vec<_>>(),
                    )
                },
            });

            self.header_def.parts.iter().for_each(|part| match part {
                QFMessagePart::FieldRef(_) | QFMessagePart::ComponentRef(_) => {
                    self.msg_defs.defs.push(QFMessageDef {
                        name: "TagValueHeader".to_owned() + &part.name(),
                        msg_type: "N/A".to_string(),
                        msg_cat: "header".to_string(),
                        parts: Some(vec![QFMessagePart::FieldRef(QFFieldRef {
                            name: part.name().to_owned(),
                            required: "Y".to_string(),
                        })]),
                    });
                }
                _ => {} // TODO TagValue structs for header groups or other groups are not created at the moment
            });
        }
        // add trailer to messages so that thier rust structs are created
        {
            self.msg_defs.defs.push(QFMessageDef {
                name: "Trailer".to_string(),
                msg_type: "N/A".to_string(),
                msg_cat: "trailer".to_string(),
                parts: Some(self.trailer_def.parts.clone()),
            });

            self.trailer_def.parts.iter().for_each(|f| {
                self.msg_defs.defs.push(QFMessageDef {
                    name: "TagValueTrailer".to_owned() + &f.name(),
                    msg_type: "N/A".to_string(),
                    msg_cat: "trailer".to_string(),
                    parts: Some(vec![QFMessagePart::FieldRef(QFFieldRef {
                        name: f.name().to_owned(),
                        required: "Y".to_string(),
                    })]),
                });
            });
        }

        {
            // already sorted because .fld_defs are sorted
            self.fld_defs_plain = self
                .fld_defs
                .get()
                .iter()
                .filter(|qf_field| qf_field.is_type_plain())
                .cloned()
                .collect::<Vec<_>>();
            // add fld to messages so that thier rust structs are created

            self.fld_defs_plain.iter().for_each(|f| {
                // log::warn!("{:?}", f);
                self.msg_defs.defs.push(QFMessageDef {
                    name: "TagValue".to_owned() + &f.name,
                    msg_type: "N/A".to_string(),
                    msg_cat: "tag_value".to_string(),
                    parts: Some(vec![QFMessagePart::FieldRef(QFFieldRef {
                        name: f.name.clone(),
                        required: "Y".to_string(),
                    })]),
                });
            });
        }

        {
            let mut fld_defs_len_data = self
                .fld_defs
                .get()
                .iter()
                .filter(|qf_field| qf_field.is_type_len_data())
                .cloned()
                .collect::<Vec<_>>();
            // technically this shold already be sorted but this code was added before fld_defs sorting was added
            // not all data/length tags are in correct order in the xml hence sort by name to bring paris together
            fld_defs_len_data.sort_by(|a, b| a.name.cmp(&b.name));

            // for fld in &fld_defs_len_data {
            //     log::warn!("{:?}", fld);
            // }
            self.fld_defs_len_data = fld_defs_len_data
                .chunks(2)
                .map(|chunk| {
                    assert_eq!(chunk.len(), 2, "Binary data field must have 2 elements");

                    let (len, data) = match (&chunk[0], &chunk[1]) {
                        (len, data) if len.r#type == "LENGTH" && data.r#type == "DATA" => (len, data),
                        (data, len) if len.r#type == "LENGTH" && data.r#type == "DATA" => (len, data),
                        (x, y) => {
                            panic!(
                                "Binary data field must have 2 elements with one type DATA and another LENGTH, found: {:?}",
                                (x, y)
                            );
                        }
                    };
                    (len.clone(), data.clone())
                })
                .collect::<Vec<_>>();

            // add binary flds to TagVale messages
            self.fld_defs_len_data.iter().for_each(|f| {
                // log::warn!("{:?}", f);
                self.msg_defs.defs.push(QFMessageDef {
                    name: "TagValue".to_owned() + &f.1.name, // data tag name
                    msg_type: "N/A".to_string(),
                    msg_cat: "tag_value".to_string(),
                    parts: Some(vec![
                        QFMessagePart::FieldRef(QFFieldRef {
                            name: f.0.name.clone(),
                            required: "Y".to_string(),
                        }),
                        QFMessagePart::FieldRef(QFFieldRef {
                            name: f.1.name.clone(),
                            required: "Y".to_string(),
                        }),
                    ]),
                });
            });
        }
    }

    pub fn all_field_defs(&self) -> &Vec<QFFieldDef> {
        self.fld_defs.get()
    }
    pub fn fld_def(&self, fld_ref: &QFFieldRef) -> Option<&QFFieldDef> {
        let flds = self.fld_defs.get();
        let idx = flds.binary_search_by(|f| f.name.cmp(&fld_ref.name));
        match idx {
            Ok(idx) => Some(&flds[idx]),
            Err(_) => None,
        }
    }
    pub fn cmp_def(&self, cmp_ref: &QFComponentRef) -> Option<&QFComponentDef> {
        let cmps = self.cmp_defs.get();
        let idx = cmps.binary_search_by(|c| c.name.cmp(&cmp_ref.name));
        match idx {
            Ok(idx) => Some(&cmps[idx]),
            Err(_) => None,
        }
    }

    pub fn cmp_def_generic_types(&self, cmp_def: &QFComponentDef) -> GenericTypeInfo {
        cmp_def
            .parts
            .iter()
            .map(|p| match p {
                QFCompomentPart::FieldRef(fld_ref) => self.fld_def(fld_ref).unwrap().generic_memeber_type_info(),
                QFCompomentPart::GroupDef(grp_def) => self.grp_def_generic_types(grp_def),
                QFCompomentPart::ComponentRef(cmp_ref) => self.cmp_def_generic_types(self.cmp_def(cmp_ref).unwrap()), // recursive}
            })
            .sum()
    }
    pub fn grp_def_generic_types(&self, grp_def: &QFGroupDef) -> GenericTypeInfo {
        grp_def
            .parts
            .iter()
            .map(|p| match p {
                QFGroupPart::FieldRef(fld_ref) => self.fld_def(fld_ref).unwrap().generic_memeber_type_info(),
                QFGroupPart::ComponentRef(cmp_ref) => self.cmp_def_generic_types(self.cmp_def(cmp_ref).unwrap()),
                QFGroupPart::GroupDef(grp_def) => self.grp_def_generic_types(grp_def), // recursive
            })
            .sum()
    }

    pub fn fld_def_plain(&self, fld_ref: &QFFieldRef) -> Option<&QFFieldDef> {
        let idx = self.fld_defs_plain.binary_search_by(|f| f.name.cmp(&fld_ref.name));
        match idx {
            Ok(idx) => Some(&self.fld_defs_plain[idx]),
            Err(_) => None,
        }
    }
    pub fn field_def_len_data(&self, fld_ref_len: &QFFieldRef) -> Option<&(QFFieldDef, QFFieldDef)> {
        assert!(
            self.fld_def(fld_ref_len).unwrap().is_type_length(),
            "Field must be of type LENGTH {:?}",
            fld_ref_len
        );

        let idx = self.fld_defs_len_data.binary_search_by(|p| p.0.name.cmp(&fld_ref_len.name));
        match idx {
            Ok(idx) => Some(&self.fld_defs_len_data[idx]),
            Err(_) => None,
        }
    }

    pub fn message_defs(&self) -> &Vec<QFMessageDef> {
        &self.msg_defs.get()
    }
    pub fn details<F: Fn(&QFMessageDef) -> bool>(&self, filter: F) -> Result<String, String> {
        let msgs = self.message_defs().iter().filter(|m| filter(m)).collect::<Vec<_>>();
        if msgs.is_empty() {
            return Err("no Quick Fix messages found matching filter".to_string());
        }
        Ok(msgs
            .iter()
            .map(|msg_def| msg_def.details(self.fld_defs.get(), self.cmp_defs.get()))
            .collect::<Vec<String>>()
            .join("\n"))
    }
    pub fn extract_rep_grp_defs(&self) -> Vec<QFRepGroupDef> {
        let mut qf_rep_grp_defs = vec![];
        qf_rep_grp_defs.extend(self.msg_defs.extract_rep_grp_defs());
        qf_rep_grp_defs.extend(self.cmp_defs.extract_rep_grp_defs());
        qf_rep_grp_defs
    }
}
impl<S: AsRef<str>> From<S> for QFModel {
    fn from(xml: S) -> QFModel {
        let mut root: QFModel = quick_xml::de::from_str(xml.as_ref()).unwrap();
        root.complete_preparation();
        root
    }
}

impl From<&QFModel> for RFModel {
    fn from(qf_model: &QFModel) -> Self {
        let mut fix_field_type_not_supported = HashSet::<String>::new();
        let mut errors = Vec::<Error>::new();

        // DEFINE FIELDS unless they are DATA or LENGTH
        let r_fld_defs_plain = qf_model
            .fld_defs_plain
            .iter()
            .filter_map(|qf_field| match RFldDefPlain::try_from(qf_field) {
                Ok(f) => Some(f),
                Err(e) => {
                    if !fix_field_type_not_supported.contains(&qf_field.r#type) {
                        fix_field_type_not_supported.insert(qf_field.r#type.to_string());
                        errors.push(e);
                    }
                    None
                }
            })
            // .filter(|rf| rf.is_some())
            // .map(|rf| rf.unwrap())
            .collect::<Vec<_>>();

        // DEFINE BINARY DATA FIELDS
        let r_fld_defs_data = qf_model
            .fld_defs_len_data
            .iter()
            .map(|(len, data)| {
                let len_id = len
                    .number
                    .parse()
                    .expect(format!("quickfix definion of field 'number' is not valid, expected usize. value: {:?}", len).as_str());
                let data_id = data
                    .number
                    .parse()
                    .expect(format!("quickfix definion of field 'number' is not valid, expected usize. value: {:?}", data).as_str());
                RFldDefData {
                    len_name: len.name.clone(),
                    len_tag: len_id,
                    data_name: data.name.clone(),
                    data_tag: data_id,
                }
            })
            .collect::<Vec<_>>();

        let mut r_fld_defs = r_fld_defs_plain.iter().map(|f| RFldDef::Plain(f.clone())).collect::<Vec<_>>();
        let r_data = r_fld_defs_data.iter().map(|f| RFldDef::Data(f.clone())).collect::<Vec<_>>();
        r_fld_defs.extend(r_data);

        let r_msg_defs = qf_model
            .message_defs()
            .iter()
            .map(
                |qf_msg_def| RFMessageDef::from((qf_msg_def, qf_model)), // Ok(r_msg_def) => Some(r_msg_def),
            )
            .collect();

        let name = format!("Fix{}{}", qf_model.major, qf_model.minor);

        let qf_rep_grp_defs = qf_model.extract_rep_grp_defs();

        let rep_grp_defs = qf_rep_grp_defs
            .iter()
            .map(|qf_rep_grp_def| RRepGrpMessageDef::from((qf_rep_grp_def, qf_model)))
            .collect::<Vec<_>>();

        let mut rf_model = Self {
            fld_defs: r_fld_defs,
            msg_defs: r_msg_defs,
            rep_grp_defs,
            errors,
            name,
        };
        rf_model.complete_preparation();
        rf_model
    }
}
