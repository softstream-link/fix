use crate::schema::rust::model::{
    field::{RFldDef, RFldDefRepGroup},
    message_member::{RMessageMember, RMessageMembers},
};

use super::{
    field_def::QFFieldDef,
    message_def::qf_fld_ref_2_r_msg_member_plain_or_data,
    refs::{QFComponentRef, QFFieldRef},
    repgroup_def::{QFGroupDef, QFRepGroupDef},
    root::QFModel,
};
use serde::{Deserialize, Serialize};
use std::{iter::Sum, vec};

#[derive(Debug, Deserialize)]
pub struct QFComponentDefs {
    #[serde(rename = "$value")]
    #[serde(default)] // <components> is empty in Fix prior to 4.4
    defs: Vec<QFComponentDef>,
}
impl QFComponentDefs {
    pub fn sort(&mut self) {
        self.defs.sort_by(|a, b| a.name.cmp(&b.name));
    }
    pub(super) fn get(&self) -> &Vec<QFComponentDef> {
        &self.defs
    }

    pub fn extract_rep_grp_defs(&self) -> Vec<QFRepGroupDef> {
        self.defs
            .iter()
            .filter_map(|cmp_def| match cmp_def.category() {
                QFComponentCategory::SingleRepGrpDefOnly(_) => {
                    let xml = quick_xml::se::to_string(cmp_def).unwrap();
                    Some(QFRepGroupDef {
                        rep_grp_name: cmp_def.name.to_owned(),
                        grp_def: cmp_def.get_single_group_def(),
                        xml,
                    })
                }
                _ => None,
            })
            .collect()
    }
}

// #[serde(rename = "field")]
// Field(QFFieldRef), // <field name='IOINaturalFlag' required='N' />
// #[serde(rename = "group")]
// Group(QFGroupRef), // <group name='NoIOIQualifiers' required='N'> <field name='IOIQualifier' required='N' /> .... <field ..../> </group>
// #[serde(rename = "component")]
// Component(QFComponentRef), // <component name='Instrument' required='Y' />
#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum QFCompomentPart {
    #[serde(rename = "field")]
    FieldRef(QFFieldRef), // <field name='IOINaturalFlag' required='N' />
    #[serde(rename = "group")]
    GroupDef(QFGroupDef),
    #[serde(rename = "component")]
    ComponentRef(QFComponentRef), // <component name='Instrument' required='Y' />
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename = "component")]
pub struct QFComponentDef {
    #[serde(rename = "@name")]
    pub name: String,
    // #[serde(rename = "@required")]
    // pub required: String,
    #[serde(rename = "$value")]
    pub parts: Vec<QFCompomentPart>,
}
impl QFComponentDef {
    pub fn details(&self, fld_defs: &Vec<QFFieldDef>, cmp_defs: &Vec<QFComponentDef>, pad: usize) -> String {
        let mut f = String::new();
        use std::fmt::Write;
        for part in &self.parts {
            match part {
                QFCompomentPart::FieldRef(fld_ref) => {
                    write!(f, "{}", fld_ref.details(fld_defs, pad)).unwrap();
                }
                QFCompomentPart::GroupDef(grp_def) => {
                    write!(f, "{}", grp_def.details(fld_defs, cmp_defs, pad)).unwrap();
                }
                QFCompomentPart::ComponentRef(cmp_ref) => {
                    write!(f, "{}", cmp_ref.details(fld_defs, cmp_defs, pad)).unwrap();
                }
            }
        }
        f
    }

    pub fn is_single_group(&self) -> bool {
        matches!(self.category(), QFComponentCategory::SingleRepGrpDefOnly(_))
    }
    pub fn is_only_fields(&self) -> bool {
        matches!(self.category(), QFComponentCategory::FieldRefsOnly(_))
    }
    pub fn parts_counter(&self) -> PartsCounter {
        self.parts
            .iter()
            .map(|g| match g {
                QFCompomentPart::FieldRef(_) => PartsCounter { grp: 0, fld: 1, cmp: 0 },
                QFCompomentPart::GroupDef(_) => PartsCounter { grp: 1, fld: 0, cmp: 0 },
                QFCompomentPart::ComponentRef(_) => PartsCounter { grp: 0, fld: 0, cmp: 1 },
            })
            .sum::<PartsCounter>()
    }
    pub fn category(&self) -> QFComponentCategory {
        let cnt = self.parts_counter();

        match cnt {
            PartsCounter { grp, fld, cmp } if grp == 0 && fld > 0 && cmp == 0 => QFComponentCategory::FieldRefsOnly(self),
            PartsCounter { grp, fld, cmp } if grp == 1 && fld == 0 && cmp == 0 => QFComponentCategory::SingleRepGrpDefOnly(self),
            PartsCounter { grp, fld, cmp } if grp == 0 && (fld > 0 || cmp > 0) => QFComponentCategory::FieldComponentRefsOnly(self),
            _ => panic!(
                "QuickFix schema has unsupported definition Component: '{}' it has multipe groups or group is mixed with other parts. counts: {:?}",
                self.name, cnt
            ),
        }
    }
    pub fn get_single_group_def(&self) -> &QFGroupDef {
        match self.is_single_group() {
            true => {
                let grp = self.parts.iter().find_map(|part| match part {
                    QFCompomentPart::GroupDef(grp_def) => Some(grp_def),
                    _ => None,
                });
                grp.unwrap()
            }
            false => panic!("QuickFix component is not a groupComponent: '{}' it is not a group", self.name),
        }
    }
    pub fn get_only_fields(&self) -> Vec<&QFFieldRef> {
        match self.is_only_fields() {
            true => self
                .parts
                .iter()
                .filter_map(|part| match part {
                    QFCompomentPart::FieldRef(fld_ref) => Some(fld_ref),
                    _ => None,
                })
                .collect(),
            false => panic!("QuickFix component is not a fieldComponent: '{}' it is not a group", self.name),
        }
    }
}

impl From<(&QFComponentDef, &QFModel)> for RMessageMembers {
    fn from(value: (&QFComponentDef, &QFModel)) -> Self {
        let mut members = vec![];
        let mut errors: Vec<super::Error> = vec![];
        let (qf_cmp_def, qf_model) = value;
        for part in &qf_cmp_def.parts {
            match part {
                QFCompomentPart::FieldRef(fld_ref) => {
                    let member = qf_fld_ref_2_r_msg_member_plain_or_data(&qf_cmp_def.name, fld_ref, qf_model);
                    match member {
                        Ok(Some(member)) => members.push(member),
                        Ok(None) => (),
                        Err(e) => {
                            errors.push(e);
                        }
                    }
                }
                QFCompomentPart::GroupDef(grp_def) => {
                    let generic_info = qf_model.grp_def_generic_types(grp_def);
                    let qf_fld_def = qf_model.fld_def(&grp_def.as_field_ref()).unwrap();

                    members.push(RMessageMember {
                        member: RFldDef::RepGroup(RFldDefRepGroup {
                            name: qf_cmp_def.name.to_owned(),
                            tag: qf_fld_def.number.parse().unwrap(),
                            generic_info,
                        }),
                        required: grp_def.required == "Y",
                    });
                }
                QFCompomentPart::ComponentRef(cmp_ref) => {
                    let cmp_def = qf_model.cmp_def(cmp_ref).unwrap();
                    let r_message_members = RMessageMembers::from((cmp_def, qf_model));
                    members.extend(r_message_members.members);
                    errors.extend(r_message_members.errors);
                }
            }
        }

        RMessageMembers { members, errors }
    }
}
#[derive(Debug)]
pub enum QFComponentCategory<'a> {
    FieldRefsOnly(&'a QFComponentDef),
    /// ```xml
    /// <component name='LegStipulations'>
    ///     <group name='NoLegStipulations' required='N'>
    ///         <field name='LegStipulationType' required='N' />
    ///         <field name='LegStipulationValue' required='N' />
    ///         <!-- ... might have nested groups -->
    ///     </group>
    /// </component>
    /// ```
    SingleRepGrpDefOnly(&'a QFComponentDef),
    FieldComponentRefsOnly(&'a QFComponentDef),
}
#[derive(Debug)]
pub struct PartsCounter {
    pub grp: usize,
    pub fld: usize,
    pub cmp: usize,
}
impl Sum for PartsCounter {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(PartsCounter { grp: 0, fld: 0, cmp: 0 }, |acc, x| PartsCounter {
            grp: acc.grp + x.grp,
            fld: acc.fld + x.fld,
            cmp: acc.cmp + x.cmp,
        })
    }
}
