//! Instructions for endoscaling public inputs.
use halo2_proofs::{
    circuit::{AssignedCell, Layouter},
    plonk::Error,
};
use pasta_curves::arithmetic::CurveAffine;

/// Instructions to map bitstring public inputs to and from endoscalars.
///
/// TODO: Replace N = 2^K once we have const evaluable.
pub trait PubInputsInstructions<C: CurveAffine, const K: usize, const N: usize>
where
    C::Base: PubInputsLookup<K, N>,
{
    /// A K-bit word.
    type Word;

    /// A commitment to a bitstring.
    type Commitment;

    /// Gets a bitstring from its endoscalar representation.
    ///
    /// These endoscalars are provided as the cells in the public input column.
    fn get_bitstring(
        &self,
        layouter: impl Layouter<C::Base>,
        row: usize,
    ) -> Result<Self::Word, Error>;

    /// Compute commitment to a K-bit word using the endoscaling algorithm.
    fn commit_word(
        &self,
        layouter: impl Layouter<C::Base>,
        base: C,
        word: &Self::Word,
        prev_acc: Option<Self::Commitment>,
    ) -> Result<Self::Commitment, Error>;

    /// Compute commitment to a full-width field element using the endoscaling algorithm.
    fn commit_field_elem<L: Layouter<C::Base>, const NUM_BITS: usize>(
        &self,
        layouter: L,
        bases: Vec<C>,
        field_elem: AssignedCell<C::Base, C::Base>,
        prev_acc: Option<Self::Commitment>,
    ) -> Result<Self::Commitment, Error>;
}

/// A trait providing the lookup table for decoding public inputs.
pub trait PubInputsLookup<const K: usize, const N: usize>
where
    Self: std::marker::Sized,
{
    /// A lookup table mapping `K`-bit values to endoscalars.
    fn table() -> [([bool; K], Self); N];
}
