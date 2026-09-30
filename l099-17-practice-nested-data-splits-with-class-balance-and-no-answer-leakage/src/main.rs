// Урок 17.5. Практика: вложенное разделение данных с сохранением долей классов и без подсказок из проверочных ответов.
// Связь с принятой терминологией: Стратифицированная вложенная кросс-валидация без утечки.
// Зачем здесь эта тема: Стратификация, подбор параметров и подготовка признаков должны соблюдаться
//   на обоих уровнях проверки.
// Почему код устроен так: Вкладываем весь конвейер во внутренний цикл и оставляем внешний блок
//   нетронутым.
// Представь: Для каждого внешнего проверочного блока заново выбираем настройки и учим
//   преобразования на оставшихся данных.
//
// Что повторяем вместе: k-fold, стратификация, nested evaluation, утечка в preprocessing.
// Зачем это нужно: Кросс-валидация использует несколько разбиений для выбора параметра, сохраняя test для
//   итоговой оценки.
// Что показывает программа: Задаём данные для выбора числа соседей. Сравниваем значения гиперпараметра на
//   кросс-валидации. После выбора параметра один раз оцениваем качество на test.
// Что проверить при изменении примера: Каждый объект ровно один раз попадает в validation; test
//   используется один раз после выбора модели.
// Дополнительная практика: Реализуй k-fold подбор одного гиперпараметра для модели из предыдущих уроков.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    lesson_trace::enable();
    lesson_trace::trace_note!("Шаг: Задаём данные для выбора числа соседей.");
    lesson_trace::trace_note!("Учебный объект: признак 0., метка класса false.");
    lesson_trace::trace_note!("Учебный объект: признак 1., метка класса false.");
    lesson_trace::trace_note!("Учебный объект: признак 2., метка класса false.");
    lesson_trace::trace_note!("Учебный объект: признак 3., метка класса true.");
    lesson_trace::trace_note!("Учебный объект: признак 4., метка класса true.");
    lesson_trace::trace_note!("Учебный объект: признак 5., метка класса true.");
    lesson_trace::trace_note!("Учебный объект: признак 6., метка класса true.");
    lesson_trace::trace_note!("Учебный объект: признак 7., метка класса false.");
    lesson_trace::trace_note!("Учебный объект: признак 8., метка класса false.");
    let training_examples: [(f64, bool); 9] = [
        (0., false),
        (1., false),
        (2., false),
        (3., true),
        (4., true),
        (5., true),
        (6., true),
        (7., false),
        (8., false),
    ];
    lesson_trace::trace_step!(training_examples);

    lesson_trace::trace_note!("Учебные реализации математических операций для этого урока.");

    lesson_trace::trace_note!(
        "Объявляем повторно используемое вычисление `choose_majority_class_among_nearest_training_values`; параметры ниже задают его входы."
    );
    /// Метод ближайших соседей: сортируем обучающие значения по расстоянию и выбираем большинство среди заданного числа ближайших.
    fn choose_majority_class_among_nearest_training_values(
        training_examples: &[(f64, bool)],

        feature_value: f64,

        neighbor_count: usize,
    ) -> bool {
        lesson_trace::trace_note!(
            "Получаем размеченные обучающие примеры по ссылке без копирования."
        );
        lesson_trace::trace_note!(
            "`feature_value` задаёт соответствующее входное значение или поле структуры."
        );
        lesson_trace::trace_note!(
            "`neighbor_count` задаёт соответствующее входное значение или поле структуры."
        );
        lesson_trace::trace_note!("Указываем тип возвращаемого значения.");
        lesson_trace::trace_note!("Проверяем ожидаемое свойство учебного примера.");
        lesson_trace::trace_note!("Обновляем значение результатом текущего вычисления.");
        lesson_trace::trace_note!("Передаём подпись или текстовое значение для следующего шага.");
        assert!(
            neighbor_count > 0 && neighbor_count <= training_examples.len(),
            "число соседей должно быть от 1 до числа обучающих примеров"
        );
        lesson_trace::trace_note!(
            "Создаём изменяемое значение `sorted_neighbors` для следующих операций."
        );
        lesson_trace::trace_note!("Перебираем элементы по ссылке, не копируя исходную коллекцию.");
        lesson_trace::trace_note!("Преобразуем каждый элемент последовательности.");
        lesson_trace::trace_note!("Собираем элементы итератора в итоговую коллекцию.");
        let mut sorted_neighbors: Vec<(f64, bool)> = training_examples

            .iter()

            .map(|&(training_feature, label)| {
                lesson_trace::trace_note!("Составляем результат из вычисленных значений в указанном порядке.");
                lesson_trace::trace_note!("Составляем результат из вычисленных значений в указанном порядке.");
                lesson_trace::trace_note!("Используем ранее рассчитанное значение `label` в текущем выражении.");
                (

                    (|| -> f64 {
                        lesson_trace::trace_note!("Используем подготовленное значение в следующем шаге примера.");
                        lesson_trace::trace_note!("Модуль числа по определению: меняем знак только у отрицательного числа.");
                        lesson_trace::trace_note!("Сохраняем результат этого шага в `value`.");
                        let value: f64 = training_feature - feature_value;
                        lesson_trace::trace_step!(value);
                        lesson_trace::trace_note!("Проверяем условие и выбираем соответствующую ветку алгоритма.");
                        if value < 0.0 { -value } else { value }
                    })(),

                    label,
                )
            })

            .collect();
        lesson_trace::trace_step!(sorted_neighbors);
        lesson_trace::trace_note!(
            "Используем ранее рассчитанное значение `sorted_neighbors` в текущем выражении."
        );
        lesson_trace::trace_note!("Упорядочиваем данные для следующего шага алгоритма.");
        sorted_neighbors
            .sort_by(|left_result, right_result| left_result.0.total_cmp(&right_result.0));
        lesson_trace::trace_note!(
            "Используем ранее рассчитанное значение `sorted_neighbors` в текущем выражении."
        );
        lesson_trace::trace_note!("Перебираем элементы по ссылке, не копируя исходную коллекцию.");
        lesson_trace::trace_note!("Оставляем только заданное число лучших элементов.");
        lesson_trace::trace_note!("Оставляем только элементы, прошедшие указанную проверку.");
        lesson_trace::trace_note!("Подсчитываем число элементов после отбора.");
        lesson_trace::trace_note!("Добавляем этот член в составное арифметическое выражение.");
        lesson_trace::trace_note!("Сравниваем значения, чтобы получить булев результат.");
        sorted_neighbors
            .iter()
            .take(neighbor_count)
            .filter(|(_, label)| *label)
            .count()
            * 2
            > neighbor_count
    }

    lesson_trace::trace_note!("Шаг: Сравниваем значения гиперпараметра на кросс-валидации.");
    lesson_trace::trace_note!("Передаём владение элементами итератору для дальнейшей обработки.");
    lesson_trace::trace_note!("Преобразуем каждый элемент последовательности.");
    lesson_trace::trace_note!("Сравниваем кандидатов и оставляем наибольший результат.");
    lesson_trace::trace_note!("Извлекаем значение: выше в примере обеспечено отсутствие ошибки.");
    let best: (usize, f64) = [1, 3, 5]

        .into_iter()

        .map(|neighbor_count| {
            lesson_trace::trace_note!("Составляем результат из вычисленных значений в указанном порядке.");
            lesson_trace::trace_note!("Используем ранее рассчитанное значение `neighbor_count` в текущем выражении.");
            lesson_trace::trace_note!("Составляем результат из вычисленных значений в указанном порядке.");
            (

                neighbor_count,

                (|| -> f64 {
                    lesson_trace::trace_note!("Используем подготовленное значение в следующем шаге примера.");
                    lesson_trace::trace_note!("Каждый объект один раз становится проверочным, остальные участвуют в обучении.");
                    lesson_trace::trace_note!("Сохраняем результат этого шага в `data`.");
                    let data: &[(f64, bool)] = &training_examples;
                    lesson_trace::trace_step!(data);
                    lesson_trace::trace_note!("Сохраняем рассчитанное значение `neighbor_count` для следующих операций.");
                    let neighbor_count: usize = neighbor_count;
                    lesson_trace::trace_step!(neighbor_count);
                    lesson_trace::trace_note!("Сохраняем рассчитанное значение `fold_count` для следующих операций.");
                    let fold_count: usize = 3;
                    lesson_trace::trace_step!(fold_count);
                    lesson_trace::trace_note!("Проверяем ожидаемое свойство учебного примера.");
                    assert!(data.len() >= fold_count, "для каждого блока нужен хотя бы один пример");
                    lesson_trace::trace_note!("Инициализируем изменяемый накопитель `correct_predictions` начальным состоянием.");
                    let mut correct_predictions: usize = 0;
                    lesson_trace::trace_step!(correct_predictions);
                    lesson_trace::trace_note!("Повторяем следующий блок для каждого элемента указанной последовательности.");
                    for fold in 0..fold_count {
                        lesson_trace::trace_step!(fold);
                        lesson_trace::trace_note!("Сохраняем рассчитанное значение `training_examples` для следующих операций.");
                        lesson_trace::trace_note!("Перебираем элементы по ссылке, не копируя исходную коллекцию.");
                        lesson_trace::trace_note!("Добавляем порядковый индекс к каждому элементу обхода.");
                        lesson_trace::trace_note!("Оставляем только элементы, прошедшие указанную проверку.");
                        lesson_trace::trace_note!("Преобразуем каждый элемент последовательности.");
                        lesson_trace::trace_note!("Собираем элементы итератора в итоговую коллекцию.");
                        let training_examples: Vec<(f64, bool)> = data

                            .iter()

                            .enumerate()

                            .filter(|(sample_index, _)| sample_index % fold_count != fold)

                            .map(|(_, record)| *record)

                            .collect();
                        lesson_trace::trace_step!(training_examples);
                        lesson_trace::trace_note!("Повторяем следующий блок для каждого элемента указанной последовательности.");
                        lesson_trace::trace_note!("Перебираем элементы по ссылке, не копируя исходную коллекцию.");
                        lesson_trace::trace_note!("Добавляем порядковый индекс к каждому элементу обхода.");
                        lesson_trace::trace_note!("Оставляем только элементы, прошедшие указанную проверку.");
                        for (_, record) in data

                            .iter()

                            .enumerate()

                            .filter(|(sample_index, _)| sample_index % fold_count == fold)
                        {
                            lesson_trace::trace_step!(record);
                            lesson_trace::trace_note!("Прибавляем очередной вклад к ранее накопленному результату.");
                            lesson_trace::trace_note!("Вызываем нужное вычисление с подготовленными аргументами.");
                            lesson_trace::trace_note!("Передаём данные по ссылке или разыменовываем их для следующей операции.");
                            lesson_trace::trace_note!("Передаём очередное значение в составе результата или вызова.");
                            lesson_trace::trace_note!("Используем ранее рассчитанное значение `neighbor_count` в текущем выражении.");
                            lesson_trace::trace_note!("Обновляем значение результатом текущего вычисления.");
                            correct_predictions += usize::from(

                                choose_majority_class_among_nearest_training_values(

                                    &training_examples,

                                    record.0,

                                    neighbor_count,

                                ) == record.1,
                            );
                            lesson_trace::trace_step!(correct_predictions);
                        }
                    }
                    lesson_trace::trace_note!("Делим значения, получая нормированную величину или среднее.");
                    correct_predictions as f64 / data.len() as f64
                })(),
            )
        })

        .max_by(|left_result, right_result| left_result.1.total_cmp(&right_result.1))

        .unwrap();
    lesson_trace::trace_step!(best);
    lesson_trace::trace_note!("Шаг: После выбора параметра один раз оцениваем качество на test.");
    let test: [(f64, bool); 2] = [(2.5, false), (5.5, true)];
    lesson_trace::trace_step!(test);
    lesson_trace::trace_note!("Сохраняем рассчитанное значение `accuracy` для следующих операций.");
    lesson_trace::trace_note!("Перебираем элементы по ссылке, не копируя исходную коллекцию.");
    lesson_trace::trace_note!("Оставляем только элементы, прошедшие указанную проверку.");
    lesson_trace::trace_note!("Подсчитываем число элементов после отбора.");
    lesson_trace::trace_note!("Делим значения, получая нормированную величину или среднее.");
    let accuracy: f64 = test
        .iter()
        .filter(|&&(feature_value, label)| {
            lesson_trace::trace_note!("Вызываем нужное вычисление с подготовленными аргументами.");
            lesson_trace::trace_note!("Проверяем логическое условие для текущих элементов.");
            choose_majority_class_among_nearest_training_values(
                &training_examples,
                feature_value,
                best.0,
            ) == label
        })
        .count() as f64
        / test.len() as f64;
    lesson_trace::trace_step!(accuracy);
    lesson_trace::trace_note!(
        "Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением."
    );
    lesson_trace::trace_note!(
        "Задаём шаблон строки: плейсхолдеры ниже заменятся рассчитанными значениями."
    );
    lesson_trace::trace_note!("Возвращаем найденные гиперпараметр и оценку его качества.");
    lesson_trace::trace_note!("Передаём найденную оценку качества выбранного числа соседей.");
    println!(
        "selected k={}, CV={:.2}, test={accuracy:.2}",
        best.0, best.1
    );

    lesson_trace::trace_note!("Построение графика вынесено из основного кода урока.");
    lesson_trace::disable();
    plot_correct_prediction_shares_on_validation_and_test_data(best, accuracy);
}

// Строим график по результатам урока.
fn plot_correct_prediction_shares_on_validation_and_test_data(best: (usize, f64), accuracy: f64) {
    lesson_trace::trace_note!("Наглядное сравнение результатов сводной практики.");
    lesson_trace::trace_note!("Передаём путь к каталогу текущего урока.");
    lesson_trace::trace_note!("Указываем имя SVG-файла.");
    lesson_trace::trace_note!("Указываем заголовок диаграммы.");
    lesson_trace::trace_note!("Указываем подпись вертикальной оси.");
    lesson_trace::trace_note!("Передаём ряды или значения для отрисовки графика.");
    lesson_trace::trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
    let chart: std::path::PathBuf = lesson_visualization::bar_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Кросс-валидация и тест",
        "accuracy",
        &[("CV", best.1), ("test", accuracy)],
    )
    .expect("не удалось сохранить график");
    lesson_trace::trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
