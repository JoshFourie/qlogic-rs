use crate::qubit::*;
use crate::ops::*;

#[test]
fn test_pauli_x_gate()
{
    let pauli_x = Operator::from(vec![0,1,1,0]);
    let qubit = Qubit::from(vec![0,1]);
    let exp = Qubit::from(vec![1,0]);
    assert_eq!(pauli_x*qubit,exp);
}