use super::{
    message::{MessageTokenParts, RFMessageDef},
    root::RFModel,
};

#[derive(Debug)]
pub struct RRepGrpMessageDef(pub RFMessageDef);

impl From<(&RRepGrpMessageDef, &RFModel)> for MessageTokenParts {
    fn from(value: (&RRepGrpMessageDef, &RFModel)) -> Self {
        let (r_rep_grp_msg_def, rf_model) = value;
        MessageTokenParts::from((&r_rep_grp_msg_def.0, rf_model))
    }
}
