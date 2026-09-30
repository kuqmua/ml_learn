// Урок 47.1. Итоговый проект: сравнение способов решения задачи и разбор ошибок.
// Связь с принятой терминологией: Итоговый AI проект со сравнением подходов и анализом ошибок.
// Зачем здесь эта тема: Итоговый AI-проект требует сравнивать подход с baseline и разбирать ошибки
//   на данных вне обучения.
// Почему код устроен так: Сравниваем простую базовую модель с более сложным методом на общих
//   примерах и разбираем ошибки.
// Представь: Если простой прогноз по большинству классов не хуже текстового классификатора,
//   сложность пока не оправдана.
//
// Что применяем: самостоятельный дизайн, сравнение подходов, анализ ошибок, презентация результата.
// Зачем это нужно: Итоговый проект соединяет простую текстовую модель с baseline, отдельной оценкой и
//   разбором прогнозов.
// Что показывает программа: Считаем частоту большинства как простую исходную точку. Оцениваем классификатор
//   на фразах вне обучения. Разбираем каждый прогноз вместе с истинной темой и числом совпавших слов.
// Что проверить при изменении примера: Из чистого checkout выполняются подготовка данных, обучение, оценка
//   и инференс; выводы подтверждены метриками.
// Возможное расширение: Сделай прикладной проект: классификатор, поиск по документам или небольшой
//   генератор; обоснуй выбор и оформи README.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    lesson_trace::enable();
    lesson_trace::trace_note!(
        "Мини-проект: классификация запроса по словам, оценка на отложенных фразах."
    );

    lesson_trace::trace_note!("Фиксируем демонстрационные данные на время выполнения программы.");
    lesson_trace::trace_note!("Составляем результат из вычисленных значений в указанном порядке.");
    lesson_trace::trace_note!("Составляем результат из вычисленных значений в указанном порядке.");
    lesson_trace::trace_note!("Составляем результат из вычисленных значений в указанном порядке.");
    lesson_trace::trace_note!("Составляем результат из вычисленных значений в указанном порядке.");
    lesson_trace::trace_note!("Составляем результат из вычисленных значений в указанном порядке.");
    lesson_trace::trace_note!("Составляем результат из вычисленных значений в указанном порядке.");
    const TRAINING_EXAMPLES: [(&str, &str); 6] = [
        ("ошибка компиляции rust", "code"),
        ("cargo не собирает проект", "code"),
        ("заимствование и владение", "code"),
        ("градиент модели", "ml"),
        ("обучение нейросети", "ml"),
        ("признаки и метрики", "ml"),
    ];

    lesson_trace::trace_note!("Проверяем ожидаемое свойство учебного примера.");
    lesson_trace::trace_note!("Используем подготовленное значение в следующем шаге примера.");
    lesson_trace::trace_note!("Передаём подпись или текстовое значение для следующего шага.");
    assert!(
        !TRAINING_EXAMPLES.is_empty(),
        "для baseline нужны обучающие примеры"
    );
    lesson_trace::trace_note!("Шаг: Считаем частоту большинства как простую исходную точку.");
    lesson_trace::trace_note!("Перебираем элементы по ссылке, не копируя исходную коллекцию.");
    lesson_trace::trace_note!("Оставляем только элементы, прошедшие указанную проверку.");
    lesson_trace::trace_note!("Подсчитываем число элементов после отбора.");
    lesson_trace::trace_note!("Делим значения, получая нормированную величину или среднее.");
    let baseline: f64 = TRAINING_EXAMPLES
        .iter()
        .filter(|(_, expected_topic)| *expected_topic == "code")
        .count() as f64
        / TRAINING_EXAMPLES.len() as f64;
    lesson_trace::trace_step!(baseline);
    lesson_trace::trace_note!("Фиксируем демонстрационные данные на время выполнения программы.");
    const TEST_EXAMPLES: [(&str, &str); 2] = [("ошибка cargo", "code"), ("метрики модели", "ml")];
    lesson_trace::trace_note!("Проверяем ожидаемое свойство учебного примера.");
    lesson_trace::trace_note!("Используем подготовленное значение в следующем шаге примера.");
    lesson_trace::trace_note!("Передаём подпись или текстовое значение для следующего шага.");
    assert!(
        !TEST_EXAMPLES.is_empty(),
        "для оценки нужны тестовые примеры"
    );

    lesson_trace::trace_note!(
        "Объявляем повторно используемое вычисление `collect_unique_words_from_text`; параметры ниже задают его входы."
    );
    fn collect_unique_words_from_text(sample_text: &str) -> std::collections::BTreeSet<&str> {
        lesson_trace::trace_note!("Разделяем текст по пробельным символам на отдельные слова.");
        sample_text.split_whitespace().collect()
    }
    lesson_trace::trace_note!("Суммируем общие слова запроса с обучающими фразами каждой темы.");
    fn choose_topic_and_count_matching_training_words(
        query: &str,

        training_examples: &[(&str, &str)],
    ) -> (&'static str, usize) {
        lesson_trace::trace_note!(
            "`query` задаёт соответствующее входное значение или поле структуры."
        );
        lesson_trace::trace_note!(
            "Получаем размеченные обучающие примеры по ссылке без копирования."
        );
        lesson_trace::trace_note!("Указываем тип возвращаемого значения.");
        lesson_trace::trace_note!(
            "Выделяем уникальные слова запроса для сравнения с обучающими фразами."
        );
        lesson_trace::trace_note!(
            "Единицу текста, которую модель обрабатывает как одно целое, называют token."
        );
        let query_text_units: std::collections::BTreeSet<&str> =
            collect_unique_words_from_text(query);
        lesson_trace::trace_step!(query_text_units);
        lesson_trace::trace_note!("Создаём изменяемое значение `scores` для следующих операций.");
        let mut scores: std::collections::BTreeMap<&str, usize> =
            std::collections::BTreeMap::from([("code", 0usize), ("ml", 0)]);
        lesson_trace::trace_step!(scores);
        lesson_trace::trace_note!(
            "Повторяем следующий блок для каждого элемента указанной последовательности."
        );
        for &(sample_text, label) in training_examples {
            lesson_trace::trace_step!(sample_text);
            lesson_trace::trace_step!(label);
            lesson_trace::trace_note!(
                "Прибавляем очередной вклад к ранее накопленному результату."
            );
            lesson_trace::trace_note!("Находим общие слова запроса и обучающего текста.");
            lesson_trace::trace_note!("Подсчитываем число элементов после отбора.");
            *scores.get_mut(label).unwrap() += query_text_units
                .intersection(&collect_unique_words_from_text(sample_text))
                .count();
            lesson_trace::trace_step!(scores);
        }
        lesson_trace::trace_note!(
            "Сохраняем рассчитанное значение `code_score` для следующих операций."
        );
        let code_score: usize = scores["code"];
        lesson_trace::trace_step!(code_score);
        lesson_trace::trace_note!(
            "Сохраняем рассчитанное значение `machine_learning_score` для следующих операций."
        );
        let machine_learning_score: usize = scores["ml"];
        lesson_trace::trace_step!(machine_learning_score);
        lesson_trace::trace_note!(
            "Составляем результат из вычисленных значений в указанном порядке."
        );
        lesson_trace::trace_note!("Проверяем условие и выбираем соответствующую ветку алгоритма.");
        lesson_trace::trace_note!("Проверяем условие и выбираем соответствующую ветку алгоритма.");
        (
            if machine_learning_score > code_score {
                lesson_trace::trace_note!(
                    "Подставляем результаты в этот шаблон вывода или текстового значения."
                );
                "ml"
            } else {
                lesson_trace::trace_note!(
                    "Обрабатываем случай, когда предыдущее условие не выполнено."
                );
                lesson_trace::trace_note!(
                    "Подставляем результаты в этот шаблон вывода или текстового значения."
                );
                "code"
            },
            if code_score > machine_learning_score {
                lesson_trace::trace_note!(
                    "Используем ранее рассчитанное значение `code_score` в текущем выражении."
                );
                code_score
            } else {
                lesson_trace::trace_note!(
                    "Обрабатываем случай, когда предыдущее условие не выполнено."
                );
                lesson_trace::trace_note!(
                    "Используем ранее рассчитанное значение `machine_learning_score` в текущем выражении."
                );
                machine_learning_score
            },
        )
    }

    lesson_trace::trace_note!("Шаг: Оцениваем классификатор на фразах вне обучения.");
    lesson_trace::trace_note!("Перебираем элементы по ссылке, не копируя исходную коллекцию.");
    lesson_trace::trace_note!("Оставляем только элементы, прошедшие указанную проверку.");
    lesson_trace::trace_note!("Подсчитываем число элементов после отбора.");
    let correct: usize = TEST_EXAMPLES
        .iter()
        .filter(|&&(query_text, expected_topic)| {
            lesson_trace::trace_note!("Вызываем нужное вычисление с подготовленными аргументами.");
            lesson_trace::trace_note!("Проверяем логическое условие для текущих элементов.");
            choose_topic_and_count_matching_training_words(query_text, &TRAINING_EXAMPLES).0
                == expected_topic
        })
        .count();
    lesson_trace::trace_step!(correct);
    lesson_trace::trace_note!(
        "Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением."
    );
    lesson_trace::trace_note!(
        "Задаём шаблон строки: плейсхолдеры ниже заменятся рассчитанными значениями."
    );
    lesson_trace::trace_note!("Делим значения, получая нормированную величину или среднее.");
    println!(
        "majority baseline train={baseline:.2}; held-out accuracy={:.2}",
        correct as f64 / TEST_EXAMPLES.len() as f64
    );
    lesson_trace::trace_note!(
        "Шаг: Разбираем каждый прогноз вместе с истинной темой и числом совпавших слов."
    );
    for &(sample_text, expected_topic) in &TEST_EXAMPLES {
        lesson_trace::trace_step!(sample_text);
        lesson_trace::trace_step!(expected_topic);
        lesson_trace::trace_note!(
            "Сохраняем рассчитанное значение `(predicted_topic, overlap_count)` для следующих операций."
        );
        lesson_trace::trace_note!("Вызываем нужное вычисление с подготовленными аргументами.");
        let (predicted_topic, overlap_count): (&str, usize) =
            choose_topic_and_count_matching_training_words(sample_text, &TRAINING_EXAMPLES);
        lesson_trace::trace_step!(predicted_topic);
        lesson_trace::trace_step!(overlap_count);
        lesson_trace::trace_note!(
            "Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением."
        );
        lesson_trace::trace_note!(
            "Задаём шаблон строки: плейсхолдеры ниже заменятся рассчитанными значениями."
        );
        println!(
            "query={sample_text:?}, predicted={predicted_topic}, actual={expected_topic}, overlap={overlap_count}"
        );
    }

    lesson_trace::trace_note!("Построение графика вынесено из основного кода урока.");
    lesson_trace::disable();
    plot_correct_prediction_shares_for_compared_text_classifiers(baseline, correct);

    lesson_trace::trace_note!("Строим график по результатам урока.");
    fn plot_correct_prediction_shares_for_compared_text_classifiers(baseline: f64, correct: usize) {
        lesson_trace::trace_note!("Наглядное сравнение результатов сводной практики.");
        lesson_trace::trace_note!("Передаём путь к каталогу текущего урока.");
        lesson_trace::trace_note!("Указываем имя SVG-файла.");
        lesson_trace::trace_note!("Указываем заголовок диаграммы.");
        lesson_trace::trace_note!("Указываем подпись вертикальной оси.");
        lesson_trace::trace_note!("Передаём ряды или значения для отрисовки графика.");
        lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
        lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
        lesson_trace::trace_note!(
            "Прерываем пример с понятной ошибкой, если SVG не удалось записать."
        );
        let chart: std::path::PathBuf = lesson_visualization::bar_chart(
            env!("CARGO_MANIFEST_DIR"),
            "lesson-chart",
            "Итоговый AI-проект",
            "accuracy",
            &[
                ("baseline", baseline),
                ("test", correct as f64 / TEST_EXAMPLES.len() as f64),
            ],
        )
        .expect("не удалось сохранить график");
        lesson_trace::trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
        println!("график: {}", chart.display());
    }
}
