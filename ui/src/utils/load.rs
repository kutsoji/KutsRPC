use leptos::{
    SignalGetUntracked,
    SignalSet,
};

use crate::{
    invoke,
    Config,
    GlobalState,
    Preset,
};

pub async fn config(gs_opt: Option<GlobalState>) -> Result<Config, String> {
    match invoke!("load_config", Config) {
        Ok(c) => {
            if let Some(gs) = gs_opt {
                gs.app_id.set(c.app_id.clone());
                gs.remember.set(c.remember);
                gs.presets_dir.set(c.presets_dir.clone());
                gs.minimize.set(c.minimize);
            }
            Ok(c)
        }
        Err(e) => Err(e.to_string()),
    }
}

#[allow(non_snake_case)]
pub async fn presets(gs: GlobalState) -> Result<Vec<Preset>, String> {
    match invoke!("load_presets", {presetsDir: String = gs.presets_dir.get_untracked().unwrap()}, Vec<Preset>)
    {
        Ok(p) => {
            gs.presets.set(p.clone());
            Ok(p)
        }
        Err(e) => Err(e.to_string()),
    }
}
