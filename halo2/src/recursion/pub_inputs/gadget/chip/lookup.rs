use crate::recursion::pub_inputs::primitive::{endoscale_scalar, i2lebsp};
use halo2_proofs::{
    circuit::Layouter,
    plonk::{ConstraintSystem, Error, TableColumn},
};
use pasta_curves::arithmetic::FieldExt;
use std::marker::PhantomData;

#[derive(Copy, Clone, Debug)]
pub struct TableConfig<F: FieldExt, const K: usize> {
    pub(in crate::recursion) bits: TableColumn,
    pub(in crate::recursion) endoscalar: TableColumn,
    _marker: PhantomData<F>,
}

impl<F: FieldExt, const K: usize> TableConfig<F, K> {
    #[allow(dead_code)]
    pub fn configure(meta: &mut ConstraintSystem<F>) -> Self {
        TableConfig {
            bits: meta.lookup_table_column(),
            endoscalar: meta.lookup_table_column(),
            _marker: PhantomData,
        }
    }

    #[allow(dead_code)]
    pub fn load(&self, layouter: &mut impl Layouter<F>) -> Result<(), Error> {
        layouter.assign_table(
            || "endoscalar_map",
            |mut table| {
                for index in 0..(1 << K) {
                    table.assign_cell(|| "bits", self.bits, index, || Ok(F::from(index as u64)))?;
                    table.assign_cell(
                        || "endoscalar",
                        self.endoscalar,
                        index,
                        || Ok(endoscale_scalar::<F, K>(i2lebsp(index as u64))),
                    )?;
                }
                Ok(())
            },
        )
    }
}
