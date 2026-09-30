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
use lesson_trace::{disable, enable_tracing, trace_note, trace_step};

fn main() {
    enable_tracing();
    trace_note!("Создаём набор значений `labels` для следующего шага примера.");
    let labels: [bool; 5] = [false, false, true, false, true];
    trace_step!(labels);
    trace_note!("Проверяем ожидаемое свойство учебного примера.");
    assert!(!labels.is_empty(), "для baseline нужна хотя бы одна метка");
    trace_note!("Преобразуем входные данные и сохраняем полученную коллекцию в `positive_count`.");
    let positive_count: usize = labels.iter().filter(|&&label| label).count();
    trace_step!(positive_count);
    trace_note!("Считаем количество элементов и сохраняем его в `majority_label`.");
    let majority_label: bool = positive_count * 2 > labels.len();
    trace_step!(majority_label);
    trace_note!("Сохраняем рассчитанное значение `accuracy` для следующих операций.");
    trace_note!("Перебираем элементы по ссылке, не копируя исходную коллекцию.");
    trace_note!("Оставляем только элементы, прошедшие указанную проверку.");
    trace_note!("Подсчитываем число элементов после отбора.");
    trace_note!("Делим значения, получая нормированную величину или среднее.");
    let accuracy: f64 = labels
        .iter()
        .filter(|&&label| label == majority_label)
        .count() as f64
        / labels.len() as f64;
    trace_step!(accuracy);
    trace_note!("Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением.");
    println!("класс большинства={majority_label}, accuracy={accuracy:.2}");

    trace_note!("Построение графика вынесено из основного кода урока.");
    disable();
    plot_counts_of_positive_and_negative_training_labels(labels, positive_count);
}

// Строим график по результатам урока.
fn plot_counts_of_positive_and_negative_training_labels(labels: [bool; 5], positive_count: usize) {
    trace_note!("Сравниваем величины, вычисленные в примере.");
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
        "Классы для baseline",
        "объектов",
        &[
            ("отрицательные", (labels.len() - positive_count) as f64),
            ("положительные", positive_count as f64),
        ],
    )
    .expect("не удалось сохранить график");
    trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
