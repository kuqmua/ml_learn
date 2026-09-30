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
use lesson_trace::{disable, enable, trace_note, trace_step};

fn main() {
    enable();
    trace_note!("Фиксируем демонстрационные данные на время выполнения программы.");
    const SAMPLE_COMMA_SEPARATED_VALUES: &str =
        "feature,target\n1,0\n2,0\n3,1\n4,1\n5,0\n6,1\n7,0\n8,1\n9,1\n10,0\n";

    trace_note!("Шаг: Читаем учебную таблицу признаков и меток.");
    let records: Vec<(f64, u8)> = (|| -> Vec<(f64, u8)> {
        trace_note!("Используем подготовленное значение в следующем шаге примера.");
        trace_note!("Разбираем демонстрационный CSV в пары «признак, метка».");
        trace_note!("Используем подготовленное значение в следующем шаге примера.");
        trace_note!("Разбиваем текст на строки для последовательной обработки.");
        trace_note!("Пропускаем первые элементы, которые не входят в полезные данные.");
        trace_note!("Преобразуем каждый элемент последовательности.");
        trace_note!("Собираем элементы итератора в итоговую коллекцию.");
        SAMPLE_COMMA_SEPARATED_VALUES

            .lines()

            .skip(1)

            .map(|line| {
                trace_note!("Сохраняем рассчитанное значение `(feature_value, target_value)` для следующих операций.");
                let (feature_value, target_value): (&str, &str) = line.split_once(',').unwrap();
                trace_step!(feature_value);
                trace_step!(target_value);
                trace_note!("Составляем результат из вычисленных значений в указанном порядке.");
                trace_note!("Преобразуем текстовое поле в требуемый числовой тип.");
                trace_note!("Преобразуем текстовое поле в требуемый числовой тип.");
                (

                    feature_value.parse().unwrap(),

                    target_value.parse().unwrap(),
                )
            })

            .collect()
    })();
    trace_step!(records);
    trace_note!("Шаг: Отделяем обучающую, проверочную и тестовую части.");
    trace_note!("Обновляем значение результатом текущего вычисления.");
    trace_note!("Передаём подпись или текстовое значение для следующего шага.");
    assert!(
        records.len() >= 9,
        "для разделения нужны строки train, validation и test"
    );
    trace_note!("Сохраняем результат этого шага в `training_records`.");
    let training_records: &[(f64, u8)] = &records[..6];
    trace_step!(training_records);
    trace_note!("Сохраняем рассчитанное значение `validation_records` для следующих операций.");
    let validation_records: &[(f64, u8)] = &records[6..8];
    trace_step!(validation_records);
    trace_note!("Сохраняем рассчитанное значение `test_records` для следующих операций.");
    let test_records: &[(f64, u8)] = &records[8..];
    trace_step!(test_records);

    trace_note!(
        "Шаг: Считаем среднее только по train: validation и test не влияют на подготовку признаков."
    );
    let training_mean: f64 = (|| -> f64 {
        trace_note!("Используем подготовленное значение в следующем шаге примера.");
        trace_note!("Среднее признака считаем только по обучающим строкам.");
        trace_note!("Сохраняем результат этого шага в `training_records`.");
        let training_records: &[(f64, u8)] = training_records;
        trace_step!(training_records);
        trace_note!("Инициализируем изменяемый накопитель `feature_sum` начальным состоянием.");
        let mut feature_sum: f64 = 0.0;
        trace_step!(feature_sum);
        trace_note!("Повторяем следующий блок для каждого элемента указанной последовательности.");
        for &(feature_value, _) in training_records {
            trace_step!(feature_value);
            trace_note!("Прибавляем очередной вклад к ранее накопленному результату.");
            feature_sum += feature_value;
            trace_step!(feature_sum);
        }
        trace_note!("Делим значения, получая нормированную величину или среднее.");
        feature_sum / training_records.len() as f64
    })();
    trace_step!(training_mean);
    trace_note!("Шаг: Применяем найденное среднее к тестовым значениям.");
    trace_note!("Задаём шаблон строки: плейсхолдеры ниже заменятся рассчитанными значениями.");
    trace_note!("Используем ранее рассчитанное значение `training_records` в текущем выражении.");
    trace_note!("Используем ранее рассчитанное значение `validation_records` в текущем выражении.");
    trace_note!("Используем ранее рассчитанное значение `test_records` в текущем выражении.");
    trace_note!("Используем ранее рассчитанное значение `test_records` в текущем выражении.");
    trace_note!("Перебираем элементы по ссылке, не копируя исходную коллекцию.");
    trace_note!("Преобразуем каждый элемент последовательности.");
    trace_note!("Собираем полученные элементы в вектор.");
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

    trace_note!("Построение графика вынесено из основного кода урока.");
    disable();
    plot_feature_means_for_training_data_and_whole_dataset(records, training_mean);
}

// Строим график по результатам урока.
fn plot_feature_means_for_training_data_and_whole_dataset(
    records: std::vec::Vec<(f64, u8)>,
    training_mean: f64,
) {
    trace_note!("Сравниваем величины, вычисленные в примере.");
    trace_note!("Передаём путь к каталогу текущего урока.");
    trace_note!("Указываем имя SVG-файла.");
    trace_note!("Указываем заголовок диаграммы.");
    trace_note!("Указываем подпись вертикальной оси.");
    trace_note!("Передаём ряды или значения для отрисовки графика.");
    trace_note!("Добавляем пару значений для сравнения или построения графика.");
    trace_note!("Передаём подпись или текстовое значение для следующего шага.");
    trace_note!("Вычисляем значение по указанной формуле.");
    trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
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
    trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
