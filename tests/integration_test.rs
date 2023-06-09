use picocadrs;

use std::{fs, env};
use picocadrs::assets::{PicoColor, Vector};
use picocadrs::save::PicoSave;

#[test]
#[ignore]
fn create_and_edit_file() {
    // generate path
    let mut path = env::var("picocad_path").expect("Invalid environment variable.");
    path.push_str("plane.txt");

    // read file and create save
    let mut save = PicoSave::from(fs::read_to_string(path.clone()).expect("Couldn't read file."));

    // set bg color
    save.header.bg_color = PicoColor::DarkGreen;

    // edit mesh
    let mesh = save.meshes.get_mut(0).unwrap();
    // rename the mesh to "first_mesh"
    mesh.name = "first_mesh".to_string();
    // set mesh origin to 0|3|0
    mesh.pos = Vector::new(0.0, 3.0, 0.0);

    // write save to file
    fs::write(path, save.to_string()).expect("Couldn't write to file.");
}

#[test]
#[ignore]
fn vector_methods() {
    // generate path
    let mut path = env::var("picocad_path").expect("Invalid environment variable.");
    path.push_str("plane.txt");

    // read file and create save
    let mut save = PicoSave::from(fs::read_to_string(path.clone()).expect("Couldn't read file."));

    let rot_vect = Vector::new(0.125, 0.125, 0.0);
    let mesh = save.meshes.get_mut(0).unwrap();
    for v in mesh.vertices.iter_mut() { v.rotate(&rot_vect); }

    // write save to file
    fs::write(path, save.to_string()).expect("Couldn't write to file.");
}