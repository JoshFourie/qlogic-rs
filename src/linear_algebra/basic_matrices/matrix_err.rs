#[derive(Debug)]
pub enum MathError
{
    NoneVal(std::option::NoneError),
    BadIndex(String),
    BadOp(String),
    BadSpec(String),
    BadInput(String)
}

impl MathError 
{
    pub fn invalid_index(r: usize, c: usize, max_r: usize, max_c: usize) -> Self {
        MathError::BadIndex(format!("Invalid Index: indexed at {},{}, but the maximum input is {},{}",r,c,max_r-1,max_c-1))
    }  
    
    pub fn bad_op(e: &'static str) -> Self { MathError::BadOp(format!("{}", e)) }

    pub fn bad_spec(e: &'static str) -> Self { MathError::BadSpec(format!("{}", e)) }

    pub fn bad_input(e: &'static str) -> Self { MathError::BadInput(format!("{}", e)) }

    pub fn as_result<T>(self) -> Result<T, Self> { Err(self) }
}

impl From<std::option::NoneError> for MathError
{
    fn from(e: std::option::NoneError) -> Self { MathError::NoneVal(e) }
}