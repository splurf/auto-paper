use {
    crate::Application,
    std::{
        env::{args_os, current_dir, set_current_dir},
        fs::{read_dir, remove_file, DirEntry, File},
        io::{Result, Write},
        path::PathBuf,
    },
};

pub fn current_application(projects: Vec<String>) -> (String, Application) {
    || -> Result<(String, Application)> {
        let path = match || -> Option<PathBuf> {
            let inner = PathBuf::from(args_os().nth(1)?.to_str()?);
            assert!(inner.is_dir(), "Not a valid directory");
            Some(inner)
        }() {
            Some(path) => path,
            None => current_dir()?,
        };

        set_current_dir(path)?;

        let jar = PaperMcJar::from(
            read_dir(current_dir()?)?
                .filter_map(std::io::Result::ok)
                .find(|de| de.file_name().to_string_lossy().ends_with(".jar"))
                .expect("Jar file not found"),
        );
        assert!(projects.contains(&jar.project));
        Ok((jar.project.clone(), jar.into()))
    }()
    .expect("Io Error")
}

pub struct PaperMcJar {
    project: String,
    version: String,
    build: u16,
}

impl PaperMcJar {
    pub fn project(&self) -> String {
        self.project.clone()
    }

    pub fn version(&self) -> String {
        self.version.clone()
    }

    pub fn build(&self) -> u16 {
        self.build
    }
}

impl From<DirEntry> for PaperMcJar {
    fn from(de: DirEntry) -> Self {
        let jar = de.file_name().to_string_lossy().to_string();

        let info = || -> Option<(String, String, u16)> {
            let base = &jar[..jar.len() - 4];

            let f = base.find("-")?;
            let l = base.rfind("-")?;

            let project = &base[..f];
            let version = &base[f + 1..l];
            let build = &base[l + 1..];

            Some((
                project.to_string(),
                version.to_string(),
                build.parse().expect("Incompatible jar file"),
            ))
        }()
        .expect("Invalid jar file name");

        Self {
            project: info.0,
            version: info.1,
            build: info.2,
        }
    }
}

pub fn replace_jar(old: String, new: String, bytes: Vec<u8>) -> Result<()> {
    let mut file = File::create(new)?;
    file.write_all(&bytes)?;
    remove_file(old)
}
