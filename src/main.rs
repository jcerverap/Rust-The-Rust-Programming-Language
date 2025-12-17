mod variables;
mod data_types;

//use variables::do_variables;
//use data_types::do_data_types;

enum ExerciseMode{

    Variables,
    DataTypes,
    Functions,
    ControlFlow,
}

const EXERCISE_MODE:ExerciseMode = ExerciseMode::DataTypes;

fn main() {


    match EXERCISE_MODE {
        ExerciseMode::Variables => variables::do_variables(),
        ExerciseMode::DataTypes => data_types::do_data_types(),
//        ExerciseMode::Functions => functions::do_functions(),
//        ExerciseMode::ControlFlow => control_flow::do_control_flow(),
        _ => println!("Exercise mode not implemented yet."),
    }
}

