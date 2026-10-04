// Урок 23.3. Адаптивный шаг обучения: масштабирование по накопленным средним скоростей изменения ошибки и их квадратов.
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
    let loss_slope: f64 = 2.0;
    let moving_average_of_gradient_as_smoothed_update_direction: f64 = 0.9 * 0.0 + 0.1 * loss_slope;
    let moving_average_of_squared_gradient_as_update_scale_estimate: f64 =
        0.999 * 0.0 + 0.001 * loss_slope * loss_slope;
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

    // Выполняем вычисления из примера.
    let _ = (
        loss_slope,
        average_gradient_corrected_for_initial_zero_estimate,
        old_weight,
        updated,
    );
}

// Чему учит этот урок:
// Учимся вычислять один адаптивный шаг по средним производных и их квадратов.
// Исправляем начальное смещение средних и масштабируем обновление — основные части шага Adam.
