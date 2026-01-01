mod variables;
mod data_types;
mod functions;
mod control_flow;

enum ExerciseMode{

    Variables,
    DataTypes,
    Functions,
    ControlFlow,
}

const EXERCISE_MODE:ExerciseMode = ExerciseMode::ControlFlow;

fn main() {


    match EXERCISE_MODE {
        ExerciseMode::Variables => variables::do_variables(),
        ExerciseMode::DataTypes => data_types::do_data_types(),
        ExerciseMode::Functions => functions::do_functions(),
        ExerciseMode::ControlFlow => control_flow::do_control_flow(),
        _ => println!("Exercise mode not implemented yet."),
    }
}

