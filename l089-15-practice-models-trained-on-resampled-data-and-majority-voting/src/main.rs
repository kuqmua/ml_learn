// Урок 15.5. Практика: обучение моделей на повторных выборках и голосование за класс.
// Связь с принятой терминологией: Ансамбль с выборками с возвращением и голосованием.
// Зачем здесь эта тема: Bagging соединяет bootstrap, независимые модели и голосование для
//   уменьшения нестабильности.
// Почему код устроен так: На одном малом наборе показываем, как повторные строки меняют модели и
//   общий ответ.
// Представь: Сначала создаём несколько выборок, затем обучаем модель на каждой и объединяем ответы.
//
// Что повторяем вместе: bagging, bootstrap, majority vote, bias/variance.
// Зачем это нужно: Ансамбль объединяет несколько простых моделей, обученных на разных выборках, чтобы
//   снизить зависимость от одной модели.
// Что показывает программа: Берём обучающие точки для нескольких базовых моделей. Каждый stump обучаем на
//   своей bootstrap-выборке. Объединяем прогнозы моделей голосованием большинства.
// Что проверить при изменении примера: Сравни одно дерево с ансамблем на одинаковом split; фиксируй seed
//   каждого дерева.
// Дополнительная практика: Собери несколько деревьев на bootstrap-выборках и усредни прогнозы.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    lesson_trace::enable();
    lesson_trace::trace_note!("Шаг: Берём обучающие точки для нескольких базовых моделей.");
    lesson_trace::trace_note!("Учебный объект: признак 0., метка класса false.");
    lesson_trace::trace_note!("Учебный объект: признак 1., метка класса false.");
    lesson_trace::trace_note!("Учебный объект: признак 2., метка класса true.");
    lesson_trace::trace_note!("Учебный объект: признак 3., метка класса true.");
    lesson_trace::trace_note!("Учебный объект: признак 4., метка класса false.");
    lesson_trace::trace_note!("Учебный объект: признак 5., метка класса true.");
    let data: [(f64, bool); 6] = [
        (0., false),
        (1., false),
        (2., true),
        (3., true),
        (4., false),
        (5., true),
    ];
    lesson_trace::trace_step!(data);

    lesson_trace::trace_note!("Шаг: Каждый stump обучаем на своей bootstrap-выборке.");
    lesson_trace::trace_note!("Преобразуем каждый элемент последовательности.");
    lesson_trace::trace_note!("Собираем элементы итератора в итоговую коллекцию.");
    let models: Vec<(f64, bool)> = (1..=9)

        .map(|seed| {
            lesson_trace::trace_note!("Составляем результат из вычисленных значений в указанном порядке.");
            (|| -> (f64, bool) {
                lesson_trace::trace_note!("Используем подготовленное значение в следующем шаге примера.");
                lesson_trace::trace_note!("Перебираем пороги и направления, выбирая пень с минимальным числом ошибок.");
                lesson_trace::trace_note!("Собираем значения для `data` в коллекцию.");
                let data: &[(f64, bool)] = &(|| -> Vec<(f64, bool)> {
                    lesson_trace::trace_note!("Используем подготовленное значение в следующем шаге примера.");
                    lesson_trace::trace_note!("Создаём выборку той же длины с возвращением и фиксированным seed.");
                    lesson_trace::trace_note!("Сохраняем результат этого шага в `data`.");
                    let data: &[(f64, bool)] = &data;
                    lesson_trace::trace_step!(data);
                    lesson_trace::trace_note!("Сохраняем рассчитанное значение `seed` для следующих операций.");
                    let seed: u64 = seed;
                    lesson_trace::trace_step!(seed);
                    lesson_trace::trace_note!("Создаём изменяемое значение `generator_state` для следующих операций.");
                    let mut generator_state: u64 = seed;
                    lesson_trace::trace_step!(generator_state);
                    lesson_trace::trace_note!("Составляем результат из вычисленных значений в указанном порядке.");
                    lesson_trace::trace_note!("Преобразуем каждый элемент последовательности.");
                    lesson_trace::trace_note!("Собираем элементы итератора в итоговую коллекцию.");
                    (0..data.len())

                        .map(|_| {
                            lesson_trace::trace_note!("Линейный конгруэнтный генератор: фиксированный множитель и +1 по mod 2⁶⁴.");
                            lesson_trace::trace_note!("Используем арифметику с переполнением для воспроизводимого генератора.");
                            lesson_trace::trace_note!("Используем арифметику с переполнением для воспроизводимого генератора.");
                            generator_state = generator_state

                                .wrapping_mul(6364136223846793005)

                                .wrapping_add(1);
                            lesson_trace::trace_step!(generator_state);
                            lesson_trace::trace_note!("Передаём ранее рассчитанное значение в текущую операцию.");
                            data[(generator_state as usize) % data.len()]
                        })

                        .collect()
                })();
                lesson_trace::trace_step!(data);
                lesson_trace::trace_note!("Создаём изменяемое значение `best` для следующих операций.");
                let mut best: (f64, f64, bool) = (f64::INFINITY, 0., false);
                lesson_trace::trace_step!(best);
                lesson_trace::trace_note!("Повторяем следующий блок для каждого элемента указанной последовательности.");
                for &(candidate_threshold, _) in data {
                    lesson_trace::trace_step!(candidate_threshold);
                    lesson_trace::trace_note!("Повторяем следующий блок для каждого элемента указанной последовательности.");
                    for reverse in [false, true] {
                        lesson_trace::trace_step!(reverse);
                        lesson_trace::trace_note!("Сохраняем рассчитанное значение `errors` для следующих операций.");
                        lesson_trace::trace_note!("Перебираем элементы по ссылке, не копируя исходную коллекцию.");
                        lesson_trace::trace_note!("Оставляем только элементы, прошедшие указанную проверку.");
                        lesson_trace::trace_note!("Подсчитываем число элементов после отбора.");
                        let errors: f64 = data

                            .iter()

                            .filter(|&&(feature_value, label)| {
                                lesson_trace::trace_note!("Составляем результат из вычисленных значений в указанном порядке.");
                                ((feature_value >= candidate_threshold) ^ reverse) != label
                            })

                            .count() as f64;
                        lesson_trace::trace_step!(errors);
                        lesson_trace::trace_note!("Проверяем условие и выбираем соответствующую ветку алгоритма.");
                        if errors < best.0 {
                            lesson_trace::trace_note!("Обновляем `best` результатом текущего шага.");
                            best = (errors, candidate_threshold, reverse);
                            lesson_trace::trace_step!(best);
                        }
                    }
                }
                lesson_trace::trace_note!("Составляем результат из вычисленных значений в указанном порядке.");
                (best.1, best.2)
            })()
        })

        .collect();
    lesson_trace::trace_step!(models);

    lesson_trace::trace_note!("Шаг: Объединяем прогнозы моделей голосованием большинства.");
    lesson_trace::trace_note!("Используем подготовленное значение в следующем шаге примера.");
    lesson_trace::trace_note!("Передаём подпись или текстовое значение для следующего шага.");
    assert!(
        !models.is_empty(),
        "для сравнения нужна хотя бы одна модель"
    );
    lesson_trace::trace_note!("Повторяем расчёт для каждого элемента последовательности.");
    for feature_value in [0.5, 2.5, 4.5] {
        lesson_trace::trace_step!(feature_value);
        lesson_trace::trace_note!(
            "Показываем ответ одного и того же базового дерева рядом с ответом ансамбля."
        );
        let first_model: bool = (feature_value >= models[0].0) ^ models[0].1;
        lesson_trace::trace_step!(first_model);
        lesson_trace::trace_note!(
            "Сохраняем рассчитанное значение `votes` для следующих операций."
        );
        lesson_trace::trace_note!("Перебираем элементы по ссылке, не копируя исходную коллекцию.");
        lesson_trace::trace_note!("Оставляем только элементы, прошедшие указанную проверку.");
        lesson_trace::trace_note!("Подсчитываем число элементов после отбора.");
        let votes: usize = models
            .iter()
            .filter(|&&model| {
                lesson_trace::trace_note!(
                    "Составляем результат из вычисленных значений в указанном порядке."
                );
                (|| -> bool {
                    lesson_trace::trace_note!(
                        "Используем подготовленное значение в следующем шаге примера."
                    );
                    lesson_trace::trace_note!(
                        "Сравниваем признак с порогом и учитываем направление пня."
                    );
                    lesson_trace::trace_note!("Сохраняем результат этого шага в `model`.");
                    let model: (f64, bool) = model;
                    lesson_trace::trace_step!(model);
                    lesson_trace::trace_note!(
                        "Сохраняем рассчитанное значение `feature_value` для следующих операций."
                    );
                    let feature_value: f64 = feature_value;
                    lesson_trace::trace_step!(feature_value);
                    lesson_trace::trace_note!(
                        "Составляем результат из вычисленных значений в указанном порядке."
                    );
                    (feature_value >= model.0) ^ model.1
                })()
            })
            .count();
        lesson_trace::trace_step!(votes);
        lesson_trace::trace_note!(
            "Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением."
        );
        lesson_trace::trace_note!(
            "Задаём шаблон строки: плейсхолдеры ниже заменятся рассчитанными значениями."
        );
        lesson_trace::trace_note!("Передаём очередное значение в составе результата или вызова.");
        lesson_trace::trace_note!("Умножаем величины согласно используемой формуле.");
        println!(
            "x={feature_value}: первое дерево={first_model}, {votes}/{} positive -> ансамбль={}",
            models.len(),
            votes * 2 > models.len()
        );
    }

    lesson_trace::trace_note!("Построение графика вынесено из основного кода урока.");
    lesson_trace::disable();
    plot_thresholds_learned_by_resampled_models(models);
}

// Строим график по результатам урока.
fn plot_thresholds_learned_by_resampled_models(models: std::vec::Vec<(f64, bool)>) {
    lesson_trace::trace_note!("Наглядное представление вычислений сводной практики.");
    lesson_trace::trace_note!("Просматриваем элементы коллекции по ссылке.");
    lesson_trace::trace_note!("Добавляем порядковый номер к каждому элементу.");
    lesson_trace::trace_note!("Преобразуем каждый элемент в новое значение.");
    lesson_trace::trace_note!("Собираем результаты в коллекцию.");
    let ensembles_points: Vec<(f64, f64)> = models
        .iter()
        .enumerate()
        .map(|(item_index, (threshold, _))| (item_index as f64, *threshold))
        .collect();
    lesson_trace::trace_note!("Строим график по рассчитанным значениям и сохраняем его как SVG.");
    lesson_trace::trace_note!("Передаём путь к каталогу текущего урока.");
    lesson_trace::trace_note!("Указываем имя SVG-файла.");
    lesson_trace::trace_note!("Указываем заголовок графика.");
    lesson_trace::trace_note!("Указываем подпись горизонтальной оси.");
    lesson_trace::trace_note!("Указываем подпись вертикальной оси.");
    lesson_trace::trace_note!("Передаём ряды или значения для отрисовки графика.");
    lesson_trace::trace_note!("Указываем подпись этого ряда в легенде.");
    lesson_trace::trace_note!("Передаём рассчитанные координаты точек.");
    lesson_trace::trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
    let chart: std::path::PathBuf = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Пороги моделей ансамбля",
        "номер модели",
        "порог",
        &[lesson_visualization::Series {
            name: "пороги",

            points: &ensembles_points,
        }],
    )
    .expect("не удалось сохранить график");
    lesson_trace::trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
