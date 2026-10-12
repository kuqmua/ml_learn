use mnist_data::numeric_types::CorrectPredictionFractionFiniteFrom0To1InclusiveF32;

#[test]
fn accepts_boundaries_and_fraction() {
    for value in [0.0, 0.92, 1.0] {
        assert_eq!(
            CorrectPredictionFractionFiniteFrom0To1InclusiveF32::try_from(value)
                .unwrap()
                .value(),
            value
        );
    }
}

#[test]
fn rejects_out_of_range_and_non_finite_values() {
    for value in [
        -f32::EPSILON,
        1.0 + f32::EPSILON,
        f32::NAN,
        f32::INFINITY,
        f32::NEG_INFINITY,
    ] {
        assert!(CorrectPredictionFractionFiniteFrom0To1InclusiveF32::try_from(value).is_err());
    }
}
