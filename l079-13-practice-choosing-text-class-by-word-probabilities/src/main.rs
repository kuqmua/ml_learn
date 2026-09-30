// Урок 13.4. Практика: выбор класса текста по вероятностям слов.
// Связь с принятой терминологией: Наивный Байес с априорными вероятностями и сглаживанием.
// Зачем здесь эта тема: Классификатор Байеса должен соединить prior, условные вероятности и
//   сглаживание.
// Почему код устроен так: Складываем логарифмы вместо умножения малых вероятностей и проверяем
//   неизвестное слово.
// Представь: Много маленьких вероятностей трудно перемножать; сумма их логарифмов сохраняет порядок
//   сравнения классов.
//
// Что повторяем вместе: условная независимость, априорные вероятности, сглаживание Лапласа.
// Зачем это нужно: Наивный Байес сравнивает вероятности слов в классах; логарифмы превращают результат умножения
//   малых чисел в сумму.
// Что показывает программа: Составляем маленький размеченный корпус положительных и отрицательных текстов.
//   Считаем логарифмические оценки классов для известных слов. Отдельно проверяем, что неизвестное слово не
//   ломает классификатор.
// Что проверить при изменении примера: Проверь неизвестные слова и класс с редкими токенами; считай
//   вероятности в log-пространстве.
// Дополнительная практика: Обучи мультиномиальный классификатор коротких текстов по счётчикам слов.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    lesson_trace::enable();
    lesson_trace::trace_note!(
        "Шаг: Составляем маленький размеченный корпус положительных и отрицательных текстов."
    );
    lesson_trace::trace_note!("Составляем результат из вычисленных значений в указанном порядке.");
    lesson_trace::trace_note!("Составляем результат из вычисленных значений в указанном порядке.");
    lesson_trace::trace_note!("Составляем результат из вычисленных значений в указанном порядке.");
    lesson_trace::trace_note!("Составляем результат из вычисленных значений в указанном порядке.");
    let training_examples: [(&str, bool); 4] = [
        ("хороший фильм", true),
        ("отличный фильм", true),
        ("плохой фильм", false),
        ("ужасный фильм", false),
    ];
    lesson_trace::trace_step!(training_examples);

    lesson_trace::trace_note!("Учебные реализации математических операций для этого урока.");

    /// ln(x) через ряд 2 * (t + t³/3 + t⁵/5 + ...), t=(x-1)/(x+1).
    /// Учебный аналог `f64::ln`; показывает вычисление ряда и может работать медленнее.
    /// Здесь неположительный вход вызывает panic, а `ln` возвращает NaN или −∞.
    /// Натуральный логарифм: приводим аргумент к [1, 2), суммируем ряд нечётных степеней и возвращаем вклад степеней двойки.
    fn approximate_natural_log_by_scaling_and_summing_odd_powers(value: f64) -> f64 {
        lesson_trace::trace_note!("Проверяем обязательное условие до дальнейшего вычисления.");
        lesson_trace::trace_note!("Передаём очередное значение в составе результата или вызова.");
        lesson_trace::trace_note!(
            "Подставляем результаты в этот шаблон вывода или текстового значения."
        );
        assert!(
            value > 0.0,
            "логарифм определён только для положительных чисел"
        );
        lesson_trace::trace_note!("Проверяем условие и выбираем соответствующую ветку алгоритма.");
        if value == f64::INFINITY {
            lesson_trace::trace_note!("Завершаем текущий расчёт и возвращаем найденное значение.");
            return f64::INFINITY;
        }
        lesson_trace::trace_note!("Создаём изменяемое значение `scaled` для следующих операций.");
        let mut scaled: f64 = value;
        lesson_trace::trace_step!(scaled);
        lesson_trace::trace_note!(
            "Инициализируем изменяемый накопитель `power_of_two` начальным состоянием."
        );
        let mut power_of_two: i32 = 0i32;
        lesson_trace::trace_step!(power_of_two);
        lesson_trace::trace_note!("Повторяем вычисление, пока выполняется указанное условие.");
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
        lesson_trace::trace_note!("Повторяем вычисление, пока выполняется указанное условие.");
        while scaled < 1.0 {
            lesson_trace::trace_note!("Умножаем накопленное значение на очередной множитель.");
            scaled *= 2.0;
            lesson_trace::trace_step!(scaled);
            lesson_trace::trace_note!("Вычитаем очередной вклад из текущего значения параметра.");
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
            lesson_trace::trace_note!("Умножаем значения и сохраняем результат в `ratio_squared`.");
            let ratio_squared: f64 = ratio * ratio;
            lesson_trace::trace_step!(ratio_squared);
            lesson_trace::trace_note!("Создаём изменяемое значение `term` для следующих операций.");
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
                lesson_trace::trace_note!("Умножаем накопленное значение на очередной множитель.");
                term *= ratio_squared;
                lesson_trace::trace_step!(term);
            }
            lesson_trace::trace_note!("Умножаем величины согласно используемой формуле.");
            2.0 * result
        }
        lesson_trace::trace_note!(
            "Сохраняем рассчитанное значение `logarithm_of_two` для следующих операций."
        );
        let logarithm_of_two: f64 =
            approximate_natural_log_as_twice_sum_of_odd_ratio_powers_over_odd_numbers(2.0);
        lesson_trace::trace_step!(logarithm_of_two);
        lesson_trace::trace_note!("Умножаем величины согласно используемой формуле.");
        approximate_natural_log_as_twice_sum_of_odd_ratio_powers_over_odd_numbers(scaled)
            + power_of_two as f64 * logarithm_of_two
    }

    lesson_trace::trace_note!("Суммируем логарифмы априорной вероятности и вероятностей токенов.");
    /// Наивный Байес: для каждого класса складываем логарифм его вероятности и логарифмы вероятностей слов со сглаживанием +1.
    fn choose_class_by_summed_log_probabilities_of_words(
        training_examples: &[(&str, bool)],

        text: &str,
    ) -> (bool, [f64; 2]) {
        lesson_trace::trace_note!(
            "Получаем размеченные обучающие примеры по ссылке без копирования."
        );
        lesson_trace::trace_note!(
            "`text` задаёт соответствующее входное значение или поле структуры."
        );
        lesson_trace::trace_note!("Указываем тип возвращаемого значения.");
        lesson_trace::trace_note!("Словарь задаёт все слова, встреченные во время обучения.");
        lesson_trace::trace_note!("Набор известных модели текстовых единиц называют vocabulary.");
        lesson_trace::trace_note!("Перебираем элементы по ссылке, не копируя исходную коллекцию.");
        lesson_trace::trace_note!("Разделяем текст по пробельным символам на отдельные слова.");
        lesson_trace::trace_note!("Собираем элементы итератора в итоговую коллекцию.");
        let known_text_units: std::collections::HashSet<&str> = training_examples
            .iter()
            .flat_map(|(document_text, _)| document_text.split_whitespace())
            .collect();
        lesson_trace::trace_step!(known_text_units);
        lesson_trace::trace_note!("Создаём набор значений `scores` для следующего шага примера.");
        let mut scores: [f64; 2] = [0.; 2];
        lesson_trace::trace_step!(scores);
        lesson_trace::trace_note!("Для каждого класса собираем частоты слов отдельно.");
        for (class, score) in scores.iter_mut().enumerate() {
            lesson_trace::trace_step!(class);
            lesson_trace::trace_step!(score);
            lesson_trace::trace_note!(
                "Сохраняем рассчитанное значение `class_documents` для следующих операций."
            );
            lesson_trace::trace_note!(
                "Перебираем элементы по ссылке, не копируя исходную коллекцию."
            );
            lesson_trace::trace_note!("Оставляем только элементы, прошедшие указанную проверку.");
            lesson_trace::trace_note!("Собираем элементы итератора в итоговую коллекцию.");
            let class_documents: Vec<&(&str, bool)> = training_examples
                .iter()
                .filter(|(_, label)| *label == (class == 1))
                .collect();
            lesson_trace::trace_step!(class_documents);
            lesson_trace::trace_note!(
                "Инициализируем изменяемый накопитель `text_unit_counts` начальным состоянием."
            );
            lesson_trace::trace_note!(
                "Единицу текста, которую модель обрабатывает как одно целое, называют token."
            );
            let mut text_unit_counts: std::collections::HashMap<&str, usize> =
                std::collections::HashMap::new();
            lesson_trace::trace_step!(text_unit_counts);
            lesson_trace::trace_note!(
                "Инициализируем изменяемый накопитель `total_text_units` начальным состоянием."
            );
            let mut total_text_units: usize = 0;
            lesson_trace::trace_step!(total_text_units);
            lesson_trace::trace_note!(
                "Повторяем следующий блок для каждого элемента указанной последовательности."
            );
            for (document_text, _) in &class_documents {
                lesson_trace::trace_step!(document_text);
                lesson_trace::trace_note!(
                    "Повторяем следующий блок для каждого элемента указанной последовательности."
                );
                for text_unit in document_text.split_whitespace() {
                    lesson_trace::trace_step!(text_unit);
                    lesson_trace::trace_note!(
                        "Прибавляем очередной вклад к ранее накопленному результату."
                    );
                    *text_unit_counts.entry(text_unit).or_insert(0usize) += 1;
                    lesson_trace::trace_step!(text_unit_counts);
                    lesson_trace::trace_note!(
                        "Прибавляем очередной вклад к ранее накопленному результату."
                    );
                    total_text_units += 1;
                    lesson_trace::trace_step!(total_text_units);
                }
            }
            lesson_trace::trace_note!(
                "Начинаем с априорной вероятности класса и добавляем логарифмы вероятностей слов."
            );
            lesson_trace::trace_note!(
                "Составляем результат из вычисленных значений в указанном порядке."
            );
            *score = approximate_natural_log_by_scaling_and_summing_odd_powers(
                (class_documents.len() as f64 + 1.) / (training_examples.len() as f64 + 2.),
            );
            lesson_trace::trace_step!(score);
            lesson_trace::trace_note!(
                "Повторяем следующий блок для каждого элемента указанной последовательности."
            );
            for text_unit in text.split_whitespace() {
                lesson_trace::trace_step!(text_unit);
                lesson_trace::trace_note!(
                    "Проверяем условие и выбираем соответствующую ветку алгоритма."
                );
                if known_text_units.contains(text_unit) {
                    lesson_trace::trace_note!(
                        "Прибавляем очередной вклад к ранее накопленному результату."
                    );
                    lesson_trace::trace_note!(
                        "Составляем результат из вычисленных значений в указанном порядке."
                    );
                    lesson_trace::trace_note!(
                        "Делим значения, получая нормированную величину или среднее."
                    );
                    *score += approximate_natural_log_by_scaling_and_summing_odd_powers(
                        (*text_unit_counts.get(text_unit).unwrap_or(&0) as f64 + 1.)
                            / (total_text_units + known_text_units.len()) as f64,
                    );
                    lesson_trace::trace_step!(score);
                }
            }
        }
        lesson_trace::trace_note!(
            "Составляем результат из вычисленных значений в указанном порядке."
        );
        (scores[1] > scores[0], scores)
    }

    lesson_trace::trace_note!("Шаг: Считаем логарифмические оценки классов для известных слов.");
    lesson_trace::trace_note!(
        "Подставляем результаты в этот шаблон вывода или текстового значения."
    );
    lesson_trace::trace_note!("Вызываем нужное вычисление с подготовленными аргументами.");
    println!(
        "{:?}",
        choose_class_by_summed_log_probabilities_of_words(&training_examples, "хороший отличный")
    );
    lesson_trace::trace_note!(
        "Шаг: Отдельно проверяем, что неизвестное слово не ломает классификатор."
    );
    lesson_trace::trace_note!(
        "Подставляем результаты в этот шаблон вывода или текстового значения."
    );
    lesson_trace::trace_note!("Вызываем нужное вычисление с подготовленными аргументами.");
    println!(
        "unknown: {:?}",
        choose_class_by_summed_log_probabilities_of_words(&training_examples, "неизвестное")
    );

    lesson_trace::trace_note!("Построение графика вынесено из основного кода урока.");
    lesson_trace::disable();
    plot_number_of_training_documents_in_each_class(training_examples);
}

// Строим график по результатам урока.
fn plot_number_of_training_documents_in_each_class(training_examples: [(&str, bool); 4]) {
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
    lesson_trace::trace_note!("Просматриваем элементы коллекции по ссылке.");
    lesson_trace::trace_note!("Оставляем элементы, отвечающие условию.");
    lesson_trace::trace_note!("Подсчитываем число подходящих элементов.");
    lesson_trace::trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
    let chart: std::path::PathBuf = lesson_visualization::bar_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Классы обучающих документов",
        "число текстов",
        &[
            (
                "положительные",
                training_examples.iter().filter(|(_, class)| *class).count() as f64,
            ),
            (
                "отрицательные",
                training_examples
                    .iter()
                    .filter(|(_, class)| !*class)
                    .count() as f64,
            ),
        ],
    )
    .expect("не удалось сохранить график");
    lesson_trace::trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
