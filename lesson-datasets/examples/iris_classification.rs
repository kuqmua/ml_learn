use l004_01_calculate_squared_point_distance_by_summing_squared_coordinate_differences::calculate_squared_point_distance_by_summing_squared_coordinate_differences;

fn predict_species_from_nearest_training_record(
    example: &lesson_datasets::IrisRecord,
    records: &[lesson_datasets::IrisRecord],
    training_indices: &[usize],
) -> u8 {
    training_indices
        .iter()
        .min_by(|&&left, &&right| {
            let left_distance =
                calculate_squared_point_distance_by_summing_squared_coordinate_differences(
                    &example.features,
                    &records[left].features,
                )
                .unwrap();
            let right_distance =
                calculate_squared_point_distance_by_summing_squared_coordinate_differences(
                    &example.features,
                    &records[right].features,
                )
                .unwrap();
            left_distance.total_cmp(&right_distance)
        })
        .map(|&index| records[index].species.class_identifier())
        .expect("обучающая часть не пустая")
}

fn classification_accuracy(
    records: &[lesson_datasets::IrisRecord],
    training_indices: &[usize],
    evaluation_indices: &[usize],
) -> f64 {
    let correct = evaluation_indices
        .iter()
        .filter(|&&index| {
            predict_species_from_nearest_training_record(&records[index], records, training_indices)
                == records[index].species.class_identifier()
        })
        .count();
    correct as f64 / evaluation_indices.len() as f64
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let records = lesson_datasets::load_iris_records()?;
    let targets: Vec<u8> = records
        .iter()
        .map(|record| record.species.class_identifier())
        .collect();
    let split = lesson_datasets::split_indices_stratified_by_class(&targets, 42)?;
    let _ = (
        &(records.len()),
        &(split.training_indices.len()),
        &(split.validation_indices.len()),
        &(split.test_indices.len()),
    );
    let _ = (
        &(classification_accuracy(&records, &split.training_indices, &split.validation_indices)),
        &(classification_accuracy(&records, &split.training_indices, &split.test_indices)),
    );
    Ok(())
}
