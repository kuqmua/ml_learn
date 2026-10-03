// Урок 05.3. Пересчёт вероятности события с учётом частоты полученного наблюдения.
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
}
