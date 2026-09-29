// Урок 32.5. Практика: сбор контекста, прибавление входа и преобразование координат.
// Связь с принятой терминологией: Блок Transformer с вниманием, остаточными связями и полносвязным слоем.
// Зачем здесь эта тема: Transformer объединяет обмен между токенами и преобразование каждого токена
//   при стабильном масштабе.
// Почему код устроен так: Собираем внимание, residual, LayerNorm и feed-forward по порядку и
//   проверяем форму после каждого шага.
// Представь: Токен сначала получает контекст через attention, затем сохраняет вход через residual и
//   преобразуется дальше.
//
// Что повторяем вместе: self-attention, residual, layer norm, feed-forward.
// Зачем это нужно: Блок Transformer сочетает внимание, остаточные связи, нормализацию и преобразование
//   каждого токена.
// Что показывает программа: Создаём вход из двух токенов с двумерными признаками. Пропускаем его через
//   attention, residual, нормализацию и feed-forward.
// Что проверить при изменении примера: Проверь сохранение формы, отсутствие NaN и детерминированный forward
//   при фиксированных весах.
// Дополнительная практика: Собери один блок на малых тензорах и опиши порядок операций.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    lesson_trace::enable();
    // Шаг: Создаём вход из двух токенов с двумерными признаками.
    let input_values: [[f64; 2]; 2] = [[1., 0.], [0., 1.]];
    lesson_trace::trace_step!(input_values);

    // Учебные реализации математических операций для этого урока.

    /// Возводим число в квадрат обычным умножением.
    /// Вместо этой учебной обёртки можно написать `value * value` или `value.powi(2)`.
    /// Само умножение не обязательно медленнее библиотечного метода.
    fn calculate_square_by_multiplying_number_by_itself(value: f64) -> f64 {
        // Умножаем величины согласно используемой формуле.
        value * value
    }

    /// Корень через итерацию Ньютона: x_(n+1) = (x_n + value / x_n) / 2.
    /// Учебный аналог `f64::sqrt`; показывает алгоритм и может работать медленнее.
    /// Здесь отрицательный вход вызывает panic, а `sqrt` возвращает NaN.
    /// Метод Ньютона для корня: повторяем estimate = (estimate + value / estimate) / 2.
    fn approximate_square_root_by_repeated_averaging(value: f64) -> f64 {
        // Проверяем обязательное условие до дальнейшего вычисления.
        assert!(value >= 0.0, "корень из отрицательного числа");
        // Проверяем условие и выбираем соответствующую ветку алгоритма.
        if value == 0.0 {
            // Завершаем текущий расчёт и возвращаем найденное значение.
            return 0.0;
        }
        // Создаём изменяемое значение `estimate` для следующих операций.
        let mut estimate: f64 = if value > 1.0 { value } else { 1.0 };
        lesson_trace::trace_step!(estimate);
        // 80 шагов Ньютона дают оценку √value с запасом для f64.
        for _ in 0..80 {
            // Среднее estimate и value/estimate приближает квадратный корень.
            estimate = (estimate + value / estimate) / 2.0;
            lesson_trace::trace_step!(estimate);
        }
        // Используем ранее рассчитанное значение `estimate` в текущем выражении.
        estimate
    }

    /// e^x по ряду Тейлора. Деление аргумента пополам ускоряет сходимость.
    /// Учебный аналог `f64::exp`; показывает вычисление ряда и может работать медленнее.
    /// При замене возможны небольшие отличия из-за точности и обработки крайних значений.
    /// Экспонента eˣ: ряд Тейлора 1 + x + x²/2! + x³/3! + … с уменьшением аргумента и восстановлением масштаба.
    fn approximate_e_to_power_by_summing_power_over_factorial_terms(value: f64) -> f64 {
        // Проверяем условие и выбираем соответствующую ветку алгоритма.
        if value == f64::NEG_INFINITY || value < -745.0 {
            // Завершаем текущий расчёт и возвращаем найденное значение.
            return 0.0;
        }
        // Проверяем условие и выбираем соответствующую ветку алгоритма.
        if value == f64::INFINITY || value > 709.0 {
            // Завершаем текущий расчёт и возвращаем найденное значение.
            return f64::INFINITY;
        }
        // Проверяем условие и выбираем соответствующую ветку алгоритма.
        if value < 0.0 {
            // Завершаем текущий расчёт и возвращаем найденное значение.
            return 1.0 / approximate_e_to_power_by_summing_power_over_factorial_terms(-value);
        }
        // Создаём изменяемое значение `reduced` для следующих операций.
        let mut reduced: f64 = value;
        lesson_trace::trace_step!(reduced);
        // Инициализируем изменяемый накопитель `halving_count` начальным состоянием.
        let mut halving_count: i32 = 0;
        lesson_trace::trace_step!(halving_count);
        // Уменьшаем аргумент до ≤0.5: на таком интервале ряд Тейлора для exp сходится быстро.
        while reduced > 0.5 {
            // Масштабируем текущую величину делением.
            reduced /= 2.0;
            lesson_trace::trace_step!(reduced);
            // Прибавляем очередной вклад к ранее накопленному результату.
            halving_count += 1;
            lesson_trace::trace_step!(halving_count);
        }
        // Создаём изменяемое значение `term` для следующих операций.
        let mut term: f64 = 1.0;
        lesson_trace::trace_step!(term);
        // Создаём изменяемое значение `result` для следующих операций.
        let mut result: f64 = 1.0;
        lesson_trace::trace_step!(result);
        // Берём 30 членов ряда exp(y)=Σ y^k/k!; это предел приближения для учебных входов.
        for term_index in 1..=30 {
            lesson_trace::trace_step!(term_index);
            // Умножаем накопленное значение на очередной множитель.
            term *= reduced / term_index as f64;
            lesson_trace::trace_step!(term);
            // Прибавляем очередной вклад к ранее накопленному результату.
            result += term;
            lesson_trace::trace_step!(result);
        }
        // Восстанавливаем exp(value): каждое возведение в квадрат отменяет одно деление аргумента на 2.
        for _ in 0..halving_count {
            // Умножаем накопленное значение на очередной множитель.
            result *= result;
            lesson_trace::trace_step!(result);
        }
        // Используем ранее рассчитанное значение `result` в текущем выражении.
        result
    }

    /// Выбираем большее из двух чисел для формул softmax, log-loss и Q-learning.
    /// Аналог `first.max(second)` для обычных чисел; при NaN результат может отличаться.
    fn choose_larger_number(first: f64, second: f64) -> f64 {
        // Проверяем условие и выбираем соответствующую ветку алгоритма.
        if first > second { first } else { second }
    }

    // Объявляем повторно используемое вычисление `normalize_two_feature_vector`; параметры ниже задают его входы.
    /// Нормализация слоя (LayerNorm): из координат вычитаем среднее и делим на sqrt(среднее квадратов отклонений + epsilon).
    fn normalize_coordinates_by_subtracting_mean_and_dividing_by_root_mean_squared_deviation(
        input_values: [f64; 2],
    ) -> [f64; 2] {
        // Нормируем или усредняем величину делением и сохраняем её в `mean`.
        let mean: f64 = (input_values[0] + input_values[1]) / 2.;
        lesson_trace::trace_step!(mean);
        // Комбинируем исходные величины и сохраняем результат в `variance`.
        let variance: f64 = (calculate_square_by_multiplying_number_by_itself(input_values[0] - mean)
            // Складываем или вычитаем величины согласно используемой формуле.
            + calculate_square_by_multiplying_number_by_itself(input_values[1] - mean))
            // Делим значения, получая нормированную величину или среднее.
            / 2.;
        lesson_trace::trace_step!(variance);
        // Составляем результат из вычисленных значений в указанном порядке.
        [
            // Составляем результат из вычисленных значений в указанном порядке.
            // 10⁻⁵ добавляем к дисперсии, чтобы нормализация работала и для равных координат.
            (input_values[0] - mean)
                / approximate_square_root_by_repeated_averaging(variance + 1e-5),
            // Составляем результат из вычисленных значений в указанном порядке.
            (input_values[1] - mean)
                / approximate_square_root_by_repeated_averaging(variance + 1e-5),
        ]
    }

    // Шаг: Пропускаем его через attention, residual, нормализацию и feed-forward.
    let transformer_output: Vec<[f64; 2]> = (|| -> Vec<[f64; 2]> {
        // Используем подготовленное значение в следующем шаге примера.
        /* Собираем causal self-attention, остаточные связи, нормализацию и feed-forward. */
        // Сохраняем результат этого шага в `input_values`.
        let input_values: &[[f64; 2]] = &input_values;
        lesson_trace::trace_step!(input_values);
        // Используем ранее рассчитанное значение `input_values` в текущем выражении.
        input_values
            // Перебираем элементы по ссылке, не копируя исходную коллекцию.
            .iter()
            // Добавляем порядковый индекс к каждому элементу обхода.
            .enumerate()
            // Преобразуем каждый элемент последовательности.
            // Единицу текста, которую модель обрабатывает как одно целое, называют token.
            .map(|(text_unit_index, &query)| {
                // Причинная маска оставляет текущему токену только предшествующие ключи.
                // Оценку модели до преобразования в вероятность называют logit.
                let raw_model_scores: Vec<f64> = input_values
                    // Перебираем элементы по ссылке, не копируя исходную коллекцию.
                    .iter()
                    // Складываем или вычитаем величины согласно используемой формуле.
                    .take(text_unit_index + 1)
                    // Преобразуем каждый элемент последовательности.
                    .map(|key| {
                        // Составляем результат из вычисленных значений в указанном порядке.
                        part_001_lesson_01_multiply_matching_coordinates_of_two_vectors_then_add::calculate_scalar_product_by_multiplying_matching_coordinates_then_adding(&query, key).unwrap()
                                // Делим значения, получая нормированную величину или среднее.
                                / approximate_square_root_by_repeated_averaging(2.0)
                    })
                    // Собираем элементы итератора в итоговую коллекцию.
                    .collect();
                lesson_trace::trace_step!(raw_model_scores);
                // Выполняем встроенный расчёт один раз и сохраняем результат в `attention_weights`.
                let attention_weights: Vec<f64> = (|| -> Vec<f64> {
                    // Используем подготовленное значение в следующем шаге примера.
                    /* Вычитаем максимум для устойчивости, затем нормируем экспоненты. */
                    // Сохраняем результат этого шага в `input_values`.
                    let input_values: &[f64] = &raw_model_scores;
                    lesson_trace::trace_step!(input_values);
                    // Создаём изменяемое значение `maximum_value` для следующих операций.
                    let mut maximum_value: f64 = f64::NEG_INFINITY;
                    lesson_trace::trace_step!(maximum_value);
                    // Повторяем следующий блок для каждого элемента указанной последовательности.
                    for &value in input_values {
                        lesson_trace::trace_step!(value);
                        // Проверяем условие и выбираем соответствующую ветку алгоритма.
                        if value > maximum_value {
                            // Обновляем `maximum_value` результатом текущего шага.
                            maximum_value = value;
                            lesson_trace::trace_step!(maximum_value);
                        }
                    }
                    // Считаем количество элементов и сохраняем его в `exponentials`.
                    let mut exponentials: Vec<f64> = Vec::with_capacity(input_values.len());
                    lesson_trace::trace_step!(exponentials);
                    // Инициализируем изменяемый накопитель `normalizer` начальным состоянием.
                    let mut normalizer: f64 = 0.0;
                    lesson_trace::trace_step!(normalizer);
                    // Повторяем следующий блок для каждого элемента указанной последовательности.
                    for &value in input_values {
                        lesson_trace::trace_step!(value);
                        // Сохраняем рассчитанное значение `exponential_value` для следующих операций.
                        let exponential_value: f64 =
                                // Складываем или вычитаем величины согласно используемой формуле.
                                approximate_e_to_power_by_summing_power_over_factorial_terms(value - maximum_value);
                        lesson_trace::trace_step!(exponential_value);
                        // Сохраняем очередной рассчитанный элемент в коллекции.
                        exponentials.push(exponential_value);
                        // Прибавляем очередной вклад к ранее накопленному результату.
                        normalizer += exponential_value;
                        lesson_trace::trace_step!(normalizer);
                    }
                    // Повторяем следующий блок для каждого элемента указанной последовательности.
                    for exponential_value in &mut exponentials {
                        lesson_trace::trace_step!(exponential_value);
                        // Масштабируем текущую величину делением.
                        *exponential_value /= normalizer;
                        lesson_trace::trace_step!(exponential_value);
                    }
                    // Используем ранее рассчитанное значение `exponentials` в текущем выражении.
                    exponentials
                })();
                lesson_trace::trace_step!(attention_weights);
                // Смешиваем value-векторы по рассчитанным весам внимания.
                let mut attended: [f64; 2] = [0.0, 0.0];
                lesson_trace::trace_step!(attended);
                // Повторяем следующий блок для каждого элемента указанной последовательности.
                for key_index in 0..attention_weights.len() {
                    lesson_trace::trace_step!(key_index);
                    // Прибавляем очередной вклад к ранее накопленному результату.
                    attended[0] += attention_weights[key_index] * input_values[key_index][0];
                    lesson_trace::trace_step!(attended);
                    // Прибавляем очередной вклад к ранее накопленному результату.
                    attended[1] += attention_weights[key_index] * input_values[key_index][1];
                    lesson_trace::trace_step!(attended);
                }
                // После первой остаточной связи применяем нормализацию и простую feed-forward функцию.
                let normalized_values: [f64; 2] = normalize_coordinates_by_subtracting_mean_and_dividing_by_root_mean_squared_deviation([
                    // Складываем или вычитаем величины согласно используемой формуле.
                    query[0] + attended[0],
                    // Складываем или вычитаем величины согласно используемой формуле.
                    query[1] + attended[1],
                ]);
                lesson_trace::trace_step!(normalized_values);
                // Создаём набор значений `feed_forward_values` для следующего шага примера.
                let feed_forward_values: [f64; 2] = [
                    // Вызываем нужное вычисление с подготовленными аргументами.
                    choose_larger_number(normalized_values[0], 0.),
                    // Вызываем нужное вычисление с подготовленными аргументами.
                    choose_larger_number(normalized_values[1], 0.),
                ];
                lesson_trace::trace_step!(feed_forward_values);
                // Вызываем нужное вычисление с подготовленными аргументами.
                normalize_coordinates_by_subtracting_mean_and_dividing_by_root_mean_squared_deviation([
                    // Складываем или вычитаем величины согласно используемой формуле.
                    normalized_values[0] + feed_forward_values[0],
                    // Складываем или вычитаем величины согласно используемой формуле.
                    normalized_values[1] + feed_forward_values[1],
                ])
            })
            // Собираем элементы итератора в итоговую коллекцию.
            .collect()
    })();
    lesson_trace::trace_step!(transformer_output);
    // Печатаем рассчитанные значения для проверки примера.
    println!("transformer block: {transformer_output:?}");

    // Построение графика вынесено из основного кода урока.
    lesson_trace::disable();
    plot_outputs_after_context_mixing_and_coordinate_transformation(
        input_values,
        transformer_output,
    );
}

// Строим график по результатам урока.
fn plot_outputs_after_context_mixing_and_coordinate_transformation(
    input_values: [[f64; 2]; 2],
    transformer_output: std::vec::Vec<[f64; 2]>,
) {
    // Собираем значения для `input_matrix` в коллекцию.
    let input_matrix: Vec<Vec<f64>> = input_values.iter().map(|row| row.to_vec()).collect();
    // Собираем значения для `output_matrix` в коллекцию.
    let output_matrix: Vec<Vec<f64>> = transformer_output.iter().map(|row| row.to_vec()).collect();
    // Повторяем расчёт для каждого элемента последовательности.
    for (name, title, values) in [
        // Добавляем пару значений для сравнения или построения графика.
        ("input", "Вход блока Transformer", &input_matrix),
        // Добавляем пару значений для сравнения или построения графика.
        ("output", "Выход блока Transformer", &output_matrix),
    ] {
        // Строим график по рассчитанным значениям и сохраняем его как SVG.
        let chart: std::path::PathBuf =
            lesson_visualization::heatmap(env!("CARGO_MANIFEST_DIR"), name, title, values)
                // Прерываем пример с понятной ошибкой, если SVG не удалось записать.
                .expect("не удалось сохранить график");
        // Печатаем путь к созданному SVG, чтобы его можно было открыть.
        println!("график: {}", chart.display());
    }
}
