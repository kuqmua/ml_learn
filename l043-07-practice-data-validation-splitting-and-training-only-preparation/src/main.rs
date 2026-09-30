// Урок 07.7. Практика: проверка данных, разделение и подготовка только по обучающей части.
// Связь с принятой терминологией: Схема данных, типы признаков, разбиение и подготовка без утечки.
// Зачем здесь эта тема: Безопасная обработка данных зависит от порядка шагов: схема, разбиение,
//   обучение преобразований, применение.
// Почему код устроен так: Собираем эти шаги в один конвейер и проверяем отсутствие утечки на
//   validation и test.
// Представь: Новый CSV сначала проходит проверку полей, затем разбиение, и только после этого
//   вычисляются статистики train.
//
// Что повторяем вместе: схема данных, типы признаков, train/validation/test, утечка данных.
// Зачем это нужно: Разделение данных и расчёт статистик только по train защищают оценку модели от утечки
//   информации.
// Что показывает программа: Читаем учебную таблицу признаков и меток. Отделяем обучающую, проверочную и
//   тестовую части. Считаем среднее только по train: validation и test не влияют на подготовку признаков.
// Что проверить при изменении примера: Проверь отсутствие пересечений между частями и повторяемость
//   разбиения.
// Дополнительная практика: Прочитай локальный CSV, проверь схему, разбей данные с seed; вычисляй параметры
//   нормализации только на train.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    lesson_trace::enable();
    lesson_trace::trace_note!("Фиксируем демонстрационные данные на время выполнения программы.");
    const SAMPLE_COMMA_SEPARATED_VALUES: &str =
        "feature,target\n1,0\n2,0\n3,1\n4,1\n5,0\n6,1\n7,0\n8,1\n9,1\n10,0\n";

    lesson_trace::trace_note!("Шаг: Читаем учебную таблицу признаков и меток.");
    let records: Vec<(f64, u8)> = (|| -> Vec<(f64, u8)> {
        lesson_trace::trace_note!("Используем подготовленное значение в следующем шаге примера.");
        lesson_trace::trace_note!("Разбираем демонстрационный CSV в пары «признак, метка».");
        lesson_trace::trace_note!("Используем подготовленное значение в следующем шаге примера.");
        lesson_trace::trace_note!("Разбиваем текст на строки для последовательной обработки.");
        lesson_trace::trace_note!(
            "Пропускаем первые элементы, которые не входят в полезные данные."
        );
        lesson_trace::trace_note!("Преобразуем каждый элемент последовательности.");
        lesson_trace::trace_note!("Собираем элементы итератора в итоговую коллекцию.");
        SAMPLE_COMMA_SEPARATED_VALUES

            .lines()

            .skip(1)

            .map(|line| {
                lesson_trace::trace_note!("Сохраняем рассчитанное значение `(feature_value, target_value)` для следующих операций.");
                let (feature_value, target_value): (&str, &str) = line.split_once(',').unwrap();
                lesson_trace::trace_step!(feature_value);
                lesson_trace::trace_step!(target_value);
                lesson_trace::trace_note!("Составляем результат из вычисленных значений в указанном порядке.");
                lesson_trace::trace_note!("Преобразуем текстовое поле в требуемый числовой тип.");
                lesson_trace::trace_note!("Преобразуем текстовое поле в требуемый числовой тип.");
                (

                    feature_value.parse().unwrap(),

                    target_value.parse().unwrap(),
                )
            })

            .collect()
    })();
    lesson_trace::trace_step!(records);
    lesson_trace::trace_note!("Шаг: Отделяем обучающую, проверочную и тестовую части.");
    lesson_trace::trace_note!("Обновляем значение результатом текущего вычисления.");
    lesson_trace::trace_note!("Передаём подпись или текстовое значение для следующего шага.");
    assert!(
        records.len() >= 9,
        "для разделения нужны строки train, validation и test"
    );
    lesson_trace::trace_note!("Сохраняем результат этого шага в `training_records`.");
    let training_records: &[(f64, u8)] = &records[..6];
    lesson_trace::trace_step!(training_records);
    lesson_trace::trace_note!(
        "Сохраняем рассчитанное значение `validation_records` для следующих операций."
    );
    let validation_records: &[(f64, u8)] = &records[6..8];
    lesson_trace::trace_step!(validation_records);
    lesson_trace::trace_note!(
        "Сохраняем рассчитанное значение `test_records` для следующих операций."
    );
    let test_records: &[(f64, u8)] = &records[8..];
    lesson_trace::trace_step!(test_records);

    lesson_trace::trace_note!(
        "Шаг: Считаем среднее только по train: validation и test не влияют на подготовку признаков."
    );
    let training_mean: f64 = (|| -> f64 {
        lesson_trace::trace_note!("Используем подготовленное значение в следующем шаге примера.");
        lesson_trace::trace_note!("Среднее признака считаем только по обучающим строкам.");
        lesson_trace::trace_note!("Сохраняем результат этого шага в `training_records`.");
        let training_records: &[(f64, u8)] = training_records;
        lesson_trace::trace_step!(training_records);
        lesson_trace::trace_note!(
            "Инициализируем изменяемый накопитель `feature_sum` начальным состоянием."
        );
        let mut feature_sum: f64 = 0.0;
        lesson_trace::trace_step!(feature_sum);
        lesson_trace::trace_note!(
            "Повторяем следующий блок для каждого элемента указанной последовательности."
        );
        for &(feature_value, _) in training_records {
            lesson_trace::trace_step!(feature_value);
            lesson_trace::trace_note!(
                "Прибавляем очередной вклад к ранее накопленному результату."
            );
            feature_sum += feature_value;
            lesson_trace::trace_step!(feature_sum);
        }
        lesson_trace::trace_note!("Делим значения, получая нормированную величину или среднее.");
        feature_sum / training_records.len() as f64
    })();
    lesson_trace::trace_step!(training_mean);
    lesson_trace::trace_note!("Шаг: Применяем найденное среднее к тестовым значениям.");
    lesson_trace::trace_note!(
        "Задаём шаблон строки: плейсхолдеры ниже заменятся рассчитанными значениями."
    );
    lesson_trace::trace_note!(
        "Используем ранее рассчитанное значение `training_records` в текущем выражении."
    );
    lesson_trace::trace_note!(
        "Используем ранее рассчитанное значение `validation_records` в текущем выражении."
    );
    lesson_trace::trace_note!(
        "Используем ранее рассчитанное значение `test_records` в текущем выражении."
    );
    lesson_trace::trace_note!(
        "Используем ранее рассчитанное значение `test_records` в текущем выражении."
    );
    lesson_trace::trace_note!("Перебираем элементы по ссылке, не копируя исходную коллекцию.");
    lesson_trace::trace_note!("Преобразуем каждый элемент последовательности.");
    lesson_trace::trace_note!("Собираем полученные элементы в вектор.");
    println!(
        "train={:?}, validation={:?}, test={:?}, train mean={training_mean:.2}, normalized test={:?}",
        training_records,
        validation_records,
        test_records,
        test_records
            .iter()
            .map(|record| record.0 - training_mean)
            .collect::<Vec<_>>()
    );

    lesson_trace::trace_note!("Построение графика вынесено из основного кода урока.");
    lesson_trace::disable();
    plot_feature_means_for_training_data_and_whole_dataset(records, training_mean);
}

// Строим график по результатам урока.
fn plot_feature_means_for_training_data_and_whole_dataset(
    records: std::vec::Vec<(f64, u8)>,
    training_mean: f64,
) {
    lesson_trace::trace_note!("Сравниваем величины, вычисленные в примере.");
    lesson_trace::trace_note!("Передаём путь к каталогу текущего урока.");
    lesson_trace::trace_note!("Указываем имя SVG-файла.");
    lesson_trace::trace_note!("Указываем заголовок диаграммы.");
    lesson_trace::trace_note!("Указываем подпись вертикальной оси.");
    lesson_trace::trace_note!("Передаём ряды или значения для отрисовки графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Передаём подпись или текстовое значение для следующего шага.");
    lesson_trace::trace_note!("Вычисляем значение по указанной формуле.");
    lesson_trace::trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
    let chart: std::path::PathBuf = lesson_visualization::bar_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Среднее признака",
        "значение",
        &[
            ("train", training_mean),
            (
                "весь набор",
                records.iter().map(|record| record.0).sum::<f64>() / records.len() as f64,
            ),
        ],
    )
    .expect("не удалось сохранить график");
    lesson_trace::trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
