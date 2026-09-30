// Урок 23.3. Адаптивный шаг обучения: масштабирование по накопленным средним скоростей изменения ошибки и их квадратов.
// Связь с принятой терминологией: Обновление Adam по оценкам моментов градиента.
// Зачем здесь эта тема: Один масштаб шага для всех параметров не всегда подходит; Adam подстраивает
//   шаг по моментам градиента.
// Почему код устроен так: Отдельно считаем скользящие средние градиента и его квадрата с поправкой
//   на начальные шаги.
// Представь: Adam помнит и средний градиент, и средний квадрат градиента, чтобы менять масштаб
//   шага.
//
// Что изучаем: Обновление Adam.
// Зачем это нужно: Adam отслеживает сглаженные первый и второй моменты градиента, чтобы подстраивать
// размер шага по параметру.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    lesson_trace::enable();
    lesson_trace::trace_note!(
        "Сохраняем рассчитанное значение `rate_of_change` для следующих операций."
    );
    lesson_trace::trace_note!(
        "Производную функции по параметру или вектор таких производных называют gradient."
    );
    let rate_of_change: f64 = 2.0;
    lesson_trace::trace_step!(rate_of_change);
    lesson_trace::trace_note!(
        "β₁ = 0.9 сохраняет 90% прежнего среднего градиента; 0.1 добавляет новый градиент."
    );
    let first_moment: f64 = 0.9 * 0.0 + 0.1 * rate_of_change;
    lesson_trace::trace_step!(first_moment);
    lesson_trace::trace_note!(
        "β₂ = 0.999 сглаживает квадрат градиента сильнее: новый вклад равен 0.001."
    );
    let second_moment: f64 = 0.999 * 0.0 + 0.001 * rate_of_change * rate_of_change;
    lesson_trace::trace_step!(second_moment);
    lesson_trace::trace_note!(
        "После первого шага оба средних смещены к нулю; делим на 1−β, чтобы убрать это смещение."
    );
    let corrected_first: f64 = first_moment / (1.0 - 0.9);
    lesson_trace::trace_step!(corrected_first);
    lesson_trace::trace_note!(
        "Нормируем или усредняем величину делением и сохраняем её в `corrected_second`."
    );
    let corrected_second: f64 = second_moment / (1.0 - 0.999);
    lesson_trace::trace_step!(corrected_second);
    lesson_trace::trace_note!("Создаём изменяемое значение `root` для следующих операций.");
    let mut root: f64 = corrected_second;
    lesson_trace::trace_step!(root);
    lesson_trace::trace_note!(
        "80 шагов Ньютона вычисляют √исправленного второго момента с запасом для f64."
    );
    for _ in 0..80 {
        lesson_trace::trace_note!(
            "Среднее root и corrected_second/root приближает искомый квадратный корень."
        );
        root = (root + corrected_second / root) / 2.0;
        lesson_trace::trace_step!(root);
    }
    lesson_trace::trace_note!(
        "Сохраняем рассчитанное значение `old_weight` для следующих операций."
    );
    let old_weight: f64 = 1.0;
    lesson_trace::trace_step!(old_weight);
    lesson_trace::trace_note!(
        "0.01 — скорость обучения; 10⁻⁸ в знаменателе защищает от деления на ноль."
    );
    let updated: f64 = old_weight - 0.01 * corrected_first / (root + 0.00000001);
    lesson_trace::trace_step!(updated);
    lesson_trace::trace_note!(
        "Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением."
    );
    println!("вес после первого шага Adam = {updated}");

    lesson_trace::trace_note!("Построение графика вынесено из основного кода урока.");
    lesson_trace::disable();
    plot_first_weight_update_using_running_gradient_averages(
        rate_of_change,
        corrected_first,
        old_weight,
        updated,
    );
}

// Строим график по результатам урока.
fn plot_first_weight_update_using_running_gradient_averages(
    rate_of_change: f64,
    corrected_first: f64,
    old_weight: f64,
    updated: f64,
) {
    lesson_trace::trace_note!("Сравниваем величины, вычисленные в примере.");
    lesson_trace::trace_note!("Передаём путь к каталогу текущего урока.");
    lesson_trace::trace_note!("Указываем имя SVG-файла.");
    lesson_trace::trace_note!("Указываем заголовок диаграммы.");
    lesson_trace::trace_note!("Указываем подпись вертикальной оси.");
    lesson_trace::trace_note!("Передаём ряды или значения для отрисовки графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
    let chart: std::path::PathBuf = lesson_visualization::bar_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Adam: первый шаг",
        "значение",
        &[
            ("градиент", rate_of_change),
            ("первый момент", corrected_first),
            ("вес до", old_weight),
            ("вес после", updated),
        ],
    )
    .expect("не удалось сохранить график");
    lesson_trace::trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
