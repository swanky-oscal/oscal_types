use crate::error::Error;

pub trait Validate {
    /// Test the value to determine whether it is valid
    fn validate(_value: &str) -> Result<(), Error> {
        Ok(())
    }
}
