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
use l001_01_calculate_scalar_product_by_multiplying_matching_coordinates_and_adding::calculate_scalar_product_by_multiplying_matching_coordinates_then_adding;

use lesson_trace::{disable, enable_tracing, trace_note, trace_step};

fn main() {
    enable_tracing();
    trace_note!("Шаг: Задаём короткую последовательность двумерных векторов.");
    let sequence: [[f64; 2]; 3] = [[1., 0.], [0., 1.], [1., 1.]];
    trace_step!(sequence);

    trace_note!("Учебные реализации математических операций для этого урока.");

    /// e^x по ряду Тейлора. Деление аргумента пополам ускоряет сходимость.
    /// Учебный аналог `f64::exp`; показывает вычисление ряда и может работать медленнее.
    /// При замене возможны небольшие отличия из-за точности и обработки крайних значений.
    /// Экспонента eˣ: ряд Тейлора 1 + x + x²/2! + x³/3! + … с уменьшением аргумента и восстановлением масштаба.
    fn approximate_e_to_power_by_summing_power_over_factorial_terms(value: f64) -> f64 {
        trace_note!("Проверяем условие и выбираем соответствующую ветку алгоритма.");
        if value == f64::NEG_INFINITY || value < -745.0 {
            trace_note!("Завершаем текущий расчёт и возвращаем найденное значение.");
            return 0.0;
        }
        trace_note!("Проверяем условие и выбираем соответствующую ветку алгоритма.");
        if value == f64::INFINITY || value > 709.0 {
            trace_note!("Завершаем текущий расчёт и возвращаем найденное значение.");
            return f64::INFINITY;
        }
        trace_note!("Проверяем условие и выбираем соответствующую ветку алгоритма.");
        if value < 0.0 {
            trace_note!("Завершаем текущий расчёт и возвращаем найденное значение.");
            return 1.0 / approximate_e_to_power_by_summing_power_over_factorial_terms(-value);
        }
        trace_note!("Создаём изменяемое значение `reduced` для следующих операций.");
        let mut reduced: f64 = value;
        trace_step!(reduced);
        trace_note!("Инициализируем изменяемый накопитель `halving_count` начальным состоянием.");
        let mut halving_count: i32 = 0;
        trace_step!(halving_count);
        trace_note!(
            "Уменьшаем аргумент до ≤0.5: на таком интервале ряд Тейлора для exp сходится быстро."
        );
        while reduced > 0.5 {
            trace_note!("Масштабируем текущую величину делением.");
            reduced /= 2.0;
            trace_step!(reduced);
            trace_note!("Прибавляем очередной вклад к ранее накопленному результату.");
            halving_count += 1;
            trace_step!(halving_count);
        }
        trace_note!("Создаём изменяемое значение `term` для следующих операций.");
        let mut term: f64 = 1.0;
        trace_step!(term);
        trace_note!("Создаём изменяемое значение `result` для следующих операций.");
        let mut result: f64 = 1.0;
        trace_step!(result);
        trace_note!(
            "Берём 30 членов ряда exp(y)=Σ y^k/k!; это предел приближения для учебных входов."
        );
        for term_index in 1..=30 {
            trace_step!(term_index);
            trace_note!("Умножаем накопленное значение на очередной множитель.");
            term *= reduced / term_index as f64;
            trace_step!(term);
            trace_note!("Прибавляем очередной вклад к ранее накопленному результату.");
            result += term;
            trace_step!(result);
        }
        trace_note!(
            "Восстанавливаем exp(value): каждое возведение в квадрат отменяет одно деление аргумента на 2."
        );
        for _ in 0..halving_count {
            trace_note!("Умножаем накопленное значение на очередной множитель.");
            result *= result;
            trace_step!(result);
        }
        trace_note!("Используем ранее рассчитанное значение `result` в текущем выражении.");
        result
    }

    trace_note!("Шаг: Считаем маскированное внимание и получаем новый вектор для каждой позиции.");
    let (attended_output, attention_weights): (Vec<[f64; 2]>, Vec<Vec<f64>>) =
        (|| -> (Vec<[f64; 2]>, Vec<Vec<f64>>) {
            trace_note!("Используем подготовленное значение в следующем шаге примера.");
            trace_note!("Веса внимания получаем из Q·K, нормализуем softmax и применяем к V.");
            trace_note!("Сохраняем результат этого шага в `queries`.");
            let queries: &[[f64; 2]] = &sequence;
            trace_step!(queries);
            trace_note!("Сохраняем рассчитанное значение `keys` для следующих операций.");
            let keys: &[[f64; 2]] = &sequence;
            trace_step!(keys);
            trace_note!("Сохраняем рассчитанное значение `values` для следующих операций.");
            let values: &[[f64; 2]] = &sequence;
            trace_step!(values);
            trace_note!(
                "Сохраняем рассчитанное значение `past_only_attention` для следующих операций."
            );
            trace_note!("Ограничение доступа к будущим значениям называют causal mask.");
            let past_only_attention: bool = true;
            trace_step!(past_only_attention);
            trace_note!(
                "Проверяем, что сравниваемые размерности или значения действительно совпадают."
            );
            assert_eq!(keys.len(), values.len());
            trace_note!("Создаём набор значений `outputs` для следующего шага примера.");
            let mut outputs: Vec<[f64; 2]> = vec![];
            trace_step!(outputs);
            trace_note!("Создаём набор значений `weights` для следующего шага примера.");
            let mut weights: Vec<Vec<f64>> = vec![];
            trace_step!(weights);
            trace_note!(
                "Повторяем следующий блок для каждого элемента указанной последовательности."
            );
            for (query_index, query) in queries.iter().enumerate() {
                trace_step!(query_index);
                trace_step!(query);
                trace_note!("Маска исключает будущие ключи до softmax.");
                trace_note!("Оценку модели до преобразования в вероятность называют logit.");
                trace_note!("Перебираем элементы по ссылке, не копируя исходную коллекцию.");
                trace_note!("Добавляем порядковый индекс к каждому элементу обхода.");
                trace_note!("Преобразуем каждый элемент последовательности.");
                trace_note!("Собираем элементы итератора в итоговую коллекцию.");
                let raw_model_scores: Vec<f64> = keys

                .iter()

                .enumerate()

                .map(|(key_index, key)| {
                    trace_note!("Проверяем условие и выбираем соответствующую ветку алгоритма.");
                    if past_only_attention && key_index > query_index {
                        trace_note!("`f64` задаёт соответствующее входное значение или поле структуры.");
                        f64::NEG_INFINITY

                    } else {
                    trace_note!("Обрабатываем случай, когда предыдущее условие не выполнено.");
                        trace_note!("Составляем результат из вычисленных значений в указанном порядке.");
                        trace_note!("Делим значения, получая нормированную величину или среднее.");
                        calculate_scalar_product_by_multiplying_matching_coordinates_then_adding(query, key).unwrap()

                            / (|| -> f64 {
        trace_note!("Обновляем значение результатом текущего вычисления.");
        trace_note!("Корень через итерацию Ньютона: x_(n+1) = (x_n + value / x_n) / 2.");
        trace_note!("Сохраняем результат этого шага в `value`.");
        let value: f64 = 2.0;
        trace_step!(value);
        trace_note!("Проверяем обязательное условие до дальнейшего вычисления.");
        assert!(value >= 0.0, "корень из отрицательного числа");
        trace_note!("Проверяем условие и выбираем соответствующую ветку алгоритма.");
        if value == 0.0 {
            trace_note!("Завершаем текущий расчёт и возвращаем найденное значение.");
            return 0.0;
        }
        trace_note!("Создаём изменяемое значение `estimate` для следующих операций.");
        let mut estimate: f64 = if value > 1.0 { value } else { 1.0 };
        trace_step!(estimate);
        trace_note!("80 шагов Ньютона дают оценку √value с запасом для небольших учебных входов.");
        for _ in 0..80 {
            trace_note!("Среднее estimate и value/estimate приближает квадратный корень.");
            estimate = (estimate + value / estimate) / 2.0;
            trace_step!(estimate);
        }
        trace_note!("Используем ранее рассчитанное значение `estimate` в текущем выражении.");
        estimate

    })()
                    }
                })

                .collect();
                trace_step!(raw_model_scores);
                trace_note!(
                    "Отдельная функция вычитает максимум, считает экспоненты и нормирует их сумму."
                );
                let attention_weights: Vec<f64> = (|| -> Vec<f64> {
                    trace_note!("Используем подготовленное значение в следующем шаге примера.");
                    trace_note!("Превращаем оценки внимания в веса с суммой, равной единице.");
                    trace_note!("Сохраняем результат этого шага в `raw_model_scores`.");
                    let raw_model_scores: &[f64] = &raw_model_scores;
                    trace_step!(raw_model_scores);
                    trace_note!(
                        "Создаём изменяемое значение `maximum_raw_model_score` для следующих операций."
                    );
                    let mut maximum_raw_model_score: f64 = f64::NEG_INFINITY;
                    trace_step!(maximum_raw_model_score);
                    trace_note!(
                        "Повторяем следующий блок для каждого элемента указанной последовательности."
                    );
                    for &raw_model_score in raw_model_scores {
                        trace_step!(raw_model_score);
                        trace_note!(
                            "Проверяем условие и выбираем соответствующую ветку алгоритма."
                        );
                        if raw_model_score > maximum_raw_model_score {
                            trace_note!(
                                "Обновляем `maximum_raw_model_score` результатом текущего шага."
                            );
                            maximum_raw_model_score = raw_model_score;
                            trace_step!(maximum_raw_model_score);
                        }
                    }
                    trace_note!("Считаем количество элементов и сохраняем его в `exponentials`.");
                    let mut exponentials: Vec<f64> = Vec::with_capacity(raw_model_scores.len());
                    trace_step!(exponentials);
                    trace_note!(
                        "Инициализируем изменяемый накопитель `normalizer` начальным состоянием."
                    );
                    let mut normalizer: f64 = 0.0;
                    trace_step!(normalizer);
                    trace_note!(
                        "Повторяем следующий блок для каждого элемента указанной последовательности."
                    );
                    for &raw_model_score in raw_model_scores {
                        trace_step!(raw_model_score);
                        trace_note!(
                            "Сохраняем рассчитанное значение `exponential_value` для следующих операций."
                        );
                        trace_note!(
                            "Складываем или вычитаем величины согласно используемой формуле."
                        );
                        let exponential_value: f64 =
                            approximate_e_to_power_by_summing_power_over_factorial_terms(
                                raw_model_score - maximum_raw_model_score,
                            );
                        trace_step!(exponential_value);
                        trace_note!("Сохраняем очередной рассчитанный элемент в коллекции.");
                        exponentials.push(exponential_value);
                        trace_note!("Прибавляем очередной вклад к ранее накопленному результату.");
                        normalizer += exponential_value;
                        trace_step!(normalizer);
                    }
                    trace_note!(
                        "Повторяем следующий блок для каждого элемента указанной последовательности."
                    );
                    for exponential_value in &mut exponentials {
                        trace_step!(exponential_value);
                        trace_note!("Масштабируем текущую величину делением.");
                        *exponential_value /= normalizer;
                        trace_step!(exponential_value);
                    }
                    trace_note!(
                        "Используем ранее рассчитанное значение `exponentials` в текущем выражении."
                    );
                    exponentials
                })();
                trace_step!(attention_weights);
                trace_note!("Взвешенная сумма value-векторов становится выходом текущего токена.");
                let mut attended_vector: [f64; 2] = [0.0, 0.0];
                trace_step!(attended_vector);
                trace_note!(
                    "Повторяем следующий блок для каждого элемента указанной последовательности."
                );
                for value_index in 0..values.len() {
                    trace_step!(value_index);
                    trace_note!("Прибавляем очередной вклад к ранее накопленному результату.");
                    attended_vector[0] += attention_weights[value_index] * values[value_index][0];
                    trace_step!(attended_vector);
                    trace_note!("Прибавляем очередной вклад к ранее накопленному результату.");
                    attended_vector[1] += attention_weights[value_index] * values[value_index][1];
                    trace_step!(attended_vector);
                }
                trace_note!("Сохраняем очередной рассчитанный элемент в коллекции.");
                outputs.push(attended_vector);
                trace_note!("Сохраняем очередной рассчитанный элемент в коллекции.");
                weights.push(attention_weights);
            }
            trace_note!("Составляем результат из вычисленных значений в указанном порядке.");
            (outputs, weights)
        })();
    trace_step!(attended_output);
    trace_step!(attention_weights);
    trace_note!("Шаг: Печатаем веса, чтобы увидеть запрет доступа к будущим токенам.");
    println!("weights={attention_weights:?}, output={attended_output:?}");

    trace_note!("Построение графика вынесено из основного кода урока.");
    disable();
    plot_weights_assigned_to_current_and_past_positions(attention_weights);
}

// Строим график по результатам урока.
fn plot_weights_assigned_to_current_and_past_positions(
    attention_weights: std::vec::Vec<std::vec::Vec<f64>>,
) {
    trace_note!("Значения ячеек видны по цвету и подписи.");
    trace_note!("Передаём путь к каталогу текущего урока.");
    trace_note!("Указываем имя SVG-файла.");
    trace_note!("Указываем заголовок тепловой карты.");
    trace_note!("Используем подготовленное значение в следующем шаге примера.");
    trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
    let chart: std::path::PathBuf = lesson_visualization::heatmap(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Причинные веса внимания",
        &attention_weights,
    )
    .expect("не удалось сохранить тепловую карту");
    trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
