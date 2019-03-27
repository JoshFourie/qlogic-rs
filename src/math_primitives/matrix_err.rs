#[derive(Debug)]
pub enum MathError
{
    NoneVal(std::option::NoneError),
    BadIndex(String),
    BadOp(String),
    BadSpec(String),
}

impl MathError 
{
    pub fn invalid_index(c: usize, r: usize, max_r: usize, max_c: usize) -> Self {
        MathError::BadIndex(format!("Invalid Index: indexed at {},{}, but the maximum input is {},{}",r,c,max_r,max_c))
    }  
    
    pub fn bad_op(e: &'static str) -> Self { MathError::BadOp(format!("{}", e)) }

    pub fn bad_spec(e: &'static str) -> Self { MathError::BadSpec(format!("{}", e)) }

    pub fn as_result<T>(self) -> Result<T, Self> { Err(self) }
}

impl From<std::option::NoneError> for MathError
{
    fn from(e: std::option::NoneError) -> Self { MathError::NoneVal(e) }
}