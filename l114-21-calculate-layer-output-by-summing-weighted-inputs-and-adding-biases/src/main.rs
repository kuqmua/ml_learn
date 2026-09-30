// Урок 21.1. Выход слоя нейросети: сложение взвешенных входов и прибавление смещений.
// Связь с принятой терминологией: Преобразование входного вектора слоем нейронной сети.
// Зачем здесь эта тема: Нейронный слой обобщает линейное преобразование из блока матриц на много
//   входов и выходов.
// Почему код устроен так: Начинаем с малых векторов и явных весов, чтобы видеть вклад каждой
//   координаты.
// Представь: Один нейрон берёт несколько входных чисел и превращает их в одно выходное через веса.
//
// Что изучаем: Слои нейронной сети.
// Зачем это нужно: Слой преобразует входной вектор в выходной по весам и смещениям каждого нейрона.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    lesson_trace::enable();
    lesson_trace::trace_note!("Создаём набор значений `input` для следующего шага примера.");
    let input: [f64; 2] = [1.0, 2.0];
    lesson_trace::trace_step!(input);
    lesson_trace::trace_note!("Создаём набор значений `weights` для следующего шага примера.");
    let weights: [[f64; 2]; 2] = [[0.5, 0.2], [-0.3, 0.8]];
    lesson_trace::trace_step!(weights);
    lesson_trace::trace_note!("Создаём набор значений `biases` для следующего шага примера.");
    let biases: [f64; 2] = [0.1, -0.2];
    lesson_trace::trace_step!(biases);
    lesson_trace::trace_note!("Создаём набор значений `output` для следующего шага примера.");
    let mut output: [f64; 2] = [0.0; 2];
    lesson_trace::trace_step!(output);
    lesson_trace::trace_note!(
        "Повторяем следующий блок для каждого элемента указанной последовательности."
    );
    for neuron in 0..2 {
        lesson_trace::trace_step!(neuron);
        lesson_trace::trace_note!(
            "Присваиваем вычисленное значение соответствующей переменной или полю."
        );
        output[neuron] = biases[neuron];
        lesson_trace::trace_step!(output);
        lesson_trace::trace_note!(
            "Повторяем следующий блок для каждого элемента указанной последовательности."
        );
        for feature in 0..2 {
            lesson_trace::trace_step!(feature);
            lesson_trace::trace_note!(
                "Прибавляем очередной вклад к ранее накопленному результату."
            );
            output[neuron] += weights[neuron][feature] * input[feature];
            lesson_trace::trace_step!(output);
        }
    }
    lesson_trace::trace_note!(
        "Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением."
    );
    println!("выход слоя = {output:?}");

    lesson_trace::trace_note!("Построение графика вынесено из основного кода урока.");
    lesson_trace::disable();
    plot_weights_used_to_sum_input_coordinates(weights);
}

// Строим график по результатам урока.
fn plot_weights_used_to_sum_input_coordinates(weights: [[f64; 2]; 2]) {
    lesson_trace::trace_note!("Значения ячеек видны по цвету и подписи.");
    lesson_trace::trace_note!("Передаём путь к каталогу текущего урока.");
    lesson_trace::trace_note!("Указываем имя SVG-файла.");
    lesson_trace::trace_note!("Указываем заголовок тепловой карты.");
    lesson_trace::trace_note!("Используем подготовленное значение в следующем шаге примера.");
    lesson_trace::trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
    let chart: std::path::PathBuf = lesson_visualization::heatmap(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Веса слоя",
        &weights.iter().map(|row| row.to_vec()).collect::<Vec<_>>(),
    )
    .expect("не удалось сохранить тепловую карту");
    lesson_trace::trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
