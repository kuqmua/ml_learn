// Урок 062. Получать четыре счётчика результатов классификации из оценок модели и выбранного порога.
// Сравнение порогов помогает увидеть изменение числа ложных тревог и пропусков.

use l062_11_count_binary_classification_outcomes_from_targets_and_scores_at_threshold::count_binary_classification_outcomes_from_targets_and_scores_at_threshold;

fn main() {
    let truth = [true, false, true, false];
    let scores = [0.9, 0.6, 0.5, 0.1];
    for threshold in [0.5, 0.7] {
        let _ = count_binary_classification_outcomes_from_targets_and_scores_at_threshold(
            &truth, &scores, threshold,
        )
        .unwrap();
    }

    for threshold in [0.5, 0.7] {
        let counts = count_binary_classification_outcomes_from_targets_and_scores_at_threshold(
            &truth, &scores, threshold,
        )
        .unwrap();
        println!("Порог {threshold}: {counts:?}");
        if threshold == 0.5 {
            assert_eq!(counts.false_poss_as_false_alarms_on_neg_cases, 1);
        } else {
            assert_eq!(counts.false_poss_as_false_alarms_on_neg_cases, 0);
            assert_eq!(counts.false_negs_as_missed_pos_cases, 1);
        }
    }
}

// Чему учит этот урок:
// Учимся получать четыре счётчика результатов классификации из оценок модели и выбранного порога.
// Сравнение порогов помогает увидеть изменение числа ложных тревог и пропусков.
