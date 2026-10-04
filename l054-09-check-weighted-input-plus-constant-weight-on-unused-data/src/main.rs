// Урок 09.5. Проверка прогноза по прямой на данных, не использованных для обучения.
// Зачем здесь эта тема: Уменьшение ошибки на train не доказывает обобщение; нужны данные, не
//   участвовавшие в подгонке.
// Почему код устроен так: Фиксируем модель и считаем ту же метрику на отложенных строках.
// Представь: Модель может идеально помнить train и ошибаться на новых строках; отложенная часть
//   показывает это.
//
// Что изучаем: Качество на отложенных данных.
// Зачем это нужно: Train служит для выбора параметров; качество модели оцениваем на новых примерах, не
// участвовавших в обучении.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
use l050_09_calc_mean_squared_error_by_summing_squared_errors_and_dividing_by_count::calc_mean_squared_error_by_summing_squared_errors_and_dividing_by_count;

fn main() {
    let training: [(f64, f64); 2] = [(1.0, 3.0), (2.0, 5.0)];
    assert!(
        training.len() >= 2,
        "для прямой нужны хотя бы две обучающие точки"
    );
    assert_ne!(
        training[0].0, training[1].0,
        "обучающие точки должны иметь разные значения x"
    );
    let test: [(f64, f64); 2] = [(3.0, 7.0), (4.0, 9.0)];
    assert!(
        !test.is_empty(),
        "для MSE нужен хотя бы один тестовый пример"
    );
    let weight: f64 = (training[1].1 - training[0].1) / (training[1].0 - training[0].0);
    let constant_input_weight: f64 = training[0].1 - weight * training[0].0;

    let _: f64 = calc_mean_squared_error_by_summing_squared_errors_and_dividing_by_count(
        &test.map(|(_, target)| target),
        &test.map(|(feature, _)| weight * feature + constant_input_weight),
    )
    .unwrap();

    // Выполняем вычисления из примера.
    let _ = (training, test, weight, constant_input_weight);
}

// Чему учит этот урок:
// Учимся определять прямую по двум обучающим точкам и оценивать её на отдельных примерах.
// Разделяем подбор параметров и проверку прогноза на данных, не использованных для подбора.
