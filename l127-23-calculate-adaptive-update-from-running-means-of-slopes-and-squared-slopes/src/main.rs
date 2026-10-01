// Урок 23.3. Адаптивный шаг обучения: масштабирование по накопленным средним скоростей изменения ошибки и их квадратов.
// Связь с принятой терминологией: Обновление Adam по оценкам моментов градиента.
// Зачем здесь эта тема: Один масштаб шага для всех параметров не всегда подходит; Adam подстраивает
//   шаг по моментам градиента.
// Почему код устроен так: Отдельно считаем скользящие средние градиента и его квадрата с поправкой
//   на начальные шаги.
// Представь: Adam помнит и средний градиент, и средний квадрат градиента, чтобы менять масштаб
//   шага.
//
// Что изучаем: Обновление Adam.
// Зачем это нужно: Adam отслеживает сглаженные первый и второй моменты градиента, чтобы подстраивать
// размер шага по параметру.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.

fn main() {
    let rate_of_change: f64 = 2.0;
    let first_moment: f64 = 0.9 * 0.0 + 0.1 * rate_of_change;
    let second_moment: f64 = 0.999 * 0.0 + 0.001 * rate_of_change * rate_of_change;
    let corrected_first: f64 = first_moment / (1.0 - 0.9);
    let corrected_second: f64 = second_moment / (1.0 - 0.999);
    let mut root: f64 = corrected_second;
    for _ in 0..80 {
        root = (root + corrected_second / root) / 2.0;
    }
    let old_weight: f64 = 1.0;
    let updated: f64 = old_weight - 0.01 * corrected_first / (root + 0.00000001);

    plot_first_weight_update_using_running_gradient_averages(
        rate_of_change,
        corrected_first,
        old_weight,
        updated,
    );
}

// Строим график по результатам урока.
fn plot_first_weight_update_using_running_gradient_averages(
    rate_of_change: f64,
    corrected_first: f64,
    old_weight: f64,
    updated: f64,
) {
    lesson_visualization::bar_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Adam: первый шаг",
        "значение",
        &[
            ("градиент", rate_of_change),
            ("первый момент", corrected_first),
            ("вес до", old_weight),
            ("вес после", updated),
        ],
    )
    .expect("не удалось сохранить график");
}
