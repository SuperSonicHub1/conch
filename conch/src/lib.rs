pub use conch_macros::sh;
pub use conch_common::{NuError, NuResult};

#[cfg(test)]
mod tests {
    use super::*;
    use nu_protocol::{PipelineData, Value};

    #[test]
    fn it_works() {
        let result: NuResult = sh! {
            42 | describe;
        };
        assert!(result.is_ok());
        if let Ok(PipelineData::Value(Value::String {val, ..}, _)) = result {
            assert_eq!(val, "int");
        } else {
            panic!("Result is not a string: {:?}", result);
        }
    }
}


