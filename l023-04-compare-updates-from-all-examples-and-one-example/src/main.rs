// Урок 04.4. Сравнение обновлений параметра по всем примерам и по одному примеру.
// Зачем здесь эта тема: Градиент можно оценить по всем примерам или по одному; это меняет шум и
//   стоимость шага.
// Почему код устроен так: Сравниваем оба режима на одних данных, фиксируя, какой набор дал
//   очередное обновление.
// Представь: Среднее по всем строкам даёт ровный шаг, а отдельная случайная строка может временно
//   толкнуть параметр в другую сторону.
//
// Что изучаем: Batch и stochastic обновления.
// Зачем это нужно: Batch использует средний градиент всех примеров, stochastic — градиент одного. На одном
// шаге их направления могут отличаться.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.

fn main() {
    let data: [(f64, f64); 2] = [(1.0, 2.0), (2.0, 4.0)];
    assert!(!data.is_empty(), "для градиента нужны обучающие примеры");
    let weight: f64 = 0.0;
    let mut summed_rates_of_change: f64 = 0.0;
    for (feature, target) in data {
        summed_rates_of_change += 2.0 * (weight * feature - target) * feature;
    }
    let batch_loss_rate_of_change: f64 = summed_rates_of_change / data.len() as f64;
    let (first_feature, first_target): (f64, f64) = data[0];
    let single_example_loss_rate_of_change: f64 =
        2.0 * (weight * first_feature - first_target) * first_feature;

    plot_average_rate_of_change_and_single_example_rates(
        batch_loss_rate_of_change,
        single_example_loss_rate_of_change,
    );
}

// Строим график по результатам урока.
fn plot_average_rate_of_change_and_single_example_rates(
    batch_loss_rate_of_change: f64,
    single_example_loss_rate_of_change: f64,
) {
    lesson_visualization::bar_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Batch и stochastic градиенты",
        "градиент",
        &[
            ("batch", batch_loss_rate_of_change),
            ("stochastic", single_example_loss_rate_of_change),
        ],
    )
    .expect("не удалось сохранить график");
}
