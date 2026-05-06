# Fiblearn: Learning Halo2 with Fibonacci Circuits

A simple Halo2 project to learn zero-knowledge proof concepts through implementing a basic Fibonacci-like arithmetic circuit.

## Overview

This project demonstrates the fundamentals of Halo2, a zero-knowledge proof library for Rust. It implements a simple circuit that:

1. Takes two input values
2. Computes their sum
3. Constrains that the sum equals a third value
4. Uses a lookup table to validate the result

## Prerequisites

- Rust (latest stable version)
- Basic understanding of zero-knowledge proofs (helpful but not required)

## Installation

Clone the repository and navigate to the project directory:

```bash
git clone <your-repo-url>
cd fiblearn
```

## Building

Build the project using Cargo:

```bash
cargo build
```

## Running

Run the project to execute the circuit proof:

```bash
cargo run
```

This will:
1. Create a test circuit with inputs `1` and `2`
2. Generate a proof that `1 + 2 = 3`
3. Verify the proof using a mock prover

## Circuit Explanation

### Components

- **Advice Columns**: Store witness values (private inputs)
- **Instance Column**: Public inputs/outputs
- **Selector**: Controls when gates are active
- **Table Column**: Lookup table for validation

### Gate Constraint

The circuit enforces: `val1 + val2 = val3`

Where:
- `val1` and `val2` are input values
- `val3` is their sum

### Lookup Table

A lookup table containing values 0-9 is used to validate that the computed sum exists in the table.

## Code Structure

- `src/main.rs`: Main circuit implementation
- `FibolearnChip`: Custom chip containing the circuit logic
- `TestCircuit`: The main circuit struct implementing the `Circuit` trait

## Learning Resources

- [Halo2 Book](https://zcash.github.io/halo2/)
- [Halo2 Documentation](https://docs.rs/halo2_proofs/latest/halo2_proofs/)
- [Zero-Knowledge Proofs Explained](https://zkp.science/)

## Next Steps

To deepen your understanding:

1. Modify the circuit to compute actual Fibonacci sequences
2. Add more complex constraints
3. Implement recursive proofs
4. Explore different proving systems

## License

This project is for educational purposes. Feel free to modify and experiment!</content>
<parameter name="filePath">/home/dell/Blocks/fiblearn/README.md