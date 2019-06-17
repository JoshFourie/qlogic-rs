# qlogic-rs
Quantum Logic for Personal Interest.

The purpose is to implement a simple simulation of a Quantum Key Distribution protocol, as well as some extras.

The project uses a nested-crates structure:
- `./algebra` contains the linear algebra for the simulation.

`Docs` are a work in progress on the project and module levels. Currently the auto-generated documentation for the linear algebra crate can either be built by the Rust Cargo compiler or navigating to `./target/doc/algebra/index.html` after cloning the repo. There are no online copies available.

The majority of the linear algebra code is based on psuedo-code from the Princeton Handbook of Linear Algebra and scattered university lecture slides freely available online. 