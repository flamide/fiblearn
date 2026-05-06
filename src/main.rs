use std::{
    marker::PhantomData,
};

use halo2_proofs::{
    circuit::{AssignedCell, Layouter, SimpleFloorPlanner, Value},
    dev::MockProver,
    plonk::{
        Advice, Circuit, Column, ConstraintSystem, Error, Selector
    },
    poly::Rotation,

};

use ff::{PrimeField, Field};

struct TestCircuit<F: Field> {
    _ph: PhantomData<F>,
    left: Value<F>,
    right: Value<F>,

}

#[derive(Clone, Debug)]
struct FibolearnChip<F: Field> {
    _ph: PhantomData<F>,
    q_selector: Selector,
    adv: [Column<Advice>; 3]
}

impl<F: Field> FibolearnChip<F> {

    fn configure(meta: &mut ConstraintSystem<F>) -> Self {
        let adv = [
            meta.advice_column(),
            meta.advice_column(),
            meta.advice_column(),
        ];

        let q_selector = meta.complex_selector();

        // define the arithmetic gate
        meta.create_gate("fibolearn", |meta| {
            let val1 = meta.query_advice(adv[0], Rotation::cur());
            let val2 = meta.query_advice(adv[1], Rotation::cur());
            let val3 = meta.query_advice(adv[2], Rotation::cur());

            let q_selector = meta.query_selector(q_selector);

            vec![q_selector * (val1 + val2 - val3)]
        });

        Self {
            _ph: PhantomData,
            q_selector,
            adv,
        }

    }

    fn assign_values(
        &self,
        layouter: &mut impl Layouter<F>,
        first_value: Value<F>,
        second_value: Value<F>,
    ) -> Result<AssignedCell<F, F>, Error> {

        let third_value = first_value
        .zip(second_value)
        .map(|(a, b)| a + b);

        layouter.assign_region(
            || "fibolearn",
            |mut region| {
                self.q_selector.enable(&mut region, 0)?;

                region.assign_advice(
                    || "first value",
                    self.adv[0],
                    0,
                    || first_value,
                )?;

                region.assign_advice(
                    || "second value",
                    self.adv[1],
                    0,
                    || second_value,
                )?;

                let third_cell = region.assign_advice(
                    || "third value",
                    self.adv[2],
                    0,
                    || third_value,
                )?;

                Ok(third_cell)
            }
        )
    }

}

#[derive(Clone, Debug)]
struct TestConfig<F: Field + Clone> {
    _ph: PhantomData<F>,
    fibolearn_chip: FibolearnChip<F>,
}

impl<F: PrimeField> Circuit<F> for TestCircuit<F> {

    type Config = TestConfig<F>;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        Self {
            _ph: PhantomData,
            left: Value::unknown(),
            right: Value::unknown(),
        }
    }

    fn configure(meta: &mut ConstraintSystem<F>) -> Self::Config {
        let fibolearn_chip = FibolearnChip::configure(meta);

        TestConfig {
            _ph: PhantomData,
            fibolearn_chip,
        }
    }

    fn synthesize(&self, config: Self::Config, mut layouter: impl Layouter<F>) -> Result<(), Error> {
        config.fibolearn_chip.assign_values(&mut layouter, self.left, self.right)?;
        Ok(())
    }

}

fn main() {
    use halo2_proofs::halo2curves::bn256::Fr;

    let circuit = TestCircuit::<Fr> {
        _ph: PhantomData,
        left: Value::known(Fr::from(1)),
        right: Value::known(Fr::from(2))
    };

    let prover = MockProver::run(8, &circuit, vec![]).unwrap();
    prover.verify().unwrap();
}