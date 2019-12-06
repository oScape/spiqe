mod error_handler;

use error_handler::MyError;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::process::Command;

#[derive(Serialize, Deserialize, Clone)]
struct Occurrence {
    children: Option<Vec<Occurrence>>,
    identifier: Option<String>,
    kind: String,
    body: Option<String>,
    loc: Location,
}

#[derive(Serialize, Deserialize, Clone)]
struct Location {
    start: Coordonate,
    end: Coordonate,
}

#[derive(Serialize, Deserialize, Clone)]
struct Coordonate {
    offset: i32,
    line: i32,
    column: i32,
}

fn main() -> Result<(), MyError> {
    let output = Command::new("./ext_libs/qml-parser")
        .arg("./input/test_es6.qml")
        .output()
        .expect("bin command failed to start");

    let json_result: Occurrence = serde_json::from_str(&String::from_utf8(output.stdout).unwrap())?;

    let occur = recursive_finder(&json_result.children.unwrap());

    let mut f = File::create("./output/js_es6.js")?;
    f.write_all(&occur.body.unwrap().as_bytes())?;
    f.sync_all()?;

    Command::new("yarn")
        .arg("babel")
        .arg("output/js_es6.js")
        .arg("--out-file")
        .arg("output/js_es5.js")
        .arg("--presets=@babel/preset-env")
        .output()
        .expect("failed on babelification");

    let test_es6 = fs::read_to_string("input/test_es6.qml").unwrap();
    let js_es5 = fs::read_to_string("output/js_es5.js").unwrap();
    let new_test_es6 = [
        test_es6.get(..(occur.loc.start.offset as usize)).unwrap(),
        &js_es5[..],
        test_es6.get((occur.loc.end.offset as usize)..).unwrap(),
    ]
    .join(" ");

    let mut f = File::create("./output/test_es6.qml")?;
    f.write_all(new_test_es6.as_bytes())?;
    f.sync_all()?;

    Ok(())
}

fn recursive_finder(children: &Vec<Occurrence>) -> Occurrence {
    for child in children {
        if child.kind == "Function" {
            return child.clone();
        } else if child.children.is_some() {
            return recursive_finder(&child.children.clone().unwrap());
        }
    }
    return children[0].clone();
}
