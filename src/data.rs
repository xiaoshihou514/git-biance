use time::OffsetDateTime;

pub struct Author {
    pub name: String,
    pub email: String,
}

pub struct Commit {
    pub author: Author,
    pub time: OffsetDateTime,
}

pub struct DetailedCommit {
    pub commit: Commit,
    pub insertions: i64,
    pub deletions: i64,
}
