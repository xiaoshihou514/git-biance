use time::OffsetDateTime;

pub struct Author {
    pub name: String,
    pub email: String,
}

pub struct Commit {
    pub author: Author,
    pub time: OffsetDateTime,
}
