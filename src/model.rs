use super::schema::*;

#[derive(Queryable, Deserialize, Serialize)]
pub struct Project {
    pub title: String,
    pub short_desc: String,
    pub long_desc: String,
    pub repo: String,
}

impl From<NewProject> for Project {
    fn from(n: NewProject) -> Self {
        Project {
            title: n.title.unwrap_or_default(),
            short_desc: n.short_desc.unwrap_or_default(),
            long_desc: n.long_desc.unwrap_or_default(),
            repo: n.repo.unwrap_or_default(),
        }
    }
}

#[derive(Insertable, Serialize, Deserialize, AsChangeset, Default)]
#[table_name = "projects"]
#[primary_key(title)]
pub struct NewProject {
    pub title: Option<String>,
    pub short_desc: Option<String>,
    pub long_desc: Option<String>,
    pub repo: Option<String>,
}

impl From<Project> for NewProject {
    fn from(p: Project) -> Self {
        NewProject {
            title: Some(p.title),
            short_desc: Some(p.short_desc),
            long_desc: Some(p.long_desc),
            repo: Some(p.repo),
        }
    }
}
