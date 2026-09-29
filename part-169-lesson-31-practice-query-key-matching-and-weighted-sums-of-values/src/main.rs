// Урок 31.7. Практика: сравнение запросов с ключами и взвешенное сложение значений.
// Связь с принятой терминологией: Масштабированное внимание с векторами запроса, ключа и значения.
// Зачем здесь эта тема: Полное внимание связывает Q, K, V, масштабирование, маску и взвешенную
//   сумму.
// Почему код устроен так: На нескольких коротких векторах проверяем отдельно оценки, веса и выход.
// Представь: Запрос сравнивается с ключами, веса выбирают важные позиции, а их V складываются с
//   этими весами.
//
// Что повторяем вместе: Q, K, V, масштабированная сумма после попарного умножения координат, softmax, causal mask.
// Зачем это нужно: Внимание взвешивает информацию от разных позиций; causal mask запрещает токену видеть
//   будущее.
// Что показывает программа: Задаём короткую последовательность двумерных векторов. Считаем маскированное
//   внимание и получаем новый вектор для каждой позиции. Печатаем веса, чтобы увидеть запрет доступа к
//   будущим токенам.
// Что проверить при изменении примера: Проверь суммы весов, форму выхода и отсутствие доступа к будущим
//   позициям при causal mask.
// Дополнительная практика: Реализуй single-head attention для короткой последовательности без готового слоя.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    lesson_trace::enable();
    // Шаг: Задаём короткую последовательность двумерных векторов.
    let sequence: [[f64; 2]; 3] = [[1., 0.], [0., 1.], [1., 1.]];
    lesson_trace::trace_step!(sequence);

    // Учебные реализации математических операций для этого урока.

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

    // Шаг: Считаем маскированное внимание и получаем новый вектор для каждой позиции.
    let (attended_output, attention_weights): (Vec<[f64; 2]>, Vec<Vec<f64>>) =
        (|| -> (Vec<[f64; 2]>, Vec<Vec<f64>>) {
            // Используем подготовленное значение в следующем шаге примера.
            /* Веса внимания получаем из Q·K, нормализуем softmax и применяем к V. */
            // Сохраняем результат этого шага в `queries`.
            let queries: &[[f64; 2]] = &sequence;
            lesson_trace::trace_step!(queries);
            // Сохраняем рассчитанное значение `keys` для следующих операций.
            let keys: &[[f64; 2]] = &sequence;
            lesson_trace::trace_step!(keys);
            // Сохраняем рассчитанное значение `values` для следующих операций.
            let values: &[[f64; 2]] = &sequence;
            lesson_trace::trace_step!(values);
            // Сохраняем рассчитанное значение `past_only_attention` для следующих операций.
            // Ограничение доступа к будущим значениям называют causal mask.
            let past_only_attention: bool = true;
            lesson_trace::trace_step!(past_only_attention);
            // Проверяем, что сравниваемые размерности или значения действительно совпадают.
            assert_eq!(keys.len(), values.len());
            // Создаём набор значений `outputs` для следующего шага примера.
            let mut outputs: Vec<[f64; 2]> = vec![];
            lesson_trace::trace_step!(outputs);
            // Создаём набор значений `weights` для следующего шага примера.
            let mut weights: Vec<Vec<f64>> = vec![];
            lesson_trace::trace_step!(weights);
            // Повторяем следующий блок для каждого элемента указанной последовательности.
            for (query_index, query) in queries.iter().enumerate() {
                lesson_trace::trace_step!(query_index);
                lesson_trace::trace_step!(query);
                // Маска исключает будущие ключи до softmax.
                // Оценку модели до преобразования в вероятность называют logit.
                let raw_model_scores: Vec<f64> = keys
                // Перебираем элементы по ссылке, не копируя исходную коллекцию.
                .iter()
                // Добавляем порядковый индекс к каждому элементу обхода.
                .enumerate()
                // Преобразуем каждый элемент последовательности.
                .map(|(key_index, key)| {
                    // Проверяем условие и выбираем соответствующую ветку алгоритма.
                    if past_only_attention && key_index > query_index {
                        // `f64` задаёт соответствующее входное значение или поле структуры.
                        f64::NEG_INFINITY
                    // Обрабатываем случай, когда предыдущее условие не выполнено.
                    } else {
                        // Составляем результат из вычисленных значений в указанном порядке.
                        part_001_lesson_01_multiply_matching_coordinates_of_two_vectors_then_add::multiply_matching_coordinates_of_two_vectors_then_add(query, key).unwrap()
                            // Делим значения, получая нормированную величину или среднее.
                            / (|| -> f64 {
        // Обновляем значение результатом текущего вычисления.
        /* Корень через итерацию Ньютона: x_(n+1) = (x_n + value / x_n) / 2. */
        // Сохраняем результат этого шага в `value`.
        let value: f64 = 2.0;
        lesson_trace::trace_step!(value);
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
        // 80 шагов Ньютона дают оценку √value с запасом для небольших учебных входов.
        for _ in 0..80 {
            // Среднее estimate и value/estimate приближает квадратный корень.
            estimate = (estimate + value / estimate) / 2.0;
            lesson_trace::trace_step!(estimate);
        }
        // Используем ранее рассчитанное значение `estimate` в текущем выражении.
        estimate

    })()
                    }
                })
                // Собираем элементы итератора в итоговую коллекцию.
                .collect();
                lesson_trace::trace_step!(raw_model_scores);
                // Отдельная функция вычитает максимум, считает экспоненты и нормирует их сумму.
                let attention_weights: Vec<f64> = (|| -> Vec<f64> {
                    // Используем подготовленное значение в следующем шаге примера.
                    /* Превращаем оценки внимания в веса с суммой, равной единице. */
                    // Сохраняем результат этого шага в `raw_model_scores`.
                    let raw_model_scores: &[f64] = &raw_model_scores;
                    lesson_trace::trace_step!(raw_model_scores);
                    // Создаём изменяемое значение `maximum_raw_model_score` для следующих операций.
                    let mut maximum_raw_model_score: f64 = f64::NEG_INFINITY;
                    lesson_trace::trace_step!(maximum_raw_model_score);
                    // Повторяем следующий блок для каждого элемента указанной последовательности.
                    for &raw_model_score in raw_model_scores {
                        lesson_trace::trace_step!(raw_model_score);
                        // Проверяем условие и выбираем соответствующую ветку алгоритма.
                        if raw_model_score > maximum_raw_model_score {
                            // Обновляем `maximum_raw_model_score` результатом текущего шага.
                            maximum_raw_model_score = raw_model_score;
                            lesson_trace::trace_step!(maximum_raw_model_score);
                        }
                    }
                    // Считаем количество элементов и сохраняем его в `exponentials`.
                    let mut exponentials: Vec<f64> = Vec::with_capacity(raw_model_scores.len());
                    lesson_trace::trace_step!(exponentials);
                    // Инициализируем изменяемый накопитель `normalizer` начальным состоянием.
                    let mut normalizer: f64 = 0.0;
                    lesson_trace::trace_step!(normalizer);
                    // Повторяем следующий блок для каждого элемента указанной последовательности.
                    for &raw_model_score in raw_model_scores {
                        lesson_trace::trace_step!(raw_model_score);
                        // Сохраняем рассчитанное значение `exponential_value` для следующих операций.
                        let exponential_value: f64 =
                        // Складываем или вычитаем величины согласно используемой формуле.
                        approximate_e_to_power_by_summing_power_over_factorial_terms(raw_model_score - maximum_raw_model_score);
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
                // Взвешенная сумма value-векторов становится выходом текущего токена.
                let mut attended_vector: [f64; 2] = [0.0, 0.0];
                lesson_trace::trace_step!(attended_vector);
                // Повторяем следующий блок для каждого элемента указанной последовательности.
                for value_index in 0..values.len() {
                    lesson_trace::trace_step!(value_index);
                    // Прибавляем очередной вклад к ранее накопленному результату.
                    attended_vector[0] += attention_weights[value_index] * values[value_index][0];
                    lesson_trace::trace_step!(attended_vector);
                    // Прибавляем очередной вклад к ранее накопленному результату.
                    attended_vector[1] += attention_weights[value_index] * values[value_index][1];
                    lesson_trace::trace_step!(attended_vector);
                }
                // Сохраняем очередной рассчитанный элемент в коллекции.
                outputs.push(attended_vector);
                // Сохраняем очередной рассчитанный элемент в коллекции.
                weights.push(attention_weights);
            }
            // Составляем результат из вычисленных значений в указанном порядке.
            (outputs, weights)
        })();
    lesson_trace::trace_step!(attended_output);
    lesson_trace::trace_step!(attention_weights);
    // Шаг: Печатаем веса, чтобы увидеть запрет доступа к будущим токенам.
    println!("weights={attention_weights:?}, output={attended_output:?}");

    // Построение графика вынесено из основного кода урока.
    lesson_trace::disable();
    plot_weights_assigned_to_current_and_past_positions(attention_weights);
}

// Строим график по результатам урока.
fn plot_weights_assigned_to_current_and_past_positions(
    attention_weights: std::vec::Vec<std::vec::Vec<f64>>,
) {
    // Значения ячеек видны по цвету и подписи.
    let chart: std::path::PathBuf = lesson_visualization::heatmap(
        // Передаём путь к каталогу текущего урока.
        env!("CARGO_MANIFEST_DIR"),
        // Указываем имя SVG-файла.
        "lesson-chart",
        // Указываем заголовок тепловой карты.
        "Причинные веса внимания",
        // Используем подготовленное значение в следующем шаге примера.
        &attention_weights,
    )
    // Прерываем пример с понятной ошибкой, если SVG не удалось записать.
    .expect("не удалось сохранить тепловую карту");
    // Печатаем путь к созданному SVG, чтобы его можно было открыть.
    println!("график: {}", chart.display());
}
