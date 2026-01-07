use strum::IntoEnumIterator; 
use strum::EnumIter; 

mod variables;
mod data_types;
mod functions;
mod control_flow;
mod ownership;
mod structs;
mod enums;
mod vectors;
mod hash_maps;
mod strings;

#[derive(Debug, EnumIter)]
enum ExerciseMode {

    Variables,
    DataTypes,
    Functions,
    ControlFlow,
    Ownership,
    Structs,
    Enums,
    Vectors,
    HashMaps,
    Strings,
}

impl ExerciseMode {

    fn do_exercise(&self) {

        match self {
            ExerciseMode::Variables => variables::do_variables(),
            ExerciseMode::DataTypes => data_types::do_data_types(),
            ExerciseMode::Functions => functions::do_functions(),
            ExerciseMode::ControlFlow => control_flow::do_control_flow(),
            ExerciseMode::Ownership => ownership::do_ownership(),
            ExerciseMode::Structs => structs::do_structs(),
            ExerciseMode::Enums => enums::do_enums(),
            ExerciseMode::Vectors => vectors::do_vectors(),
            ExerciseMode::HashMaps => hash_maps::do_hash_maps(),
            ExerciseMode::Strings => strings::do_strings(),
            _ => println!("Exercise mode not implemented yet."),

        }
    }
}

fn main() {

    for mode in ExerciseMode::iter() {

        mode.do_exercise();
    }        

}

