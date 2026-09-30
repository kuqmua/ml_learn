// Урок 45.5. Практика: наблюдение за изменениями данных, ошибками, временем ответа и предупреждениями.
// Связь с принятой терминологией: Мониторинг дрейфа, качества, задержки и предупреждений.
// Зачем здесь эта тема: Наблюдение модели после выпуска требует вместе видеть входы, метки,
//   задержку и пороги.
// Почему код устроен так: Собираем показатели в одном примере, чтобы предупреждение можно было
//   объяснить исходными числами.
// Представь: Один отчёт показывает дрейф признаков, новую ошибку и задержку, чтобы решение об
//   обновлении имело основания.
//
// Что повторяем вместе: распределения признаков, качество после релиза, латентность, алерты.
// Зачем это нужно: Мониторинг сравнивает новые данные с эталоном, чтобы заметить изменение распределения
//   входного признака.
// Что показывает программа: Задаём эталонное распределение признака. Готовим контрольный набор без сдвига и
//   набор с сильным сдвигом. Сравниваем PSI при одинаковом и изменившемся распределении.
// Что проверить при изменении примера: Проверь сценарий без дрейфа и искусственный сдвиг; отчёт указывает
//   размер выборки.
// Дополнительная практика: Сравни эталонные и новые данные, рассчитай простую метрику дрейфа и статистику
//   ошибок.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    lesson_trace::enable();
    lesson_trace::trace_note!("Шаг: Задаём эталонное распределение признака.");
    let reference: [f64; 6] = [-1., -0.5, 0.1, 0.2, 1.2, 1.5];
    lesson_trace::trace_step!(reference);
    lesson_trace::trace_note!(
        "Шаг: Готовим контрольный набор без сдвига и набор с сильным сдвигом."
    );
    let stable: [f64; 6] = reference;
    lesson_trace::trace_step!(stable);
    lesson_trace::trace_note!("Создаём набор значений `shifted` для следующего шага примера.");
    let shifted: [f64; 6] = [1.1, 1.2, 1.3, 1.4, 1.5, 1.6];
    lesson_trace::trace_step!(shifted);

    lesson_trace::trace_note!("Учебные реализации математических операций для этого урока.");

    /// Выбираем большее из двух чисел для формул softmax, log-loss и Q-learning.
    /// Аналог `first.max(second)` для обычных чисел; при NaN результат может отличаться.
    fn choose_larger_number(first: f64, second: f64) -> f64 {
        lesson_trace::trace_note!("Проверяем условие и выбираем соответствующую ветку алгоритма.");
        if first > second { first } else { second }
    }

    lesson_trace::trace_note!(
        "Объявляем повторно используемое вычисление `calculate_three_group_shares`; параметры ниже задают его входы."
    );
    lesson_trace::trace_note!(
        "Долю значений, попавших в одну группу, называют fraction этой группы."
    );
    fn calculate_shares_of_feature_values_in_three_bins(data: &[f64]) -> [f64; 3] {
        lesson_trace::trace_note!(
            "Создаём набор значений `bin_counts` для следующего шага примера."
        );
        let mut bin_counts: [f64; 3] = [0.; 3];
        lesson_trace::trace_step!(bin_counts);
        lesson_trace::trace_note!(
            "Одни и те же границы интервалов используются для обоих наборов данных."
        );
        for &feature_value in data {
            lesson_trace::trace_step!(feature_value);
            lesson_trace::trace_note!(
                "Сохраняем рассчитанное значение `histogram_bin` для следующих операций."
            );
            let histogram_bin: usize = if feature_value < 0. {
                lesson_trace::trace_note!(
                    "Используем фиксированное значение для этого варианта примера."
                );
                lesson_trace::trace_note!(
                    "Используем подготовленное значение в следующем шаге примера."
                );
                0
            } else if feature_value < 1. {
                lesson_trace::trace_note!(
                    "Используем фиксированное значение для этого варианта примера."
                );
                1
            } else {
                lesson_trace::trace_note!(
                    "Обрабатываем случай, когда предыдущее условие не выполнено."
                );
                lesson_trace::trace_note!(
                    "Используем фиксированное значение для этого варианта примера."
                );
                2
            };
            lesson_trace::trace_step!(histogram_bin);
            lesson_trace::trace_note!(
                "Прибавляем очередной вклад к ранее накопленному результату."
            );
            bin_counts[histogram_bin] += 1.;
            lesson_trace::trace_step!(bin_counts);
        }
        lesson_trace::trace_note!(
            "Переходим от числа объектов к долям, чтобы сравнивать разные размеры выборок."
        );
        for group_share in &mut bin_counts {
            lesson_trace::trace_step!(group_share);
            lesson_trace::trace_note!("Масштабируем текущую величину делением.");
            *group_share /= data.len() as f64;
            lesson_trace::trace_step!(group_share);
        }
        lesson_trace::trace_note!(
            "Используем ранее рассчитанное значение `bin_counts` в текущем выражении."
        );
        bin_counts
    }
    lesson_trace::trace_note!(
        "Сравниваем доли объектов в одинаковых интервалах эталона и новых данных."
    );
    /// Индекс стабильности популяции (PSI): суммируем (current−reference)·ln(current/reference) по долям трёх интервалов, ограничивая доли снизу.
    fn calculate_distribution_shift_score_as_sum_of_bin_share_differences_times_log_share_ratios(
        reference: &[f64],
        current: &[f64],
    ) -> f64 {
        lesson_trace::trace_note!(
            "Сохраняем рассчитанное значение `reference_group_shares` для следующих операций."
        );
        let reference_group_shares: [f64; 3] =
            calculate_shares_of_feature_values_in_three_bins(reference);
        lesson_trace::trace_step!(reference_group_shares);
        lesson_trace::trace_note!(
            "Сохраняем рассчитанное значение `current_group_shares` для следующих операций."
        );
        let current_group_shares: [f64; 3] =
            calculate_shares_of_feature_values_in_three_bins(current);
        lesson_trace::trace_step!(current_group_shares);
        lesson_trace::trace_note!(
            "Инициализируем изменяемый накопитель `stability_index` начальным состоянием."
        );
        let mut stability_index: f64 = 0.0;
        lesson_trace::trace_step!(stability_index);
        lesson_trace::trace_note!(
            "Повторяем следующий блок для каждого элемента указанной последовательности."
        );
        for bin_index in 0..reference_group_shares.len() {
            lesson_trace::trace_step!(bin_index);
            lesson_trace::trace_note!(
                "Нижняя граница 10⁻⁶ предотвращает деление на ноль и log(0) в пустом интервале."
            );
            lesson_trace::trace_note!("Она намного меньше ненулевых долей в этом учебном наборе.");
            let reference_group_share: f64 =
                choose_larger_number(reference_group_shares[bin_index], 1e-6);
            lesson_trace::trace_step!(reference_group_share);
            lesson_trace::trace_note!(
                "Комбинируем исходные величины и сохраняем результат в `current_group_share`."
            );
            let current_group_share: f64 =
                choose_larger_number(current_group_shares[bin_index], 1e-6);
            lesson_trace::trace_step!(current_group_share);
            lesson_trace::trace_note!(
                "Прибавляем очередной вклад к ранее накопленному результату."
            );
            lesson_trace::trace_note!("Добавляем этот член в составное арифметическое выражение.");
            stability_index += (current_group_share - reference_group_share)
                * (|| -> f64 {
                    lesson_trace::trace_note!(
                        "Обновляем значение результатом текущего вычисления."
                    );
                    lesson_trace::trace_note!(
                        "ln(x) через ряд 2 * (t + t³/3 + t⁵/5 + ...), t=(x-1)/(x+1)."
                    );
                    lesson_trace::trace_note!("Сохраняем результат этого шага в `value`.");
                    let value: f64 = current_group_share / reference_group_share;
                    lesson_trace::trace_step!(value);
                    lesson_trace::trace_note!(
                        "Проверяем обязательное условие до дальнейшего вычисления."
                    );
                    lesson_trace::trace_note!(
                        "Передаём очередное значение в составе результата или вызова."
                    );
                    lesson_trace::trace_note!(
                        "Подставляем результаты в этот шаблон вывода или текстового значения."
                    );
                    assert!(
                        value > 0.0,
                        "логарифм определён только для положительных чисел"
                    );
                    lesson_trace::trace_note!(
                        "Проверяем условие и выбираем соответствующую ветку алгоритма."
                    );
                    if value == f64::INFINITY {
                        lesson_trace::trace_note!(
                            "Завершаем текущий расчёт и возвращаем найденное значение."
                        );
                        return f64::INFINITY;
                    }
                    lesson_trace::trace_note!(
                        "Создаём изменяемое значение `scaled` для следующих операций."
                    );
                    let mut scaled: f64 = value;
                    lesson_trace::trace_step!(scaled);
                    lesson_trace::trace_note!(
                        "Инициализируем изменяемый накопитель `power_of_two` начальным состоянием."
                    );
                    let mut power_of_two: i32 = 0i32;
                    lesson_trace::trace_step!(power_of_two);
                    lesson_trace::trace_note!(
                        "Повторяем вычисление, пока выполняется указанное условие."
                    );
                    while scaled >= 2.0 {
                        lesson_trace::trace_note!("Масштабируем текущую величину делением.");
                        scaled /= 2.0;
                        lesson_trace::trace_step!(scaled);
                        lesson_trace::trace_note!(
                            "Прибавляем очередной вклад к ранее накопленному результату."
                        );
                        power_of_two += 1;
                        lesson_trace::trace_step!(power_of_two);
                    }
                    lesson_trace::trace_note!(
                        "Повторяем вычисление, пока выполняется указанное условие."
                    );
                    while scaled < 1.0 {
                        lesson_trace::trace_note!(
                            "Умножаем накопленное значение на очередной множитель."
                        );
                        scaled *= 2.0;
                        lesson_trace::trace_step!(scaled);
                        lesson_trace::trace_note!(
                            "Вычитаем очередной вклад из текущего значения параметра."
                        );
                        power_of_two -= 1;
                        lesson_trace::trace_step!(power_of_two);
                    }
                    lesson_trace::trace_note!(
                        "Этот ряд — учебное раскрытие `value.ln()`; он может работать медленнее и отличаться по точности."
                    );
                    lesson_trace::trace_note!(
                        "Объявляем повторно используемое вычисление `approximate_natural_log_as_twice_sum_of_odd_ratio_powers_over_odd_numbers`; параметры ниже задают его входы."
                    );
                    /// Ряд для ln(x): 2·(t + t³/3 + t⁵/5 + …), где t = (x−1)/(x+1).
                    fn approximate_natural_log_as_twice_sum_of_odd_ratio_powers_over_odd_numbers(
                        value: f64,
                    ) -> f64 {
                        lesson_trace::trace_note!(
                            "Нормируем или усредняем величину делением и сохраняем её в `ratio`."
                        );
                        let ratio: f64 = (value - 1.0) / (value + 1.0);
                        lesson_trace::trace_step!(ratio);
                        lesson_trace::trace_note!(
                            "Умножаем значения и сохраняем результат в `ratio_squared`."
                        );
                        let ratio_squared: f64 = ratio * ratio;
                        lesson_trace::trace_step!(ratio_squared);
                        lesson_trace::trace_note!(
                            "Создаём изменяемое значение `term` для следующих операций."
                        );
                        let mut term: f64 = ratio;
                        lesson_trace::trace_step!(term);
                        lesson_trace::trace_note!(
                            "Инициализируем изменяемый накопитель `result` начальным состоянием."
                        );
                        let mut result: f64 = 0.0;
                        lesson_trace::trace_step!(result);
                        lesson_trace::trace_note!(
                            "Используем 40 первых членов ряда ln(value) = 2·Σ ratio^(2k+1)/(2k+1)."
                        );
                        lesson_trace::trace_note!(
                            "Это конечное приближение: для положительного value выполняется |ratio| < 1."
                        );
                        for term_index in 0..40 {
                            lesson_trace::trace_step!(term_index);
                            lesson_trace::trace_note!(
                                "Прибавляем очередной вклад к ранее накопленному результату."
                            );
                            result += term / (2 * term_index + 1) as f64;
                            lesson_trace::trace_step!(result);
                            lesson_trace::trace_note!(
                                "Умножаем накопленное значение на очередной множитель."
                            );
                            term *= ratio_squared;
                            lesson_trace::trace_step!(term);
                        }
                        lesson_trace::trace_note!(
                            "Умножаем величины согласно используемой формуле."
                        );
                        2.0 * result
                    }
                    lesson_trace::trace_note!(
                        "Сохраняем рассчитанное значение `logarithm_of_two` для следующих операций."
                    );
                    let logarithm_of_two: f64 =
                        approximate_natural_log_as_twice_sum_of_odd_ratio_powers_over_odd_numbers(
                            2.0,
                        );
                    lesson_trace::trace_step!(logarithm_of_two);
                    lesson_trace::trace_note!("Умножаем величины согласно используемой формуле.");
                    approximate_natural_log_as_twice_sum_of_odd_ratio_powers_over_odd_numbers(
                        scaled,
                    ) + power_of_two as f64 * logarithm_of_two
                })();
            lesson_trace::trace_step!(stability_index);
        }
        lesson_trace::trace_note!(
            "Используем ранее рассчитанное значение `stability_index` в текущем выражении."
        );
        stability_index
    }

    lesson_trace::trace_note!("Шаг: Сравниваем PSI при одинаковом и изменившемся распределении.");
    lesson_trace::trace_note!(
        "Задаём шаблон строки: плейсхолдеры ниже заменятся рассчитанными значениями."
    );
    lesson_trace::trace_note!("Передаём очередное значение в составе результата или вызова.");
    lesson_trace::trace_note!("Передаём очередное значение в составе результата или вызова.");
    lesson_trace::trace_note!("Вызываем нужное вычисление с подготовленными аргументами.");
    lesson_trace::trace_note!("Вызываем нужное вычисление с подготовленными аргументами.");
    println!(
        "n_ref={}, n_current={}, PSI stable={:.3}, shifted={:.3}",
        reference.len(),
        shifted.len(),
        calculate_distribution_shift_score_as_sum_of_bin_share_differences_times_log_share_ratios(
            &reference, &stable
        ),
        calculate_distribution_shift_score_as_sum_of_bin_share_differences_times_log_share_ratios(
            &reference, &shifted
        )
    );

    lesson_trace::trace_note!("Построение графика вынесено из основного кода урока.");
    lesson_trace::disable();
    plot_distribution_shift_score_as_sum_of_interval_share_changes_times_log_share_ratios(
        reference, stable, shifted,
    );

    lesson_trace::trace_note!("Строим график по результатам урока.");
    fn plot_distribution_shift_score_as_sum_of_interval_share_changes_times_log_share_ratios(
        reference: [f64; 6],
        stable: [f64; 6],
        shifted: [f64; 6],
    ) {
        lesson_trace::trace_note!("Наглядное сравнение результатов сводной практики.");
        lesson_trace::trace_note!("Передаём путь к каталогу текущего урока.");
        lesson_trace::trace_note!("Указываем имя SVG-файла.");
        lesson_trace::trace_note!("Указываем заголовок диаграммы.");
        lesson_trace::trace_note!("Указываем подпись вертикальной оси.");
        lesson_trace::trace_note!("Передаём ряды или значения для отрисовки графика.");
        lesson_trace::trace_note!("Передаём подпись или текстовое значение для следующего шага.");
        lesson_trace::trace_note!("Используем подготовленное значение в следующем шаге примера.");
        lesson_trace::trace_note!("Передаём подпись или текстовое значение для следующего шага.");
        lesson_trace::trace_note!("Используем подготовленное значение в следующем шаге примера.");
        lesson_trace::trace_note!(
            "Прерываем пример с понятной ошибкой, если SVG не удалось записать."
        );
        let chart: std::path::PathBuf = lesson_visualization::bar_chart(

            env!("CARGO_MANIFEST_DIR"),

            "lesson-chart",

            "Сдвиг признака",

            "PSI",

            &[
                (

                    "стабильно",

                    calculate_distribution_shift_score_as_sum_of_bin_share_differences_times_log_share_ratios(&reference, &stable),
                ),
                (

                    "сдвиг",

                    calculate_distribution_shift_score_as_sum_of_bin_share_differences_times_log_share_ratios(&reference, &shifted),
                ),
            ],
        )

        .expect("не удалось сохранить график");
        lesson_trace::trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
        println!("график: {}", chart.display());
    }
}
