pub enum NString {
    Single(String),
    Multiple(Vec<String>),
}

pub enum GroupBy {
    FileExtension,
    Size,
    CreatedAt,
    LastModifiedAt,
}
