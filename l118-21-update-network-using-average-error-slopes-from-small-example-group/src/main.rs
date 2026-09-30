// Урок 21.5. Обновление сети по средним скоростям изменения ошибки для небольшой группы примеров.
// Связь с принятой терминологией: Обновление параметров сети по среднему градиенту мини пакета.
// Зачем здесь эта тема: Градиент по одному примеру шумен, а по всему набору дорог; мини пакет даёт
//   промежуточный вариант.
// Почему код устроен так: Усредняем градиенты нескольких строк и делаем одно обновление параметров.
// Представь: Для градиентов 2 и 4 средний градиент мини пакета равен 3; шаг делаем по нему один
//   раз.
//
// Что изучаем: Mini-batch обучение.
// Зачем это нужно: Шаг обновления может использовать средний градиент небольшой группы примеров вместо
// одного объекта или всего набора.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.

fn main() {
    let rates_of_change: [f64; 2] = [2.0, 4.0];
    assert!(
        !rates_of_change.is_empty(),
        "мини-пакет градиентов не должен быть пустым"
    );
    let small_batch_loss_rate_of_change: f64 =
        rates_of_change.iter().sum::<f64>() / rates_of_change.len() as f64;
    let old_weight: f64 = 1.0;
    let learning_rate: f64 = 0.1;
    let new_weight: f64 = old_weight - learning_rate * small_batch_loss_rate_of_change;

    plot_weight_before_and_after_averaged_example_update(old_weight, new_weight);
}

// Строим график по результатам урока.
fn plot_weight_before_and_after_averaged_example_update(old_weight: f64, new_weight: f64) {
    let _chart: std::path::PathBuf = lesson_visualization::bar_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Mini-batch обновление",
        "вес",
        &[("до", old_weight), ("после", new_weight)],
    )
    .expect("не удалось сохранить график");
}
