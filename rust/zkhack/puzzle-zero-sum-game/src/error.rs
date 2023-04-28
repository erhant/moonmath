#[derive(Debug)]
pub enum Error<E> {
    OpeningError,
    PolynomialCommitmentError(E),
    IncorrectSum,
}

impl<E> Error<E> {
    pub fn from_pc_err(err: E) -> Self {
        Error::PolynomialCommitmentError(err)
    }
}
