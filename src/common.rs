pub type MathResult<T1> = Result<T1, MathError>;

#[derive(Debug)]
#[derive(Clone)]
pub enum MathError {
    Overflow,
    Underflow,
    DivByZero,
    IncompatiblePrecision
}