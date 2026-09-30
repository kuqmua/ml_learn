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
use lesson_trace::{disable, enable_tracing, trace_note, trace_step};

fn main() {
    enable_tracing();
    trace_note!("Мини-проект: классификация запроса по словам, оценка на отложенных фразах.");

    trace_note!("Фиксируем демонстрационные данные на время выполнения программы.");
    trace_note!("Составляем результат из вычисленных значений в указанном порядке.");
    trace_note!("Составляем результат из вычисленных значений в указанном порядке.");
    trace_note!("Составляем результат из вычисленных значений в указанном порядке.");
    trace_note!("Составляем результат из вычисленных значений в указанном порядке.");
    trace_note!("Составляем результат из вычисленных значений в указанном порядке.");
    trace_note!("Составляем результат из вычисленных значений в указанном порядке.");
    const TRAINING_EXAMPLES: [(&str, &str); 6] = [
        ("ошибка компиляции rust", "code"),
        ("cargo не собирает проект", "code"),
        ("заимствование и владение", "code"),
        ("градиент модели", "ml"),
        ("обучение нейросети", "ml"),
        ("признаки и метрики", "ml"),
    ];

    trace_note!("Проверяем ожидаемое свойство учебного примера.");
    trace_note!("Используем подготовленное значение в следующем шаге примера.");
    trace_note!("Передаём подпись или текстовое значение для следующего шага.");
    assert!(
        !TRAINING_EXAMPLES.is_empty(),
        "для baseline нужны обучающие примеры"
    );
    trace_note!("Шаг: Считаем частоту большинства как простую исходную точку.");
    trace_note!("Перебираем элементы по ссылке, не копируя исходную коллекцию.");
    trace_note!("Оставляем только элементы, прошедшие указанную проверку.");
    trace_note!("Подсчитываем число элементов после отбора.");
    trace_note!("Делим значения, получая нормированную величину или среднее.");
    let baseline: f64 = TRAINING_EXAMPLES
        .iter()
        .filter(|(_, expected_topic)| *expected_topic == "code")
        .count() as f64
        / TRAINING_EXAMPLES.len() as f64;
    trace_step!(baseline);
    trace_note!("Фиксируем демонстрационные данные на время выполнения программы.");
    const TEST_EXAMPLES: [(&str, &str); 2] = [("ошибка cargo", "code"), ("метрики модели", "ml")];
    trace_note!("Проверяем ожидаемое свойство учебного примера.");
    trace_note!("Используем подготовленное значение в следующем шаге примера.");
    trace_note!("Передаём подпись или текстовое значение для следующего шага.");
    assert!(
        !TEST_EXAMPLES.is_empty(),
        "для оценки нужны тестовые примеры"
    );

    trace_note!(
        "Объявляем повторно используемое вычисление `collect_unique_words_from_text`; параметры ниже задают его входы."
    );
    fn collect_unique_words_from_text(sample_text: &str) -> std::collections::BTreeSet<&str> {
        trace_note!("Разделяем текст по пробельным символам на отдельные слова.");
        sample_text.split_whitespace().collect()
    }
    trace_note!("Суммируем общие слова запроса с обучающими фразами каждой темы.");
    fn choose_topic_and_count_matching_training_words(
        query: &str,

        training_examples: &[(&str, &str)],
    ) -> (&'static str, usize) {
        trace_note!("`query` задаёт соответствующее входное значение или поле структуры.");
        trace_note!("Получаем размеченные обучающие примеры по ссылке без копирования.");
        trace_note!("Указываем тип возвращаемого значения.");
        trace_note!("Выделяем уникальные слова запроса для сравнения с обучающими фразами.");
        trace_note!("Единицу текста, которую модель обрабатывает как одно целое, называют token.");
        let query_text_units: std::collections::BTreeSet<&str> =
            collect_unique_words_from_text(query);
        trace_step!(query_text_units);
        trace_note!("Создаём изменяемое значение `scores` для следующих операций.");
        let mut scores: std::collections::BTreeMap<&str, usize> =
            std::collections::BTreeMap::from([("code", 0usize), ("ml", 0)]);
        trace_step!(scores);
        trace_note!("Повторяем следующий блок для каждого элемента указанной последовательности.");
        for &(sample_text, label) in training_examples {
            trace_step!(sample_text);
            trace_step!(label);
            trace_note!("Прибавляем очередной вклад к ранее накопленному результату.");
            trace_note!("Находим общие слова запроса и обучающего текста.");
            trace_note!("Подсчитываем число элементов после отбора.");
            *scores.get_mut(label).unwrap() += query_text_units
                .intersection(&collect_unique_words_from_text(sample_text))
                .count();
            trace_step!(scores);
        }
        trace_note!("Сохраняем рассчитанное значение `code_score` для следующих операций.");
        let code_score: usize = scores["code"];
        trace_step!(code_score);
        trace_note!(
            "Сохраняем рассчитанное значение `machine_learning_score` для следующих операций."
        );
        let machine_learning_score: usize = scores["ml"];
        trace_step!(machine_learning_score);
        trace_note!("Составляем результат из вычисленных значений в указанном порядке.");
        trace_note!("Проверяем условие и выбираем соответствующую ветку алгоритма.");
        trace_note!("Проверяем условие и выбираем соответствующую ветку алгоритма.");
        (
            if machine_learning_score > code_score {
                trace_note!("Подставляем результаты в этот шаблон вывода или текстового значения.");
                "ml"
            } else {
                trace_note!("Обрабатываем случай, когда предыдущее условие не выполнено.");
                trace_note!("Подставляем результаты в этот шаблон вывода или текстового значения.");
                "code"
            },
            if code_score > machine_learning_score {
                trace_note!(
                    "Используем ранее рассчитанное значение `code_score` в текущем выражении."
                );
                code_score
            } else {
                trace_note!("Обрабатываем случай, когда предыдущее условие не выполнено.");
                trace_note!(
                    "Используем ранее рассчитанное значение `machine_learning_score` в текущем выражении."
                );
                machine_learning_score
            },
        )
    }

    trace_note!("Шаг: Оцениваем классификатор на фразах вне обучения.");
    trace_note!("Перебираем элементы по ссылке, не копируя исходную коллекцию.");
    trace_note!("Оставляем только элементы, прошедшие указанную проверку.");
    trace_note!("Подсчитываем число элементов после отбора.");
    let correct: usize = TEST_EXAMPLES
        .iter()
        .filter(|&&(query_text, expected_topic)| {
            trace_note!("Вызываем нужное вычисление с подготовленными аргументами.");
            trace_note!("Проверяем логическое условие для текущих элементов.");
            choose_topic_and_count_matching_training_words(query_text, &TRAINING_EXAMPLES).0
                == expected_topic
        })
        .count();
    trace_step!(correct);
    trace_note!("Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением.");
    trace_note!("Задаём шаблон строки: плейсхолдеры ниже заменятся рассчитанными значениями.");
    trace_note!("Делим значения, получая нормированную величину или среднее.");
    println!(
        "majority baseline train={baseline:.2}; held-out accuracy={:.2}",
        correct as f64 / TEST_EXAMPLES.len() as f64
    );
    trace_note!("Шаг: Разбираем каждый прогноз вместе с истинной темой и числом совпавших слов.");
    for &(sample_text, expected_topic) in &TEST_EXAMPLES {
        trace_step!(sample_text);
        trace_step!(expected_topic);
        trace_note!(
            "Сохраняем рассчитанное значение `(predicted_topic, overlap_count)` для следующих операций."
        );
        trace_note!("Вызываем нужное вычисление с подготовленными аргументами.");
        let (predicted_topic, overlap_count): (&str, usize) =
            choose_topic_and_count_matching_training_words(sample_text, &TRAINING_EXAMPLES);
        trace_step!(predicted_topic);
        trace_step!(overlap_count);
        trace_note!("Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением.");
        trace_note!("Задаём шаблон строки: плейсхолдеры ниже заменятся рассчитанными значениями.");
        println!(
            "query={sample_text:?}, predicted={predicted_topic}, actual={expected_topic}, overlap={overlap_count}"
        );
    }

    trace_note!("Построение графика вынесено из основного кода урока.");
    disable();
    plot_correct_prediction_shares_for_compared_text_classifiers(baseline, correct);

    trace_note!("Строим график по результатам урока.");
    fn plot_correct_prediction_shares_for_compared_text_classifiers(baseline: f64, correct: usize) {
        trace_note!("Наглядное сравнение результатов сводной практики.");
        trace_note!("Передаём путь к каталогу текущего урока.");
        trace_note!("Указываем имя SVG-файла.");
        trace_note!("Указываем заголовок диаграммы.");
        trace_note!("Указываем подпись вертикальной оси.");
        trace_note!("Передаём ряды или значения для отрисовки графика.");
        trace_note!("Добавляем пару значений для сравнения или построения графика.");
        trace_note!("Добавляем пару значений для сравнения или построения графика.");
        trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
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
        trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
        println!("график: {}", chart.display());
    }
}
