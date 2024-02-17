use plonky2::{
    field::extension::Extendable,
    hash::hash_types::{HashOut, HashOutTarget, RichField},
    iop::{
        target::{BoolTarget, Target},
        witness::Witness,
    },
    plonk::circuit_builder::CircuitBuilder,
};

use super::{
    builder::select::CircuitBuilderSelectHelpers,
    hash::traits::hasher::ToTargets,
    u32::{arithmetic_u32::U32Target, witness::WitnessU32},
    WHashOut,
};

pub trait SwappableTarget {
    fn swap<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
        swap: BoolTarget,
        left: Self,
        right: Self,
    ) -> Self;
}
pub trait ConnectableTarget {
    fn connect<F: RichField + Extendable<D>, const D: usize>(
        &self,
        builder: &mut CircuitBuilder<F, D>,
        connect_value: Self,
    );
}

pub trait CreatableTarget {
    fn create_virtual<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
    ) -> Self;
}


pub trait HashableTarget<Hash> {
    fn to_hash_target<F: RichField + Extendable<D>, const D: usize>(
        &self,
        builder: &mut CircuitBuilder<F, D>,
    ) -> Hash;
}

impl<T: Copy> HashableTarget<T> for T {
    fn to_hash_target<F: RichField + Extendable<D>, const D: usize>(
        &self,
        _: &mut CircuitBuilder<F, D>,
    ) -> Self {
        *self
    }
}

pub trait WitnessValueFor<T, F: RichField, const BIG_ENDIAN: bool = true> {
    fn set_for_witness(&self, witness: &mut impl Witness<F>, target: T);
}

// HashOutTarget
impl CreatableTarget for HashOutTarget {
    fn create_virtual<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
    ) -> Self {
        builder.add_virtual_hash()
    }
}
impl ConnectableTarget for HashOutTarget {
    fn connect<F: RichField + Extendable<D>, const D: usize>(
        &self,
        builder: &mut CircuitBuilder<F, D>,
        connect_value: Self,
    ) {
        builder.connect_hashes(*self, connect_value)
    }
}
impl SwappableTarget for HashOutTarget {
    fn swap<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
        swap: BoolTarget,
        left: Self,
        right: Self,
    ) -> Self {
        builder.select_hash(swap, right, left)
    }
}

impl<F: RichField> WitnessValueFor<HashOutTarget, F, true> for HashOut<F> {
    fn set_for_witness(&self, witness: &mut impl Witness<F>, target: HashOutTarget) {
        witness.set_hash_target(target, *self)
    }
}

impl<F: RichField> WitnessValueFor<HashOutTarget, F, true> for WHashOut<F> {
    fn set_for_witness(&self, witness: &mut impl Witness<F>, target: HashOutTarget) {
        witness.set_hash_target(target, self.0)
    }
}

// Target
impl CreatableTarget for Target {
    fn create_virtual<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
    ) -> Self {
        builder.add_virtual_target()
    }
}
impl ConnectableTarget for Target {
    fn connect<F: RichField + Extendable<D>, const D: usize>(
        &self,
        builder: &mut CircuitBuilder<F, D>,
        connect_value: Self,
    ) {
        builder.connect(*self, connect_value)
    }
}
impl SwappableTarget for Target {
    fn swap<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
        swap: BoolTarget,
        left: Self,
        right: Self,
    ) -> Self {
        builder.select(swap, right, left)
    }
}
impl<F: RichField> WitnessValueFor<Target, F, true> for F {
    fn set_for_witness(&self, witness: &mut impl Witness<F>, target: Target) {
        witness.set_target(target, *self)
    }
}

impl<F: RichField> WitnessValueFor<U32Target, F, true> for u32 {
    fn set_for_witness(&self, witness: &mut impl Witness<F>, target: U32Target) {
        witness.set_u32_target(target, *self)
    }
}

impl<F: RichField> WitnessValueFor<BoolTarget, F, true> for bool {
    fn set_for_witness(&self, witness: &mut impl Witness<F>, target: BoolTarget) {
        witness.set_bool_target(target, *self)
    }
}


impl ToTargets for &[Target] {
    fn to_targets(&self) -> Vec<Target> {
        self.to_vec()
    }
}

impl ToTargets for &[U32Target] {
    fn to_targets(&self) -> Vec<Target> {
        self.iter().map(|x| x.0).collect()
    }
}
impl ToTargets for HashOutTarget {
    fn to_targets(&self) -> Vec<Target> {
        self.elements.to_vec()
    }
}

