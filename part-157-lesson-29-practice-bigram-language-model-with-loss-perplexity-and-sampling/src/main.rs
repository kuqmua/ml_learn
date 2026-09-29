// Сводная практика 29. Биграммная языковая модель с ошибкой и выборкой токенов.
//
// Что повторяем вместе: предсказание следующего токена, cross-entropy, perplexity, sampling.
// Зачем это нужно: Биграммная модель оценивает следующее слово по предыдущему; perplexity измеряет качество
//   вероятностного прогноза.
// Что показывает программа: Задаём маленький корпус для подсчёта биграмм. Считаем частоты переходов и
//   словарь возможных следующих токенов. Сравниваем perplexity на обучающих и новых сочетаниях слов.
// Что проверить при изменении примера: Сравни train/validation perplexity и покажи влияние temperature и
//   seed.
// Дополнительная практика: Обучи маленькую n-gram модель или tiny decoder на игрушечном корпусе; реализуй
//   генерацию.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    lesson_trace::enable();
    // Шаг: Задаём маленький корпус для подсчёта биграмм.
    let training_sentences: [&str; 3] = ["кот спит", "кот ест", "пёс спит"];
    lesson_trace::trace_step!(training_sentences);

    // Шаг: Считаем частоты переходов и словарь возможных следующих токенов.
    // Набор известных модели текстовых единиц называют vocabulary.
    let (bigram_counts, known_text_units): (std::collections::BTreeMap<(String, String), usize>, std::collections::BTreeSet<String>) =
        // Составляем результат из вычисленных значений в указанном порядке.
        (|| -> (std::collections::BTreeMap<(String, String), usize>, std::collections::BTreeSet<String>) {
            // Используем подготовленное значение в следующем шаге примера.
            /* Считаем пары соседних слов и собираем словарь возможных продолжений. */
            // Сохраняем результат этого шага в `sentences`.
            let sentences: &[&str] = &training_sentences;
            lesson_trace::trace_step!(sentences);
            // Инициализируем изменяемый накопитель `counts` начальным состоянием.
            let mut counts: std::collections::BTreeMap<(String, String), usize> = std::collections::BTreeMap::new();
            lesson_trace::trace_step!(counts);
            // Создаём изменяемое значение `known_text_units` для следующих операций.
            let mut known_text_units: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
            lesson_trace::trace_step!(known_text_units);
            // Повторяем следующий блок для каждого элемента указанной последовательности.
            for sentence in sentences {
                lesson_trace::trace_step!(sentence);
                // Создаём изменяемое значение `previous` для следующих операций.
                let mut previous: &str = "<s>";
                lesson_trace::trace_step!(previous);
                // Повторяем следующий блок для каждого элемента указанной последовательности.
                for word in sentence.split_whitespace().chain(["</s>"]) {
                    lesson_trace::trace_step!(word);
                    // Прибавляем очередной вклад к ранее накопленному результату.
                    *counts.entry((previous.into(), word.into())).or_insert(0) += 1;
                    lesson_trace::trace_step!(counts);
                    // Выполняем очередное действие, после которого продолжаем следующий шаг.
                    known_text_units.insert(word.into());
                    // Обновляем `previous` результатом текущего шага.
                    previous = word;
                    lesson_trace::trace_step!(previous);
                }
            }
            // Составляем результат из вычисленных значений в указанном порядке.
            (counts, known_text_units)
        })();
    lesson_trace::trace_step!(bigram_counts);
    lesson_trace::trace_step!(known_text_units);

    // Учебные реализации математических операций для этого урока.

    /// e^x по ряду Тейлора. Деление аргумента пополам ускоряет сходимость.
    fn approximate_exponential_with_taylor_series(value: f64) -> f64 {
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
            return 1.0 / approximate_exponential_with_taylor_series(-value);
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

    // Сглаживание Лапласа даёт ненулевую вероятность непоказанным биграммам.
    fn calculate_smoothed_next_token_probability(
        // Получаем таблицу частот биграмм для оценки вероятности перехода.
        counts: &std::collections::BTreeMap<(String, String), usize>,
        // `known_text_units` задаёт соответствующее входное значение или поле структуры.
        known_text_units: &std::collections::BTreeSet<String>,
        // `previous_text_unit` задаёт соответствующее входное значение или поле структуры.
        // Единицу текста, которую модель обрабатывает как одно целое, называют token.
        previous_text_unit: &str,
        // `next_text_unit` задаёт соответствующее входное значение или поле структуры.
        next_text_unit: &str,
        // Указываем тип возвращаемого значения.
    ) -> f64 {
        // Инициализируем изменяемый накопитель `total` начальным состоянием.
        let mut total: usize = 0;
        lesson_trace::trace_step!(total);
        // Повторяем следующий блок для каждого элемента указанной последовательности.
        for ((previous_context, _), &text_unit_count) in counts {
            lesson_trace::trace_step!(previous_context);
            lesson_trace::trace_step!(text_unit_count);
            // Проверяем условие и выбираем соответствующую ветку алгоритма.
            if previous_context == previous_text_unit {
                // Прибавляем очередной вклад к ранее накопленному результату.
                total += text_unit_count;
                lesson_trace::trace_step!(total);
            }
        }
        // Составляем результат из вычисленных значений в указанном порядке.
        (*counts
            // Ищем сохранённую частоту указанной пары слов.
            .get(&(previous_text_unit.into(), next_text_unit.into()))
            // При отсутствии значения используем запасной вариант.
            .unwrap_or(&0) as f64
            // Складываем или вычитаем величины согласно используемой формуле.
            + 1.)
            // Делим значения, получая нормированную величину или среднее.
            / (total + known_text_units.len()) as f64
    }

    // Объявляем повторно используемое вычисление `calculate_perplexity_of_sentences`; параметры ниже задают его входы.
    fn calculate_perplexity_of_sentences(
        // `sentences` задаёт соответствующее входное значение или поле структуры.
        sentences: &[&str],
        // Получаем таблицу частот биграмм для оценки вероятности перехода.
        counts: &std::collections::BTreeMap<(String, String), usize>,
        // `known_text_units` задаёт соответствующее входное значение или поле структуры.
        known_text_units: &std::collections::BTreeSet<String>,
        // Указываем тип возвращаемого значения.
    ) -> f64 {
        // Сохраняем рассчитанное значение `(mut token_count, mut negative_log_likelihood)` для следующих операций.
        let (mut text_unit_count, mut negative_log_likelihood): (i32, f64) = (0, 0.);
        lesson_trace::trace_step!(text_unit_count);
        lesson_trace::trace_step!(negative_log_likelihood);
        // Повторяем следующий блок для каждого элемента указанной последовательности.
        for sentence in sentences {
            lesson_trace::trace_step!(sentence);
            // Создаём изменяемое значение `previous_text_unit` для следующих операций.
            let mut previous_text_unit: &str = "<s>";
            lesson_trace::trace_step!(previous_text_unit);
            // Повторяем следующий блок для каждого элемента указанной последовательности.
            for word in sentence.split_whitespace().chain(["</s>"]) {
                lesson_trace::trace_step!(word);
                // Вычитаем очередной вклад из текущего значения параметра.
                negative_log_likelihood -= (|| -> f64 {
                    // Обновляем значение результатом текущего вычисления.
                    /* ln(x) через ряд 2 * (t + t³/3 + t⁵/5 + ...), t=(x-1)/(x+1). */
                    // Сохраняем результат этого шага в `value`.
                    let value: f64 = calculate_smoothed_next_token_probability(
                        // Используем ранее рассчитанное значение `counts` в текущем выражении.
                        counts,
                        // Используем ранее рассчитанное значение `known_text_units` в текущем выражении.
                        known_text_units,
                        // Используем ранее рассчитанное значение `previous_text_unit` в текущем выражении.
                        previous_text_unit,
                        // Используем ранее рассчитанное значение `word` в текущем выражении.
                        word,
                    );
                    lesson_trace::trace_step!(value);
                    // Проверяем обязательное условие до дальнейшего вычисления.
                    assert!(
                        // Передаём очередное значение в составе результата или вызова.
                        value > 0.0,
                        // Подставляем результаты в этот шаблон вывода или текстового значения.
                        "логарифм определён только для положительных чисел"
                    );
                    // Проверяем условие и выбираем соответствующую ветку алгоритма.
                    if value == f64::INFINITY {
                        // Завершаем текущий расчёт и возвращаем найденное значение.
                        return f64::INFINITY;
                    }
                    // Создаём изменяемое значение `scaled` для следующих операций.
                    let mut scaled: f64 = value;
                    lesson_trace::trace_step!(scaled);
                    // Инициализируем изменяемый накопитель `power_of_two` начальным состоянием.
                    let mut power_of_two: i32 = 0i32;
                    lesson_trace::trace_step!(power_of_two);
                    // Повторяем вычисление, пока выполняется указанное условие.
                    while scaled >= 2.0 {
                        // Масштабируем текущую величину делением.
                        scaled /= 2.0;
                        lesson_trace::trace_step!(scaled);
                        // Прибавляем очередной вклад к ранее накопленному результату.
                        power_of_two += 1;
                        lesson_trace::trace_step!(power_of_two);
                    }
                    // Повторяем вычисление, пока выполняется указанное условие.
                    while scaled < 1.0 {
                        // Умножаем накопленное значение на очередной множитель.
                        scaled *= 2.0;
                        lesson_trace::trace_step!(scaled);
                        // Вычитаем очередной вклад из текущего значения параметра.
                        power_of_two -= 1;
                        lesson_trace::trace_step!(power_of_two);
                    }
                    // Объявляем повторно используемое вычисление `sum_logarithm_series_terms`; параметры ниже задают его входы.
                    fn sum_logarithm_series_terms(value: f64) -> f64 {
                        // Нормируем или усредняем величину делением и сохраняем её в `ratio`.
                        let ratio: f64 = (value - 1.0) / (value + 1.0);
                        lesson_trace::trace_step!(ratio);
                        // Умножаем значения и сохраняем результат в `ratio_squared`.
                        let ratio_squared: f64 = ratio * ratio;
                        lesson_trace::trace_step!(ratio_squared);
                        // Создаём изменяемое значение `term` для следующих операций.
                        let mut term: f64 = ratio;
                        lesson_trace::trace_step!(term);
                        // Инициализируем изменяемый накопитель `result` начальным состоянием.
                        let mut result: f64 = 0.0;
                        lesson_trace::trace_step!(result);
                        // Используем 40 первых членов ряда ln(value) = 2·Σ ratio^(2k+1)/(2k+1).
                        // Это конечное приближение: для положительного value выполняется |ratio| < 1.
                        for term_index in 0..40 {
                            lesson_trace::trace_step!(term_index);
                            // Прибавляем очередной вклад к ранее накопленному результату.
                            result += term / (2 * term_index + 1) as f64;
                            lesson_trace::trace_step!(result);
                            // Умножаем накопленное значение на очередной множитель.
                            term *= ratio_squared;
                            lesson_trace::trace_step!(term);
                        }
                        // Умножаем величины согласно используемой формуле.
                        2.0 * result
                    }
                    // Сохраняем рассчитанное значение `logarithm_of_two` для следующих операций.
                    let logarithm_of_two: f64 = sum_logarithm_series_terms(2.0);
                    lesson_trace::trace_step!(logarithm_of_two);
                    // Умножаем величины согласно используемой формуле.
                    sum_logarithm_series_terms(scaled) + power_of_two as f64 * logarithm_of_two
                })();
                lesson_trace::trace_step!(negative_log_likelihood);
                // Прибавляем очередной вклад к ранее накопленному результату.
                text_unit_count += 1;
                lesson_trace::trace_step!(text_unit_count);
                // Обновляем `previous_text_unit` результатом текущего шага.
                previous_text_unit = word;
                lesson_trace::trace_step!(previous_text_unit);
            }
        }
        // Делим значения, получая нормированную величину или среднее.
        approximate_exponential_with_taylor_series(negative_log_likelihood / text_unit_count as f64)
    }

    // Шаг: Сравниваем perplexity на обучающих и новых сочетаниях слов.
    println!(
        // Задаём шаблон строки: плейсхолдеры ниже заменятся рассчитанными значениями.
        "train perplexity={:.3}, validation perplexity={:.3}",
        // Вызываем нужное вычисление с подготовленными аргументами.
        calculate_perplexity_of_sentences(&training_sentences, &bigram_counts, &known_text_units),
        // Вызываем нужное вычисление с подготовленными аргументами.
        calculate_perplexity_of_sentences(&["пёс ест"], &bigram_counts, &known_text_units)
    );
    // Шаг: Генерируем цепочку, каждый раз выбирая наиболее вероятный следующий токен.
    let mut previous_text_unit: &str = "<s>";
    lesson_trace::trace_step!(previous_text_unit);
    // Создаём набор значений `generated_text_units` для следующего шага примера.
    let mut generated_text_units: Vec<&str> = vec![];
    lesson_trace::trace_step!(generated_text_units);
    // Генерируем ровно пять следующих токенов, чтобы показать короткое продолжение фразы.
    for _ in 0..5 {
        // Сохраняем рассчитанное значение `next_text_unit` для следующих операций.
        let next_text_unit: &String = known_text_units
            // Перебираем элементы по ссылке, не копируя исходную коллекцию.
            .iter()
            // Сравниваем кандидатов и оставляем наибольший результат.
            .max_by(|first_candidate, second_candidate| {
                // Вызываем нужное вычисление с подготовленными аргументами.
                calculate_smoothed_next_token_probability(
                    // Передаём данные по ссылке или разыменовываем их для следующей операции.
                    &bigram_counts,
                    // Передаём данные по ссылке или разыменовываем их для следующей операции.
                    &known_text_units,
                    // Используем ранее рассчитанное значение `previous_text_unit` в текущем выражении.
                    previous_text_unit,
                    // Используем ранее рассчитанное значение `first_candidate` в текущем выражении.
                    first_candidate,
                )
                // Сравниваем числа с полным порядком, включая специальные значения.
                .total_cmp(&calculate_smoothed_next_token_probability(
                    // Передаём данные по ссылке или разыменовываем их для следующей операции.
                    &bigram_counts,
                    // Передаём данные по ссылке или разыменовываем их для следующей операции.
                    &known_text_units,
                    // Используем ранее рассчитанное значение `previous_text_unit` в текущем выражении.
                    previous_text_unit,
                    // Используем ранее рассчитанное значение `second_candidate` в текущем выражении.
                    second_candidate,
                ))
            })
            // Извлекаем значение: выше в примере обеспечено отсутствие ошибки.
            .unwrap();
        lesson_trace::trace_step!(next_text_unit);
        // Проверяем условие и выбираем соответствующую ветку алгоритма.
        if next_text_unit == "</s>" {
            // Останавливаем цикл после достижения условия завершения.
            break;
        }
        // Сохраняем очередной рассчитанный элемент в коллекции.
        generated_text_units.push(next_text_unit.as_str());
        // Обновляем `previous_text_unit` результатом текущего шага.
        previous_text_unit = next_text_unit;
        lesson_trace::trace_step!(previous_text_unit);
    }
    // Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением.
    println!("generated: {}", generated_text_units.join(" "));

    // Построение графика вынесено из основного кода урока.
    lesson_trace::disable();
    visualize_practice_bigram_language_model_with_loss_perplexity_and_sampling(bigram_counts);
}

// Строим график по результатам урока.
fn visualize_practice_bigram_language_model_with_loss_perplexity_and_sampling(
    bigram_counts: std::collections::BTreeMap<(std::string::String, std::string::String), usize>,
) {
    // Наглядное представление вычислений сводной практики.
    let language_model_points: Vec<(f64, f64)> = bigram_counts
        // Просматриваем элементы коллекции по ссылке.
        .iter()
        // Оставляем элементы, отвечающие условию.
        .filter(|((previous, _), _)| previous == "<s>")
        // Добавляем порядковый номер к каждому элементу.
        .enumerate()
        // Преобразуем каждый элемент в новое значение.
        .map(|(item_index, (_, count))| (item_index as f64, *count as f64))
        // Собираем результаты в коллекцию.
        .collect();
    // Строим график по рассчитанным значениям и сохраняем его как SVG.
    let chart: std::path::PathBuf = lesson_visualization::line_chart(
        // Передаём путь к каталогу текущего урока.
        env!("CARGO_MANIFEST_DIR"),
        // Указываем имя SVG-файла.
        "lesson-chart",
        // Указываем заголовок графика.
        "Частоты переходов после начала строки",
        // Указываем подпись горизонтальной оси.
        "номер следующего токена",
        // Указываем подпись вертикальной оси.
        "частота",
        // Передаём ряды или значения для отрисовки графика.
        &[lesson_visualization::Series {
            // Указываем подпись этого ряда в легенде.
            name: "биграммы",
            // Передаём рассчитанные координаты точек.
            points: &language_model_points,
        }],
    )
    // Прерываем пример с понятной ошибкой, если SVG не удалось записать.
    .expect("не удалось сохранить график");
    // Печатаем путь к созданному SVG, чтобы его можно было открыть.
    println!("график: {}", chart.display());
}
