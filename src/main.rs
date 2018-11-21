#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use] extern crate rocket_contrib;
extern crate rodio;
extern crate toml;
extern crate serde;
#[macro_use] extern crate serde_derive;

#[macro_use] mod macros;
mod sound;
mod util;

use rocket_contrib::{Json, Value};
use rocket::State;
use std::collections::HashMap;
use util::result;
use std::boxed::Box;

#[derive(Deserialize)]
struct Sound {
    kind: String
}

struct Config(HashMap<String, String>);


#[post("/play", data = "<sound>")]
fn play(sound: Json<Sound>, config: State<Config>) -> Json<Value> {
    if let Some(ref file) = config.0.get(&sound.kind) {
        sound::play(file);
        Json(json!({ "status": "ok" }))
    }
    else {
        Json(json!({ "status": "error", "message": format!("No such sound! {}", sound.kind) }))
    }
}


fn load_sounds() -> result::Result<HashMap<String, String>> {
    let val = util::toml::toml_value_from_file("sounds.toml")?;

    if let toml::Value::Table(ref table) = val["sounds"] {
        let mut sounds = HashMap::new();
        for (k, v) in table.iter() {
            let name = k.clone();
            let file = v.as_str().unwrap().to_string();
            println!("Load the sound! {} {}", name, file);
            sounds.insert(name, file);
        }
        Ok(sounds)
    } else {
        Err(Box::new(result::Error))
    }
}

fn main() {
    let sounds = load_sounds().expect("valid sounds");
    rocket::ignite()
        .mount("/", routes![play])
        .manage(Config(sounds))
        .launch();
}
