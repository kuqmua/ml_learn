// Урок 13.3. Защита от нулевых вероятностей слов: добавление единицы к частотам.
// Связь с принятой терминологией: Сглаживание Лапласа для частот слов при известном классе.
// Зачем здесь эта тема: Нулевое число наблюдений слова не должно обнулять вероятность всего текста.
// Почему код устроен так: Добавляем псевдосчётчик к частотам и пересчитываем знаменатель для
//   каждого класса.
// Представь: Если слово ни разу не встретилось в классе, добавление единицы не даёт вероятности
//   стать нулём.
//
// Что изучаем: Сглаживание Лапласа.
// Зачем это нужно: Прибавление единицы к частотам не даёт неизвестному слову обнулить вероятность всего
// текста.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
use lesson_trace::{disable, enable, trace_note, trace_step};

fn main() {
    enable();
    trace_note!("Инициализируем значение `observed_count` начальным состоянием.");
    let observed_count: f64 = 0.0;
    trace_step!(observed_count);
    trace_note!("Сохраняем рассчитанное значение `total_words_in_class` для следующих операций.");
    let total_words_in_class: f64 = 8.0;
    trace_step!(total_words_in_class);
    trace_note!("Сохраняем рассчитанное значение `known_text_unit_count` для следующих операций.");
    trace_note!("Набор известных модели текстовых единиц называют vocabulary.");
    let known_text_unit_count: f64 = 4.0;
    trace_step!(known_text_unit_count);
    trace_note!("Нормируем или усредняем величину делением и сохраняем её в `unsmoothed`.");
    let unsmoothed: f64 = observed_count / total_words_in_class;
    trace_step!(unsmoothed);
    trace_note!("Нормируем или усредняем величину делением и сохраняем её в `smoothed`.");
    let smoothed: f64 = (observed_count + 1.0) / (total_words_in_class + known_text_unit_count);
    trace_step!(smoothed);
    trace_note!("Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением.");
    println!("без сглаживания={unsmoothed}, со сглаживанием={smoothed}");

    trace_note!("Построение графика вынесено из основного кода урока.");
    disable();
    plot_word_probabilities_before_and_after_adding_one_to_counts();
}

// Строим график по результатам урока.
fn plot_word_probabilities_before_and_after_adding_one_to_counts() {
    trace_note!("График величин и зависимостей, изученных в этом уроке.");
    let unsmoothed_probability_points: Vec<(f64, f64)> = (0..=8)
        .map(|sample_count| (sample_count as f64, sample_count as f64 / 10.0))
        .collect();
    trace_note!("Собираем значения для `smoothed_probability_points` в коллекцию.");
    trace_note!("Преобразуем каждый элемент в новое значение.");
    trace_note!("Собираем результаты в коллекцию.");
    let smoothed_probability_points: Vec<(f64, f64)> = (0..=8)
        .map(|sample_count| (sample_count as f64, (sample_count as f64 + 1.0) / 12.0))
        .collect();
    trace_note!("Строим график по рассчитанным значениям и сохраняем его как SVG.");
    trace_note!("Передаём путь к каталогу текущего урока.");
    trace_note!("Указываем имя SVG-файла.");
    trace_note!("Указываем заголовок графика.");
    trace_note!("Указываем подпись горизонтальной оси.");
    trace_note!("Указываем подпись вертикальной оси.");
    trace_note!("Передаём ряды или значения для отрисовки графика.");
    trace_note!("Добавляем ряд данных с подписью к графику.");
    trace_note!("Указываем подпись этого ряда в легенде.");
    trace_note!("Передаём рассчитанные координаты точек.");
    trace_note!("Добавляем ряд данных с подписью к графику.");
    trace_note!("Указываем подпись этого ряда в легенде.");
    trace_note!("Передаём рассчитанные координаты точек.");
    trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
    let chart: std::path::PathBuf = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Сглаживание Лапласа",
        "частота токена",
        "оценка вероятности",
        &[
            lesson_visualization::Series {
                name: "без сглаживания",

                points: &unsmoothed_probability_points,
            },
            lesson_visualization::Series {
                name: "со сглаживанием",

                points: &smoothed_probability_points,
            },
        ],
    )
    .expect("не удалось сохранить график");
    trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
