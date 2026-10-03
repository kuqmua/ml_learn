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
    let loss_slope_where_positive_calls_for_decreasing_parameter_and_negative_calls_for_increasing_it: f64 = 2.0;
    let moving_average_of_gradient_as_smoothed_update_direction: f64 = 0.9 * 0.0 + 0.1 * loss_slope_where_positive_calls_for_decreasing_parameter_and_negative_calls_for_increasing_it;
    let moving_average_of_squared_gradient_as_update_scale_estimate: f64 = 0.999 * 0.0 + 0.001 * loss_slope_where_positive_calls_for_decreasing_parameter_and_negative_calls_for_increasing_it * loss_slope_where_positive_calls_for_decreasing_parameter_and_negative_calls_for_increasing_it;
    let average_gradient_corrected_for_initial_zero_estimate: f64 =
        moving_average_of_gradient_as_smoothed_update_direction / (1.0 - 0.9);
    let average_squared_gradient_corrected_for_initial_zero_estimate: f64 =
        moving_average_of_squared_gradient_as_update_scale_estimate / (1.0 - 0.999);
    let mut root_mean_squared_gradient_used_to_scale_update: f64 =
        average_squared_gradient_corrected_for_initial_zero_estimate;
    for _ in 0..80 {
        root_mean_squared_gradient_used_to_scale_update =
            (root_mean_squared_gradient_used_to_scale_update
                + average_squared_gradient_corrected_for_initial_zero_estimate
                    / root_mean_squared_gradient_used_to_scale_update)
                / 2.0;
    }
    let old_weight: f64 = 1.0;
    let updated: f64 = old_weight
        - 0.01 * average_gradient_corrected_for_initial_zero_estimate
            / (root_mean_squared_gradient_used_to_scale_update + 0.00000001);

    plot_first_weight_update_using_running_gradient_averages(
        loss_slope_where_positive_calls_for_decreasing_parameter_and_negative_calls_for_increasing_it,
        average_gradient_corrected_for_initial_zero_estimate,
        old_weight,
        updated,
    );
}

// Строим график по результатам урока.
fn plot_first_weight_update_using_running_gradient_averages(
    loss_slope_where_positive_calls_for_decreasing_parameter_and_negative_calls_for_increasing_it: f64,
    average_gradient_corrected_for_initial_zero_estimate: f64,
    old_weight: f64,
    updated: f64,
) {
    lesson_visualization::bar_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Adam: первый шаг",
        "значение",
        &[
            ("градиент", loss_slope_where_positive_calls_for_decreasing_parameter_and_negative_calls_for_increasing_it),
            ("первый момент", average_gradient_corrected_for_initial_zero_estimate),
            ("вес до", old_weight),
            ("вес после", updated),
        ],
    )
    .expect("не удалось сохранить график");
}
