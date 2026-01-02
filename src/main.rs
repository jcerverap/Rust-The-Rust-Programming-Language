mod variables;
mod data_types;
mod functions;
mod control_flow;
mod ownership;

enum ExerciseMode{

    Variables,
    DataTypes,
    Functions,
    ControlFlow,
    Ownership,
}

const EXERCISE_MODE:ExerciseMode = ExerciseMode::Ownership;

fn main() {


    match EXERCISE_MODE {
        ExerciseMode::Variables => variables::do_variables(),
        ExerciseMode::DataTypes => data_types::do_data_types(),
        ExerciseMode::Functions => functions::do_functions(),
        ExerciseMode::ControlFlow => control_flow::do_control_flow(),
        ExerciseMode::Ownership => ownership::do_ownership(),
        _ => println!("Exercise mode not implemented yet."),
    }
}

