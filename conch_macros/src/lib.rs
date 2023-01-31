use conch_common::NuError;
use nu_command::create_default_context;
use proc_macro;
use quote::quote;
use proc_macro_error::{proc_macro_error, abort_call_site};
use nu_parser::parse;


fn syntax_check(input: &String) -> Result<(), NuError> {
    let engine_state = create_default_context();
    let mut working_set = nu_protocol::engine::StateWorkingSet::new(&engine_state);
    let (_, error) = parse(&mut working_set, None, input.as_bytes(), false, &[]);

    match error {
        Some(err) => Err(err.into()),
        None => Ok(())
    }
}

#[proc_macro]
#[proc_macro_error]
pub fn sh(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // This is hilariously tacky.
    let script = input
        .to_string()
        .split("\n")
        .collect::<Vec<_>>()
        .join(" ");

    if let Err(err) = syntax_check(&script) {
        // TODO: Error printing kinda sucks.
        // I'm having a hard time understanding how to take
        // advantage of
        abort_call_site!("{}", err);
    }

    quote!({
        use nu_cli::{get_init_cwd, gather_parent_env_vars};
        use nu_command::create_default_context;
        use nu_protocol::{PipelineData, engine::{StateWorkingSet}};
        use nu_engine::{convert_env_values, eval_block};
        use nu_parser::{parse};
        use conch_common::NuResult;

        (|| -> NuResult {
            let init_cwd = get_init_cwd();
            let mut engine_state = create_default_context();
            let input = PipelineData::empty();
            gather_parent_env_vars(&mut engine_state, &init_cwd);
            let mut stack = nu_protocol::engine::Stack::new();
    
            // Translate environment variables from Strings to Values
            if let Some(err) = convert_env_values(&mut engine_state, &stack) {
                return Err(err.into());
            }
    
            let mut working_set = StateWorkingSet::new(&engine_state);
            let (block, err) = parse(&mut working_set, None, #script.as_bytes(), false, &[]);
            if let Some(e) = err {
                return Err(e.into())
            }
            
            engine_state.merge_delta(working_set.render())?;
            
            match eval_block(&mut engine_state, &mut stack, &block, input, false, false) {
                Ok(pipeline_data) => Ok(pipeline_data),
                Err(err) => Err(err.into())
            }
        })()
    }).into()
}
