// Урок 14.2. Смешанность классов (мера Джини): единица минус сумма квадратов долей классов.
// Связь с принятой терминологией: Нечистота Джини по долям классов в узле дерева решений.
// Зачем здесь эта тема: Нечистота Джини даёт другой простой критерий выбора разбиения дерева.
// Почему код устроен так: Сравниваем суммы квадратов долей с энтропией на одинаковых составах узла.
// Представь: Чистый узел имеет Джини 0; для равных долей двух классов нечистота выше.
//
// Что изучаем: Нечистота Gini.
// Зачем это нужно: Gini равна единице минус сумма квадратов долей классов; чистому узлу соответствует
// ноль.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
use lesson_trace::{disable, enable, trace_note, trace_step};

fn main() {
    enable();
    trace_note!("Повторяем следующий блок для каждого элемента указанной последовательности.");
    trace_note!("Долю объектов одного класса среди всех объектов называют fraction.");
    for positive_class_share in [0.0, 0.5, 1.0] {
        trace_step!(positive_class_share);
        trace_note!(
            "Комбинируем исходные величины и сохраняем результат в `negative_class_share`."
        );
        let negative_class_share: f64 = 1.0 - positive_class_share;
        trace_step!(negative_class_share);
        trace_note!("Сохраняем рассчитанное значение `gini` для следующих операций.");
        trace_note!("Умножаем величины согласно используемой формуле.");
        let gini: f64 = 1.0
            - positive_class_share * positive_class_share
            - negative_class_share * negative_class_share;
        trace_step!(gini);
        trace_note!("Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением.");
        println!("доля положительных={positive_class_share}, Gini={gini}");
    }

    trace_note!("Построение графика вынесено из основного кода урока.");
    disable();
    plot_class_mixing_as_twice_positive_share_times_negative_share();
}

// Строим график по результатам урока.
fn plot_class_mixing_as_twice_positive_share_times_negative_share() {
    trace_note!("График величин и зависимостей, изученных в этом уроке.");
    trace_note!("Преобразуем каждый элемент в новое значение.");
    trace_note!("Собираем результаты в коллекцию.");
    let gini_points: Vec<(f64, f64)> = (0..=100)
        .map(|plot_step_index| {
            trace_note!("Сохраняем результат этого шага в `probability`.");
            let probability: f64 = plot_step_index as f64 / 100.0;
            trace_note!("Добавляем пару значений для сравнения или построения графика.");
            (probability, 2.0 * probability * (1.0 - probability))
        })
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
        "Нечистота Gini",
        "доля положительных",
        "Gini",
        &[lesson_visualization::Series {
            name: "Gini(p)",

            points: &gini_points,
        }],
    )
    .expect("не удалось сохранить график");
    trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
