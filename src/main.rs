mod variables;
mod data_types;
mod functions;
mod control_flow;
mod ownership;
mod structs;

enum ExerciseMode{

    Variables,
    DataTypes,
    Functions,
    ControlFlow,
    Ownership,
    Structs
}

const EXERCISE_MODE:ExerciseMode = ExerciseMode::Structs;

fn main() {


    match EXERCISE_MODE {
        ExerciseMode::Variables => variables::do_variables(),
        ExerciseMode::DataTypes => data_types::do_data_types(),
        ExerciseMode::Functions => functions::do_functions(),
        ExerciseMode::ControlFlow => control_flow::do_control_flow(),
        ExerciseMode::Ownership => ownership::do_ownership(),
        ExerciseMode::Structs => structs::do_structs(),
        _ => println!("Exercise mode not implemented yet."),
    }
}

