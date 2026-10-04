// Урок 027. Пересчитывать вероятность события после наблюдения с учётом исходной частоты и ложных
// срабатываний.
// На условном примере видно, почему вероятность положительного теста при болезни и вероятность
// болезни после теста различаются.

fn main() {
    let disease_probability_before_observing_test_result: f64 = 0.01;
    let pos_test_probability_given_disease: f64 = 0.90;
    let joint_probability_of_disease_and_pos_test: f64 =
        disease_probability_before_observing_test_result * pos_test_probability_given_disease;
    let neg_test_probability_given_no_disease: f64 = 0.95;
    let joint_probability_of_no_disease_and_pos_test: f64 = (1.0
        - disease_probability_before_observing_test_result)
        * (1.0 - neg_test_probability_given_no_disease);
    let disease_probability_after_pos_test: f64 = joint_probability_of_disease_and_pos_test
        / (joint_probability_of_disease_and_pos_test
            + joint_probability_of_no_disease_and_pos_test);

    // Выполняем вычисления из примера.
    let _ = disease_probability_after_pos_test;

    println!("До наблюдения: {disease_probability_before_observing_test_result:.2}");
    println!(
        "Истинные положительные: {joint_probability_of_disease_and_pos_test:.4}, ложные: {joint_probability_of_no_disease_and_pos_test:.4}"
    );
    println!("Вероятность после положительного теста: {disease_probability_after_pos_test:.4}");
    assert!(disease_probability_after_pos_test > disease_probability_before_observing_test_result);
    assert!(disease_probability_after_pos_test < 0.2);
}

// Чему учит этот урок:
// Учимся пересчитывать вероятность события после наблюдения с учётом исходной частоты и ложных
// срабатываний.
// На условном примере видно, почему вероятность положительного теста при болезни и вероятность
// болезни после теста различаются.
