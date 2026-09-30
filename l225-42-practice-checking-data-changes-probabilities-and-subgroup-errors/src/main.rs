// Урок 42.5. Практика: проверка изменений данных, вероятностей и ошибок по подгруппам.
// Зачем здесь эта тема: После выпуска одновременно важны сдвиг входов, калибровка, качество групп и
//   неопределённость.
// Почему код устроен так: Собираем метрики вместе и интерпретируем их с учётом числа наблюдений.
// Представь: Хорошее среднее качество ещё не отменяет сдвиг входов или ошибки отдельной группы.
//
// Что повторяем вместе: сдвиг распределения, калибровка, ошибки подгрупп, ограничения модели.
// Зачем это нужно: Качество AI нужно смотреть по подгруппам и вероятностям, учитывая ограничения маленькой
//   выборки.
// Что показывает программа: Разделяем набор по подгруппам. Для каждой группы считаем accuracy и
//   вероятностную ошибку Brier. Указываем ограничение вывода из очень маленькой выборки.
// Что проверить при изменении примера: Покажи ошибки, интервалы неопределённости и конкретные ограничения
//   вывода.
// Дополнительная практика: Составь набор сложных случаев для одного классификатора и отчёт по подгруппам.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    lesson_trace::enable();
    lesson_trace::trace_note!("Учебные реализации математических операций для этого урока.");

    lesson_trace::trace_note!(
        "Автоматически получаем стандартные реализации перечисленных трейтов для этого типа."
    );
    lesson_trace::trace_note!(
        "Описываем тип `EvaluationCase`, чтобы явно хранить состояние и допустимые варианты."
    );
    lesson_trace::trace_note!(
        "`group` задаёт соответствующее входное значение или поле структуры."
    );
    lesson_trace::trace_note!(
        "`truth` задаёт соответствующее входное значение или поле структуры."
    );
    lesson_trace::trace_note!(
        "`score` задаёт соответствующее входное значение или поле структуры."
    );
    #[derive(Clone, Copy, Debug)]
    struct EvaluationCase {
        group: &'static str,

        truth: bool,

        score: f64,
    }

    lesson_trace::trace_note!("Фиксируем демонстрационные данные на время выполнения программы.");
    lesson_trace::trace_note!("Создаём один пример с меткой, вероятностью и группой для оценки.");
    lesson_trace::trace_note!(
        "`group` задаёт соответствующее входное значение или поле структуры."
    );
    lesson_trace::trace_note!(
        "`truth` задаёт соответствующее входное значение или поле структуры."
    );
    lesson_trace::trace_note!(
        "`score` задаёт соответствующее входное значение или поле структуры."
    );
    lesson_trace::trace_note!("Создаём один пример с меткой, вероятностью и группой для оценки.");
    lesson_trace::trace_note!(
        "`group` задаёт соответствующее входное значение или поле структуры."
    );
    lesson_trace::trace_note!(
        "`truth` задаёт соответствующее входное значение или поле структуры."
    );
    lesson_trace::trace_note!(
        "`score` задаёт соответствующее входное значение или поле структуры."
    );
    lesson_trace::trace_note!("Создаём один пример с меткой, вероятностью и группой для оценки.");
    lesson_trace::trace_note!(
        "`group` задаёт соответствующее входное значение или поле структуры."
    );
    lesson_trace::trace_note!(
        "`truth` задаёт соответствующее входное значение или поле структуры."
    );
    lesson_trace::trace_note!(
        "`score` задаёт соответствующее входное значение или поле структуры."
    );
    lesson_trace::trace_note!("Создаём один пример с меткой, вероятностью и группой для оценки.");
    lesson_trace::trace_note!(
        "`group` задаёт соответствующее входное значение или поле структуры."
    );
    lesson_trace::trace_note!(
        "`truth` задаёт соответствующее входное значение или поле структуры."
    );
    lesson_trace::trace_note!(
        "`score` задаёт соответствующее входное значение или поле структуры."
    );
    lesson_trace::trace_note!("Создаём один пример с меткой, вероятностью и группой для оценки.");
    lesson_trace::trace_note!(
        "`group` задаёт соответствующее входное значение или поле структуры."
    );
    lesson_trace::trace_note!(
        "`truth` задаёт соответствующее входное значение или поле структуры."
    );
    lesson_trace::trace_note!(
        "`score` задаёт соответствующее входное значение или поле структуры."
    );
    lesson_trace::trace_note!("Создаём один пример с меткой, вероятностью и группой для оценки.");
    lesson_trace::trace_note!(
        "`group` задаёт соответствующее входное значение или поле структуры."
    );
    lesson_trace::trace_note!(
        "`truth` задаёт соответствующее входное значение или поле структуры."
    );
    lesson_trace::trace_note!(
        "`score` задаёт соответствующее входное значение или поле структуры."
    );
    lesson_trace::trace_note!("Создаём один пример с меткой, вероятностью и группой для оценки.");
    lesson_trace::trace_note!(
        "`group` задаёт соответствующее входное значение или поле структуры."
    );
    lesson_trace::trace_note!(
        "`truth` задаёт соответствующее входное значение или поле структуры."
    );
    lesson_trace::trace_note!(
        "`score` задаёт соответствующее входное значение или поле структуры."
    );
    lesson_trace::trace_note!("Создаём один пример с меткой, вероятностью и группой для оценки.");
    lesson_trace::trace_note!(
        "`group` задаёт соответствующее входное значение или поле структуры."
    );
    lesson_trace::trace_note!(
        "`truth` задаёт соответствующее входное значение или поле структуры."
    );
    lesson_trace::trace_note!(
        "`score` задаёт соответствующее входное значение или поле структуры."
    );
    const EVALUATION_CASES: [EvaluationCase; 8] = [
        EvaluationCase {
            group: "A",

            truth: true,

            score: 0.9,
        },
        EvaluationCase {
            group: "A",

            truth: false,

            score: 0.1,
        },
        EvaluationCase {
            group: "A",

            truth: true,

            score: 0.8,
        },
        EvaluationCase {
            group: "A",

            truth: false,

            score: 0.2,
        },
        EvaluationCase {
            group: "B",

            truth: true,

            score: 0.4,
        },
        EvaluationCase {
            group: "B",

            truth: false,

            score: 0.7,
        },
        EvaluationCase {
            group: "B",

            truth: true,

            score: 0.6,
        },
        EvaluationCase {
            group: "B",

            truth: false,

            score: 0.3,
        },
    ];

    lesson_trace::trace_note!("Шаг: Разделяем набор по подгруппам.");
    for group in ["A", "B"] {
        lesson_trace::trace_step!(group);
        lesson_trace::trace_note!(
            "Сохраняем рассчитанное значение `group_cases` для следующих операций."
        );
        lesson_trace::trace_note!("Перебираем элементы по ссылке, не копируя исходную коллекцию.");
        lesson_trace::trace_note!(
            "Копируем значения из ссылок, чтобы получить самостоятельные элементы."
        );
        lesson_trace::trace_note!("Оставляем только элементы, прошедшие указанную проверку.");
        lesson_trace::trace_note!("Собираем элементы итератора в итоговую коллекцию.");
        let group_cases: Vec<EvaluationCase> = EVALUATION_CASES
            .iter()
            .copied()
            .filter(|case| case.group == group)
            .collect();
        lesson_trace::trace_step!(group_cases);
        lesson_trace::trace_note!("Проверяем ожидаемое свойство учебного примера.");
        lesson_trace::trace_note!("Используем подготовленное значение в следующем шаге примера.");
        lesson_trace::trace_note!("Передаём подпись или текстовое значение для следующего шага.");
        assert!(
            !group_cases.is_empty(),
            "для оценки группы нужен хотя бы один пример"
        );
        lesson_trace::trace_note!("Проверяем ожидаемое свойство учебного примера.");
        lesson_trace::trace_note!("Используем подготовленное значение в следующем шаге примера.");
        lesson_trace::trace_note!("Просматриваем элементы коллекции по ссылке.");
        lesson_trace::trace_note!("Настраиваем или преобразуем результат предыдущего шага.");
        lesson_trace::trace_note!("Передаём подпись или текстовое значение для следующего шага.");
        assert!(
            group_cases
                .iter()
                .all(|case| (0.0..=1.0).contains(&case.score)),
            "оценка вероятности должна быть от 0 до 1"
        );
        lesson_trace::trace_note!(
            "Шаг: Для каждой группы считаем accuracy и вероятностную ошибку Brier."
        );
        lesson_trace::trace_note!(
            "Задаём шаблон строки: плейсхолдеры ниже заменятся рассчитанными значениями."
        );
        lesson_trace::trace_note!("Передаём очередное значение в составе результата или вызова.");
        lesson_trace::trace_note!(
            "Составляем результат из вычисленных значений в указанном порядке."
        );
        lesson_trace::trace_note!("Используем подготовленное значение в следующем шаге примера.");
        lesson_trace::trace_note!("Считаем долю прогнозов, совпавших с истинными метками.");
        lesson_trace::trace_note!("Сохраняем результат этого шага в `data`.");
        lesson_trace::trace_note!("Перебираем оценочные примеры по ссылке.");
        lesson_trace::trace_note!(
            "Порог 0.5 превращает вероятность класса 1 в бинарный прогноз перед сравнением с truth."
        );
        lesson_trace::trace_note!("Подсчитываем число элементов после отбора.");
        lesson_trace::trace_note!("Делим значения, получая нормированную величину или среднее.");
        lesson_trace::trace_note!(
            "Составляем результат из вычисленных значений в указанном порядке."
        );
        lesson_trace::trace_note!("Используем подготовленное значение в следующем шаге примера.");
        lesson_trace::trace_note!(
            "Средний квадрат разницы вероятности и метки измеряет качество вероятностного прогноза."
        );
        lesson_trace::trace_note!("Сохраняем результат этого шага в `data`.");
        lesson_trace::trace_note!(
            "Инициализируем изменяемый накопитель `squared_error_sum` начальным состоянием."
        );
        lesson_trace::trace_note!(
            "Повторяем следующий блок для каждого элемента указанной последовательности."
        );
        lesson_trace::trace_note!(
            "Сохраняем рассчитанное значение `target` для следующих операций."
        );
        lesson_trace::trace_note!("Прибавляем очередной вклад к ранее накопленному результату.");
        lesson_trace::trace_note!("Используем подготовленное значение в следующем шаге примера.");
        lesson_trace::trace_note!("Возводим число в квадрат обычным умножением.");
        lesson_trace::trace_note!("Сохраняем результат этого шага в `value`.");
        lesson_trace::trace_note!("Умножаем величины согласно используемой формуле.");
        lesson_trace::trace_note!("Делим значения, получая нормированную величину или среднее.");
        println!(
            "group={group}, n={}, accuracy={:.2}, Brier={:.3}",
            group_cases.len(),
            (|| -> f64 {
                let data: &[EvaluationCase] = &group_cases;
                lesson_trace::trace_step!(data);
                lesson_trace::trace_step!(data);

                data.iter()
                    .filter(|case| (case.score >= 0.5) == case.truth)
                    .count() as f64
                    / data.len() as f64
            })(),
            (|| -> f64 {
                let data: &[EvaluationCase] = &group_cases;
                lesson_trace::trace_step!(data);
                lesson_trace::trace_step!(data);

                let mut squared_error_sum: f64 = 0.0;
                lesson_trace::trace_step!(squared_error_sum);
                lesson_trace::trace_step!(squared_error_sum);

                for case in data {
                    lesson_trace::trace_step!(case);

                    let target: f64 = if case.truth { 1.0 } else { 0.0 };
                    lesson_trace::trace_step!(target);
                    lesson_trace::trace_step!(target);

                    squared_error_sum += (|| -> f64 {
                        let value: f64 = case.score - target;
                        lesson_trace::trace_step!(value);
                        lesson_trace::trace_step!(value);

                        value * value
                    })();
                    lesson_trace::trace_step!(squared_error_sum);
                }

                squared_error_sum / data.len() as f64
            })()
        );
    }
    lesson_trace::trace_note!("Шаг: Указываем ограничение вывода из очень маленькой выборки.");
    println!("Ограничение: по четырём примерам на группу нельзя оценить реальное качество.");

    lesson_trace::trace_note!("Построение графика вынесено из основного кода урока.");
    lesson_trace::disable();
    plot_correct_prediction_share_in_each_evaluation_subgroup();

    lesson_trace::trace_note!("Строим график по результатам урока.");
    fn plot_correct_prediction_share_in_each_evaluation_subgroup() {
        lesson_trace::trace_note!(
            "Сравниваем качество по группам, используя те же оценочные примеры."
        );
        let group_accuracy: &dyn Fn(&str) -> f64 = &|group: &str| {
            lesson_trace::trace_note!("Собираем значения для `cases` в коллекцию.");
            lesson_trace::trace_note!("Просматриваем элементы коллекции по ссылке.");
            lesson_trace::trace_note!("Оставляем элементы, отвечающие условию.");
            lesson_trace::trace_note!("Собираем результаты в коллекцию.");
            let cases: Vec<&EvaluationCase> = EVALUATION_CASES
                .iter()
                .filter(|case| case.group == group)
                .collect();
            lesson_trace::trace_note!(
                "Используем подготовленное значение в следующем шаге примера."
            );
            lesson_trace::trace_note!("Просматриваем элементы коллекции по ссылке.");
            lesson_trace::trace_note!(
                "Используем тот же порог 0.5 для сравнимой точности каждой группы."
            );
            lesson_trace::trace_note!("Подсчитываем число подходящих элементов.");
            lesson_trace::trace_note!(
                "Используем подготовленное значение в следующем шаге примера."
            );
            cases
                .iter()
                .filter(|case| (case.score >= 0.5) == case.truth)
                .count() as f64
                / cases.len() as f64
        };
        lesson_trace::trace_note!(
            "Строим график по рассчитанным значениям и сохраняем его как SVG."
        );
        lesson_trace::trace_note!("Передаём путь к каталогу текущего урока.");
        lesson_trace::trace_note!("Указываем имя SVG-файла.");
        lesson_trace::trace_note!("Указываем заголовок диаграммы.");
        lesson_trace::trace_note!("Указываем подпись вертикальной оси.");
        lesson_trace::trace_note!("Передаём ряды или значения для отрисовки графика.");
        lesson_trace::trace_note!(
            "Прерываем пример с понятной ошибкой, если SVG не удалось записать."
        );
        let chart: std::path::PathBuf = lesson_visualization::bar_chart(
            env!("CARGO_MANIFEST_DIR"),
            "lesson-chart",
            "Accuracy по подгруппам",
            "accuracy",
            &[("A", group_accuracy("A")), ("B", group_accuracy("B"))],
        )
        .expect("не удалось сохранить график");
        lesson_trace::trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
        println!("график: {}", chart.display());
    }
}
