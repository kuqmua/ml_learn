// Урок 05.3. Пересчёт вероятности события с учётом частоты полученного наблюдения.
// Связь с принятой терминологией: Пересчёт вероятности события после наблюдения по формуле Байеса.
// Зачем здесь эта тема: Диагностический сигнал меняет исходную вероятность события; формула Байеса
//   соединяет prior и качество сигнала.
// Почему код устроен так: Разделяем истинные и ложные сигналы, чтобы увидеть влияние редкости
//   события.
// Представь: Даже хороший тест даёт много ложных тревог, когда проверяемое событие очень редкое.
//
// Что изучаем: Формула Байеса.
// Зачем это нужно: Формула пересчитывает вероятность причины после наблюдения результата. Редкая болезнь
// остаётся редкой даже после несовершенного теста.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.

fn main() {
    let disease_probability_before_observing_test_result: f64 = 0.01;
    let pos_test_probability_given_disease_where_1_means_no_missed_cases: f64 = 0.90;
    let joint_probability_of_disease_and_pos_test: f64 =
        disease_probability_before_observing_test_result
            * pos_test_probability_given_disease_where_1_means_no_missed_cases;
    let neg_test_probability_given_no_disease_where_1_means_no_false_alarms: f64 = 0.95;
    let joint_probability_of_no_disease_and_pos_test: f64 = (1.0
        - disease_probability_before_observing_test_result)
        * (1.0 - neg_test_probability_given_no_disease_where_1_means_no_false_alarms);
    let disease_probability_after_pos_test: f64 = joint_probability_of_disease_and_pos_test
        / (joint_probability_of_disease_and_pos_test
            + joint_probability_of_no_disease_and_pos_test);

    plot_disease_probability_before_and_after_pos_test(disease_probability_after_pos_test);
}

// Строим график по результатам урока.
fn plot_disease_probability_before_and_after_pos_test(disease_probability_after_pos_test: f64) {
    lesson_visualization::bar_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Байес: до и после теста",
        "вероятность",
        &[
            ("до теста", 0.01),
            ("после теста", disease_probability_after_pos_test),
        ],
    )
    .expect("не удалось сохранить график");
}
