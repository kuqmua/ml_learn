// Урок 08.2. Простая модель для сравнения: прогноз самого частого обучающего класса.
// Связь с принятой терминологией: Базовый классификатор по наиболее частому классу.
// Зачем здесь эта тема: Сложной модели нужна точка отсчёта; большинство классов даёт простой
//   baseline.
// Почему код устроен так: Считаем частоты только на train и сравниваем более сложные модели с этим
//   прогнозом.
// Представь: Если 80% примеров относятся к классу A, постоянный ответ A уже даёт 80% accuracy;
//   модель должна превзойти его.
//
// Что изучаем: Базовая модель.
// Зачем это нужно: Baseline задаёт простую исходную точку. Классификатор большинства всегда предсказывает
// самый частый класс.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    lesson_trace::enable();
    lesson_trace::trace_note!("Создаём набор значений `labels` для следующего шага примера.");
    let labels: [bool; 5] = [false, false, true, false, true];
    lesson_trace::trace_step!(labels);
    lesson_trace::trace_note!("Проверяем ожидаемое свойство учебного примера.");
    assert!(!labels.is_empty(), "для baseline нужна хотя бы одна метка");
    lesson_trace::trace_note!(
        "Преобразуем входные данные и сохраняем полученную коллекцию в `positive_count`."
    );
    let positive_count: usize = labels.iter().filter(|&&label| label).count();
    lesson_trace::trace_step!(positive_count);
    lesson_trace::trace_note!("Считаем количество элементов и сохраняем его в `majority_label`.");
    let majority_label: bool = positive_count * 2 > labels.len();
    lesson_trace::trace_step!(majority_label);
    lesson_trace::trace_note!("Сохраняем рассчитанное значение `accuracy` для следующих операций.");
    lesson_trace::trace_note!("Перебираем элементы по ссылке, не копируя исходную коллекцию.");
    lesson_trace::trace_note!("Оставляем только элементы, прошедшие указанную проверку.");
    lesson_trace::trace_note!("Подсчитываем число элементов после отбора.");
    lesson_trace::trace_note!("Делим значения, получая нормированную величину или среднее.");
    let accuracy: f64 = labels
        .iter()
        .filter(|&&label| label == majority_label)
        .count() as f64
        / labels.len() as f64;
    lesson_trace::trace_step!(accuracy);
    lesson_trace::trace_note!(
        "Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением."
    );
    println!("класс большинства={majority_label}, accuracy={accuracy:.2}");

    lesson_trace::trace_note!("Построение графика вынесено из основного кода урока.");
    lesson_trace::disable();
    plot_counts_of_positive_and_negative_training_labels(labels, positive_count);
}

// Строим график по результатам урока.
fn plot_counts_of_positive_and_negative_training_labels(labels: [bool; 5], positive_count: usize) {
    lesson_trace::trace_note!("Сравниваем величины, вычисленные в примере.");
    lesson_trace::trace_note!("Передаём путь к каталогу текущего урока.");
    lesson_trace::trace_note!("Указываем имя SVG-файла.");
    lesson_trace::trace_note!("Указываем заголовок диаграммы.");
    lesson_trace::trace_note!("Указываем подпись вертикальной оси.");
    lesson_trace::trace_note!("Передаём ряды или значения для отрисовки графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
    let chart: std::path::PathBuf = lesson_visualization::bar_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Классы для baseline",
        "объектов",
        &[
            ("отрицательные", (labels.len() - positive_count) as f64),
            ("положительные", positive_count as f64),
        ],
    )
    .expect("не удалось сохранить график");
    lesson_trace::trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
