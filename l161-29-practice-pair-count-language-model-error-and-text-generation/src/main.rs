// Урок 29.5. Практика: модель текста по частотам пар, оценка ошибки и генерация продолжения.
// Связь с принятой терминологией: Биграммная языковая модель с ошибкой и выборкой токенов.
// Зачем здесь эта тема: Биграммная модель позволяет увидеть полный цикл: счётчики, вероятности,
//   loss и генерация.
// Почему код устроен так: Оставляем контекст длиной один, чтобы каждый шаг можно было проверить по
//   таблице частот.
// Представь: Из частот пар получаем вероятности, по ним считаем ошибку, затем выбираем следующий
//   токен.
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
use lesson_trace::{disable, enable, trace_note, trace_step};

fn main() {
    enable();
    trace_note!("Шаг: Задаём маленький корпус для подсчёта биграмм.");
    let training_sentences: [&str; 3] = ["кот спит", "кот ест", "пёс спит"];
    trace_step!(training_sentences);

    trace_note!("Шаг: Считаем частоты переходов и словарь возможных следующих токенов.");
    trace_note!("Набор известных модели текстовых единиц называют vocabulary.");
    trace_note!("Составляем результат из вычисленных значений в указанном порядке.");
    let (bigram_counts, known_text_units): (std::collections::BTreeMap<(String, String), usize>, std::collections::BTreeSet<String>) =

        (|| -> (std::collections::BTreeMap<(String, String), usize>, std::collections::BTreeSet<String>) {
            trace_note!("Используем подготовленное значение в следующем шаге примера.");
            trace_note!("Считаем пары соседних слов и собираем словарь возможных продолжений.");
            trace_note!("Сохраняем результат этого шага в `sentences`.");
            let sentences: &[&str] = &training_sentences;
            trace_step!(sentences);
            trace_note!("Инициализируем изменяемый накопитель `counts` начальным состоянием.");
            let mut counts: std::collections::BTreeMap<(String, String), usize> = std::collections::BTreeMap::new();
            trace_step!(counts);
            trace_note!("Создаём изменяемое значение `known_text_units` для следующих операций.");
            let mut known_text_units: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
            trace_step!(known_text_units);
            trace_note!("Повторяем следующий блок для каждого элемента указанной последовательности.");
            for sentence in sentences {
                trace_step!(sentence);
                trace_note!("Создаём изменяемое значение `previous` для следующих операций.");
                let mut previous: &str = "<s>";
                trace_step!(previous);
                trace_note!("Повторяем следующий блок для каждого элемента указанной последовательности.");
                for word in sentence.split_whitespace().chain(["</s>"]) {
                    trace_step!(word);
                    trace_note!("Прибавляем очередной вклад к ранее накопленному результату.");
                    *counts.entry((previous.into(), word.into())).or_insert(0) += 1;
                    trace_step!(counts);
                    trace_note!("Выполняем очередное действие, после которого продолжаем следующий шаг.");
                    known_text_units.insert(word.into());
                    trace_note!("Обновляем `previous` результатом текущего шага.");
                    previous = word;
                    trace_step!(previous);
                }
            }
            trace_note!("Составляем результат из вычисленных значений в указанном порядке.");
            (counts, known_text_units)
        })();
    trace_step!(bigram_counts);
    trace_step!(known_text_units);

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

    trace_note!("Сглаживание Лапласа даёт ненулевую вероятность непоказанным биграммам.");
    /// Сглаженная вероятность следующего токена: (число пары + 1) / (число переходов из контекста + размер словаря).
    fn calculate_next_token_probability_as_pair_count_plus_one_over_context_count_plus_vocabulary_size(
        counts: &std::collections::BTreeMap<(String, String), usize>,

        known_text_units: &std::collections::BTreeSet<String>,

        previous_text_unit: &str,

        next_text_unit: &str,
    ) -> f64 {
        trace_note!("Получаем таблицу частот биграмм для оценки вероятности перехода.");
        trace_note!(
            "`known_text_units` задаёт соответствующее входное значение или поле структуры."
        );
        trace_note!(
            "`previous_text_unit` задаёт соответствующее входное значение или поле структуры."
        );
        trace_note!("Единицу текста, которую модель обрабатывает как одно целое, называют token.");
        trace_note!("`next_text_unit` задаёт соответствующее входное значение или поле структуры.");
        trace_note!("Указываем тип возвращаемого значения.");
        trace_note!("Инициализируем изменяемый накопитель `total` начальным состоянием.");
        let mut total: usize = 0;
        trace_step!(total);
        trace_note!("Повторяем следующий блок для каждого элемента указанной последовательности.");
        for ((previous_context, _), &text_unit_count) in counts {
            trace_step!(previous_context);
            trace_step!(text_unit_count);
            trace_note!("Проверяем условие и выбираем соответствующую ветку алгоритма.");
            if previous_context == previous_text_unit {
                trace_note!("Прибавляем очередной вклад к ранее накопленному результату.");
                total += text_unit_count;
                trace_step!(total);
            }
        }
        trace_note!("Составляем результат из вычисленных значений в указанном порядке.");
        trace_note!("Ищем сохранённую частоту указанной пары слов.");
        trace_note!("При отсутствии значения используем запасной вариант.");
        trace_note!("Складываем или вычитаем величины согласно используемой формуле.");
        trace_note!("Делим значения, получая нормированную величину или среднее.");
        (*counts
            .get(&(previous_text_unit.into(), next_text_unit.into()))
            .unwrap_or(&0) as f64
            + 1.)
            / (total + known_text_units.len()) as f64
    }

    trace_note!(
        "Объявляем повторно используемое вычисление `calculate_perplexity_as_e_to_average_negative_log_next_word_probability`; параметры ниже задают его входы."
    );
    /// Перплексия: e в степени среднего отрицательного логарифма вероятности следующего слова, включая конец строки.
    fn calculate_perplexity_as_e_to_average_negative_log_next_word_probability(
        sentences: &[&str],

        counts: &std::collections::BTreeMap<(String, String), usize>,

        known_text_units: &std::collections::BTreeSet<String>,
    ) -> f64 {
        trace_note!("`sentences` задаёт соответствующее входное значение или поле структуры.");
        trace_note!("Получаем таблицу частот биграмм для оценки вероятности перехода.");
        trace_note!(
            "`known_text_units` задаёт соответствующее входное значение или поле структуры."
        );
        trace_note!("Указываем тип возвращаемого значения.");
        trace_note!(
            "Сохраняем рассчитанное значение `(mut token_count, mut negative_log_likelihood)` для следующих операций."
        );
        let (mut text_unit_count, mut negative_log_likelihood): (i32, f64) = (0, 0.);
        trace_step!(text_unit_count);
        trace_step!(negative_log_likelihood);
        trace_note!("Повторяем следующий блок для каждого элемента указанной последовательности.");
        for sentence in sentences {
            trace_step!(sentence);
            trace_note!("Создаём изменяемое значение `previous_text_unit` для следующих операций.");
            let mut previous_text_unit: &str = "<s>";
            trace_step!(previous_text_unit);
            trace_note!(
                "Повторяем следующий блок для каждого элемента указанной последовательности."
            );
            for word in sentence.split_whitespace().chain(["</s>"]) {
                trace_step!(word);
                trace_note!("Вычитаем очередной вклад из текущего значения параметра.");
                negative_log_likelihood -= (|| -> f64 {
                    trace_note!("Обновляем значение результатом текущего вычисления.");
                    trace_note!("ln(x) через ряд 2 * (t + t³/3 + t⁵/5 + ...), t=(x-1)/(x+1).");
                    trace_note!("Сохраняем результат этого шага в `value`.");
                    trace_note!(
                        "Используем ранее рассчитанное значение `counts` в текущем выражении."
                    );
                    trace_note!(
                        "Используем ранее рассчитанное значение `known_text_units` в текущем выражении."
                    );
                    trace_note!(
                        "Используем ранее рассчитанное значение `previous_text_unit` в текущем выражении."
                    );
                    trace_note!(
                        "Используем ранее рассчитанное значение `word` в текущем выражении."
                    );
                    let value: f64 =
                        calculate_next_token_probability_as_pair_count_plus_one_over_context_count_plus_vocabulary_size(

                            counts,

                            known_text_units,

                            previous_text_unit,

                            word,
                        );
                    trace_step!(value);
                    trace_note!("Проверяем обязательное условие до дальнейшего вычисления.");
                    trace_note!("Передаём очередное значение в составе результата или вызова.");
                    trace_note!(
                        "Подставляем результаты в этот шаблон вывода или текстового значения."
                    );
                    assert!(
                        value > 0.0,
                        "логарифм определён только для положительных чисел"
                    );
                    trace_note!("Проверяем условие и выбираем соответствующую ветку алгоритма.");
                    if value == f64::INFINITY {
                        trace_note!("Завершаем текущий расчёт и возвращаем найденное значение.");
                        return f64::INFINITY;
                    }
                    trace_note!("Создаём изменяемое значение `scaled` для следующих операций.");
                    let mut scaled: f64 = value;
                    trace_step!(scaled);
                    trace_note!(
                        "Инициализируем изменяемый накопитель `power_of_two` начальным состоянием."
                    );
                    let mut power_of_two: i32 = 0i32;
                    trace_step!(power_of_two);
                    trace_note!("Повторяем вычисление, пока выполняется указанное условие.");
                    while scaled >= 2.0 {
                        trace_note!("Масштабируем текущую величину делением.");
                        scaled /= 2.0;
                        trace_step!(scaled);
                        trace_note!("Прибавляем очередной вклад к ранее накопленному результату.");
                        power_of_two += 1;
                        trace_step!(power_of_two);
                    }
                    trace_note!("Повторяем вычисление, пока выполняется указанное условие.");
                    while scaled < 1.0 {
                        trace_note!("Умножаем накопленное значение на очередной множитель.");
                        scaled *= 2.0;
                        trace_step!(scaled);
                        trace_note!("Вычитаем очередной вклад из текущего значения параметра.");
                        power_of_two -= 1;
                        trace_step!(power_of_two);
                    }
                    trace_note!(
                        "Этот ряд — учебное раскрытие `value.ln()`; он может работать медленнее и отличаться по точности."
                    );
                    trace_note!(
                        "Объявляем повторно используемое вычисление `approximate_natural_log_as_twice_sum_of_odd_ratio_powers_over_odd_numbers`; параметры ниже задают его входы."
                    );
                    /// Ряд для ln(x): 2·(t + t³/3 + t⁵/5 + …), где t = (x−1)/(x+1).
                    fn approximate_natural_log_as_twice_sum_of_odd_ratio_powers_over_odd_numbers(
                        value: f64,
                    ) -> f64 {
                        trace_note!(
                            "Нормируем или усредняем величину делением и сохраняем её в `ratio`."
                        );
                        let ratio: f64 = (value - 1.0) / (value + 1.0);
                        trace_step!(ratio);
                        trace_note!("Умножаем значения и сохраняем результат в `ratio_squared`.");
                        let ratio_squared: f64 = ratio * ratio;
                        trace_step!(ratio_squared);
                        trace_note!("Создаём изменяемое значение `term` для следующих операций.");
                        let mut term: f64 = ratio;
                        trace_step!(term);
                        trace_note!(
                            "Инициализируем изменяемый накопитель `result` начальным состоянием."
                        );
                        let mut result: f64 = 0.0;
                        trace_step!(result);
                        trace_note!(
                            "Используем 40 первых членов ряда ln(value) = 2·Σ ratio^(2k+1)/(2k+1)."
                        );
                        trace_note!(
                            "Это конечное приближение: для положительного value выполняется |ratio| < 1."
                        );
                        for term_index in 0..40 {
                            trace_step!(term_index);
                            trace_note!(
                                "Прибавляем очередной вклад к ранее накопленному результату."
                            );
                            result += term / (2 * term_index + 1) as f64;
                            trace_step!(result);
                            trace_note!("Умножаем накопленное значение на очередной множитель.");
                            term *= ratio_squared;
                            trace_step!(term);
                        }
                        trace_note!("Умножаем величины согласно используемой формуле.");
                        2.0 * result
                    }
                    trace_note!(
                        "Сохраняем рассчитанное значение `logarithm_of_two` для следующих операций."
                    );
                    let logarithm_of_two: f64 =
                        approximate_natural_log_as_twice_sum_of_odd_ratio_powers_over_odd_numbers(
                            2.0,
                        );
                    trace_step!(logarithm_of_two);
                    trace_note!("Умножаем величины согласно используемой формуле.");
                    approximate_natural_log_as_twice_sum_of_odd_ratio_powers_over_odd_numbers(
                        scaled,
                    ) + power_of_two as f64 * logarithm_of_two
                })();
                trace_step!(negative_log_likelihood);
                trace_note!("Прибавляем очередной вклад к ранее накопленному результату.");
                text_unit_count += 1;
                trace_step!(text_unit_count);
                trace_note!("Обновляем `previous_text_unit` результатом текущего шага.");
                previous_text_unit = word;
                trace_step!(previous_text_unit);
            }
        }
        trace_note!("Делим значения, получая нормированную величину или среднее.");
        approximate_e_to_power_by_summing_power_over_factorial_terms(
            negative_log_likelihood / text_unit_count as f64,
        )
    }

    trace_note!("Шаг: Сравниваем perplexity на обучающих и новых сочетаниях слов.");
    trace_note!("Задаём шаблон строки: плейсхолдеры ниже заменятся рассчитанными значениями.");
    trace_note!("Вызываем нужное вычисление с подготовленными аргументами.");
    trace_note!("Вызываем нужное вычисление с подготовленными аргументами.");
    println!(
        "train perplexity={:.3}, validation perplexity={:.3}",
        calculate_perplexity_as_e_to_average_negative_log_next_word_probability(
            &training_sentences,
            &bigram_counts,
            &known_text_units
        ),
        calculate_perplexity_as_e_to_average_negative_log_next_word_probability(
            &["пёс ест"],
            &bigram_counts,
            &known_text_units
        )
    );
    trace_note!("Шаг: Генерируем цепочку, каждый раз выбирая наиболее вероятный следующий токен.");
    let mut previous_text_unit: &str = "<s>";
    trace_step!(previous_text_unit);
    trace_note!("Создаём набор значений `generated_text_units` для следующего шага примера.");
    let mut generated_text_units: Vec<&str> = vec![];
    trace_step!(generated_text_units);
    trace_note!(
        "Генерируем ровно пять следующих токенов, чтобы показать короткое продолжение фразы."
    );
    for _ in 0..5 {
        trace_note!("Сохраняем рассчитанное значение `next_text_unit` для следующих операций.");
        trace_note!("Перебираем элементы по ссылке, не копируя исходную коллекцию.");
        trace_note!("Сравниваем кандидатов и оставляем наибольший результат.");
        trace_note!("Извлекаем значение: выше в примере обеспечено отсутствие ошибки.");
        let next_text_unit: &String = known_text_units

            .iter()

            .max_by(|first_candidate, second_candidate| {
                trace_note!("Вызываем нужное вычисление с подготовленными аргументами.");
                trace_note!("Передаём данные по ссылке или разыменовываем их для следующей операции.");
                trace_note!("Передаём данные по ссылке или разыменовываем их для следующей операции.");
                trace_note!("Используем ранее рассчитанное значение `previous_text_unit` в текущем выражении.");
                trace_note!("Используем ранее рассчитанное значение `first_candidate` в текущем выражении.");
                trace_note!("Сравниваем числа с полным порядком, включая специальные значения.");
                trace_note!("Передаём данные по ссылке или разыменовываем их для следующей операции.");
                trace_note!("Передаём данные по ссылке или разыменовываем их для следующей операции.");
                trace_note!("Используем ранее рассчитанное значение `previous_text_unit` в текущем выражении.");
                trace_note!("Используем ранее рассчитанное значение `second_candidate` в текущем выражении.");
                calculate_next_token_probability_as_pair_count_plus_one_over_context_count_plus_vocabulary_size(

                    &bigram_counts,

                    &known_text_units,

                    previous_text_unit,

                    first_candidate,
                )

                .total_cmp(
                    &calculate_next_token_probability_as_pair_count_plus_one_over_context_count_plus_vocabulary_size(

                        &bigram_counts,

                        &known_text_units,

                        previous_text_unit,

                        second_candidate,
                    ),
                )
            })

            .unwrap();
        trace_step!(next_text_unit);
        trace_note!("Проверяем условие и выбираем соответствующую ветку алгоритма.");
        if next_text_unit == "</s>" {
            trace_note!("Останавливаем цикл после достижения условия завершения.");
            break;
        }
        trace_note!("Сохраняем очередной рассчитанный элемент в коллекции.");
        generated_text_units.push(next_text_unit.as_str());
        trace_note!("Обновляем `previous_text_unit` результатом текущего шага.");
        previous_text_unit = next_text_unit;
        trace_step!(previous_text_unit);
    }
    trace_note!("Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением.");
    println!("generated: {}", generated_text_units.join(" "));

    trace_note!("Построение графика вынесено из основного кода урока.");
    disable();
    plot_next_word_counts_after_start_of_sentence(bigram_counts);
}

// Строим график по результатам урока.
fn plot_next_word_counts_after_start_of_sentence(
    bigram_counts: std::collections::BTreeMap<(std::string::String, std::string::String), usize>,
) {
    trace_note!("Наглядное представление вычислений сводной практики.");
    trace_note!("Просматриваем элементы коллекции по ссылке.");
    trace_note!("Оставляем элементы, отвечающие условию.");
    trace_note!("Добавляем порядковый номер к каждому элементу.");
    trace_note!("Преобразуем каждый элемент в новое значение.");
    trace_note!("Собираем результаты в коллекцию.");
    let language_model_points: Vec<(f64, f64)> = bigram_counts
        .iter()
        .filter(|((previous, _), _)| previous == "<s>")
        .enumerate()
        .map(|(item_index, (_, count))| (item_index as f64, *count as f64))
        .collect();
    trace_note!("Строим график по рассчитанным значениям и сохраняем его как SVG.");
    trace_note!("Передаём путь к каталогу текущего урока.");
    trace_note!("Указываем имя SVG-файла.");
    trace_note!("Указываем заголовок графика.");
    trace_note!("Указываем подпись горизонтальной оси.");
    trace_note!("Указываем подпись вертикальной оси.");
    trace_note!("Передаём ряды или значения для отрисовки графика.");
    trace_note!("Указываем подпись этого ряда в легенде.");
    trace_note!("Передаём рассчитанные координаты точек.");
    trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
    let chart: std::path::PathBuf = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Частоты переходов после начала строки",
        "номер следующего токена",
        "частота",
        &[lesson_visualization::Series {
            name: "биграммы",

            points: &language_model_points,
        }],
    )
    .expect("не удалось сохранить график");
    trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
