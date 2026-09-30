// Урок 04.3. Сравнение минимумов функции при движении из разных начальных точек.
// Связь с принятой терминологией: Локальные минимумы невыпуклой функции.
// Зачем здесь эта тема: Для невыпуклой ошибки разные начальные точки могут привести к разным
//   минимумам.
// Почему код устроен так: Показываем несколько впадин, чтобы не принимать найденный минимум за
//   глобальный.
// Представь: Шарик в рельефе с двумя ямами останется в ближайшей яме, хотя другая может быть
//   глубже.
//
// Что изучаем: Локальные минимумы.
// Зачем это нужно: У невыпуклой функции разные начальные точки могут привести к разным минимумам.
// Исследуем f(x)=x⁴−2x².

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    lesson_trace::enable();
    lesson_trace::trace_note!(
        "Повторяем следующий блок для каждого элемента указанной последовательности."
    );
    for start in [-0.5, 0.5] {
        lesson_trace::trace_step!(start);
        lesson_trace::trace_note!(
            "Создаём изменяемое значение `input_value` для следующих операций."
        );
        let mut input_value: f64 = start;
        lesson_trace::trace_step!(input_value);
        lesson_trace::trace_note!(
            "Делаем 100 шагов, чтобы оба старта успели приблизиться к своим локальным минимумам."
        );
        lesson_trace::trace_note!(
            "Число шагов выбрано для демонстрации; 0.1 ниже — длина одного шага против градиента."
        );
        for _ in 0..100 {
            lesson_trace::trace_note!("Производная равна 4x³−4x; шагаем против её знака.");
            lesson_trace::trace_note!(
                "Производную функции по параметру или вектор таких производных называют gradient."
            );
            let rate_of_change: f64 =
                4.0 * input_value * input_value * input_value - 4.0 * input_value;
            lesson_trace::trace_step!(rate_of_change);
            lesson_trace::trace_note!("Вычитаем очередной вклад из текущего значения параметра.");
            input_value -= 0.1 * rate_of_change;
            lesson_trace::trace_step!(input_value);
        }
        lesson_trace::trace_note!("Умножаем значения и сохраняем результат в `loss`.");
        let loss: f64 =
            input_value * input_value * input_value * input_value - 2.0 * input_value * input_value;
        lesson_trace::trace_step!(loss);
        lesson_trace::trace_note!(
            "Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением."
        );
        println!("старт={start}, найдено x={input_value:.3}, f(x)={loss:.3}");
    }

    lesson_trace::trace_note!("Построение графика вынесено из основного кода урока.");
    lesson_trace::disable();
    plot_function_with_two_valleys();
}

// Строим график по результатам урока.
fn plot_function_with_two_valleys() {
    lesson_trace::trace_note!("Наглядное представление величин из этого урока.");
    lesson_trace::trace_note!("Преобразуем каждый элемент в новое значение.");
    lesson_trace::trace_note!("Собираем результаты в коллекцию.");
    let local_minima_points: Vec<(f64, f64)> = (-150..=150)
        .map(|plot_step_index| {
            lesson_trace::trace_note!("Сохраняем результат этого шага в `horizontal_value`.");
            let horizontal_value: f64 = plot_step_index as f64 / 100.0;
            lesson_trace::trace_note!(
                "Добавляем пару значений для сравнения или построения графика."
            );
            (
                horizontal_value,
                horizontal_value * horizontal_value * horizontal_value * horizontal_value
                    - 2.0 * horizontal_value * horizontal_value,
            )
        })
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
        "Два локальных минимума",
        "x",
        "f(x)",
        &[lesson_visualization::Series {
            name: "x⁴−2x²",

            points: &local_minima_points,
        }],
    )
    .expect("не удалось сохранить график");
    lesson_trace::trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
