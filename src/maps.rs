use {
    crate::{Api, PaperMcJar, Result},
    serde::{Deserialize, Serialize},
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Projects {
    projects: Vec<String>,
}

impl Projects {
    pub fn get(self, pt: &str) -> String {
        self.projects
            .into_iter()
            .find(|p| p == pt)
            .expect("Invalid project type")
    }

    pub fn projects(self) -> Vec<String> {
        self.projects
    }
}

#[derive(Serialize, Deserialize)]
pub struct Project {
    versions: Vec<String>,
}

impl Project {
    pub fn version(self) -> String {
        self.versions[self.versions.len() - 1].clone()
    }
}

#[derive(Serialize, Deserialize)]
pub struct Versions {
    builds: Vec<u16>,
}

impl Versions {
    pub fn build(self) -> u16 {
        self.builds[self.builds.len() - 1]
    }
}

#[derive(Serialize, Deserialize)]
pub struct Build {
    downloads: Downloads,
}

impl Build {
    pub fn application(self) -> Application {
        self.downloads.application
    }
}

#[derive(Serialize, Deserialize)]
pub struct Downloads {
    application: Application,
}

#[derive(Serialize, Deserialize)]
pub struct Application {
    name: String,
    sha256: String,
}

impl Application {
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn sha256(&self) -> String {
        self.sha256.clone()
    }
}

impl From<PaperMcJar> for Application {
    fn from(jar: PaperMcJar) -> Self {
        move || -> Result<Self> {
            let mut api = Api::new();

            let project = api
                .get("projects", None)?
                .json::<Projects>()?
                .projects
                .into_iter()
                .find(|p| p == &jar.project())
                .expect("Invalid Platform");

            let version = api
                .get(&project, None)?
                .json::<Project>()?
                .versions
                .into_iter()
                .find(|v| v == &jar.version())
                .expect("Invalid Version");

            let build = api
                .get(&version, Some("versions"))?
                .json::<Versions>()?
                .builds
                .into_iter()
                .find(|b| b == &jar.build())
                .expect("Invalid Build");

            Ok(api
                .get(&build.to_string(), Some("builds"))?
                .json::<Build>()?
                .application())
        }()
        .expect("Error retrieving application")
    }
}
