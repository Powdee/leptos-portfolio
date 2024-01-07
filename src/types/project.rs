use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectData {
    pub data: Vec<Project>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Project {
    pub name: String,
    pub description: String,
    pub information: Information,
    pub tags: Vec<String>,
    pub link: Link,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Information {
    pub about: String,
    pub timeline: String,
    pub responsibility: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Link {
    pub source: String,
    pub url: String,
}

