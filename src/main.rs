mod api;
mod jar;
mod maps;

use {
    api::*,
    jar::*,
    maps::*,
    std::{thread::sleep, time::Duration},
};

fn update(pt: String, app: &mut Application) -> Result<()> {
    let mut api = Api::new();

    let project = api.get("projects", None)?.json::<Projects>()?.get(&pt);

    let version = api.get(&project, None)?.json::<Project>()?.version();

    let build = api
        .get(&version, Some("versions"))?
        .json::<Versions>()?
        .build();

    let new_app = api
        .get(&build.to_string(), Some("builds"))?
        .json::<Build>()?
        .application();

    if app.sha256() != new_app.sha256() {
        replace_jar(
            app.name(),
            new_app.name(),
            api.get(&new_app.name(), Some("downloads"))?
                .bytes()?
                .to_vec(),
        )
        .unwrap_or_default();
        *app = new_app
    }
    Ok(())
}

fn main() -> Result<()> {
    let (pt, mut app) = current_application(
        Api::get_once("projects", None)?
            .json::<Projects>()?
            .projects(),
    );

    //  Update every day
    const DUR: Duration = Duration::from_secs(86400);

    loop {
        if let Err(error) = update(pt.clone(), &mut app) {
            println!("{}", error)
        }
        sleep(DUR)
    }
}
