pub trait FieldMeta {
    const TAG: usize;
    const TAG_NAME: &'static str;
    const SHORT_NAME: &'static str;
    const SHORT_NAME_WITH_TAG_NAME: &'static str;
}
