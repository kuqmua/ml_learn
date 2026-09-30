// Урок 29.4. Случайный выбор следующей части текста согласно вероятностям.
// Связь с принятой терминологией: Выбор следующего токена из распределения вероятностей.
// Зачем здесь эта тема: Вероятности ещё не создают текст; нужен способ выбрать конкретный следующий
//   токен.
// Почему код устроен так: Используем фиксированное число из [0, 1), чтобы вручную проследить
//   накопленные вероятности и выбор токена.
// Представь: При вероятностях [0,5; 0,3; 0,2] число 0,65 попадёт во второй накопленный интервал.
//
// Что изучаем: Выборка следующего токена.
// Зачем это нужно: Сэмплирование использует распределение вероятностей, а не всегда берёт самый вероятный
// токен.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    lesson_trace::enable();
    lesson_trace::trace_note!("Создаём набор значений `words` для следующего шага примера.");
    let words: [&str; 3] = ["кот", "пёс", "мир"];
    lesson_trace::trace_step!(words);
    lesson_trace::trace_note!(
        "Создаём набор значений `probabilities` для следующего шага примера."
    );
    let probabilities: [f64; 3] = [0.5, 0.3, 0.2];
    lesson_trace::trace_step!(probabilities);
    lesson_trace::trace_note!(
        "Инициализируем значение `random_number_between_zero_and_one` начальным состоянием."
    );
    lesson_trace::trace_note!(
        "Число от 0 до 1 задаёт долю единичного интервала; такую долю называют fraction."
    );
    let random_number_between_zero_and_one: f64 = 0.65;
    lesson_trace::trace_step!(random_number_between_zero_and_one);
    lesson_trace::trace_note!(
        "Инициализируем изменяемый накопитель `cumulative` начальным состоянием."
    );
    lesson_trace::trace_note!("Каждому слову соответствует одна неотрицательная вероятность.");
    lesson_trace::trace_note!("Используем подготовленное значение в следующем шаге примера.");
    lesson_trace::trace_note!("Используем подготовленное значение в следующем шаге примера.");
    lesson_trace::trace_note!("Передаём подпись или текстовое значение для следующего шага.");
    assert_eq!(
        words.len(),
        probabilities.len(),
        "каждому слову нужна вероятность"
    );
    lesson_trace::trace_note!("Проверяем ожидаемое свойство учебного примера.");
    assert!(!words.is_empty(), "для выбора нужно хотя бы одно слово");
    lesson_trace::trace_note!("Проверяем ожидаемое свойство учебного примера.");
    lesson_trace::trace_note!("Используем подготовленное значение в следующем шаге примера.");
    lesson_trace::trace_note!("Просматриваем элементы коллекции по ссылке.");
    lesson_trace::trace_note!("Настраиваем или преобразуем результат предыдущего шага.");
    lesson_trace::trace_note!("Передаём подпись или текстовое значение для следующего шага.");
    assert!(
        probabilities
            .iter()
            .all(|&value| value >= 0.0 && value.is_finite()),
        "вероятности должны быть конечными и неотрицательными"
    );
    lesson_trace::trace_note!("Проверяем ожидаемое свойство учебного примера.");
    lesson_trace::trace_note!(
        "Допуск 10⁻⁹ учитывает округление f64 при суммировании вероятностей до единицы."
    );
    lesson_trace::trace_note!("Передаём подпись или текстовое значение для следующего шага.");
    assert!(
        (probabilities.iter().sum::<f64>() - 1.0).abs() < 1e-9,
        "сумма вероятностей должна быть равна 1"
    );
    lesson_trace::trace_note!("Проверяем ожидаемое свойство учебного примера.");
    lesson_trace::trace_note!("Используем подготовленное значение в следующем шаге примера.");
    lesson_trace::trace_note!("Передаём подпись или текстовое значение для следующего шага.");
    assert!(
        (0.0..1.0).contains(&random_number_between_zero_and_one),
        "случайное число должно быть от 0 до 1, не включая 1"
    );
    lesson_trace::trace_note!("Сохраняем результат этого шага в `cumulative`.");
    let mut cumulative: f64 = 0.0;
    lesson_trace::trace_step!(cumulative);
    lesson_trace::trace_note!(
        "Повторяем следующий блок для каждого элемента указанной последовательности."
    );
    for index in 0..words.len() {
        lesson_trace::trace_step!(index);
        lesson_trace::trace_note!("Прибавляем очередной вклад к ранее накопленному результату.");
        cumulative += probabilities[index];
        lesson_trace::trace_step!(cumulative);
        lesson_trace::trace_note!("Проверяем условие и выбираем соответствующую ветку алгоритма.");
        if random_number_between_zero_and_one < cumulative {
            lesson_trace::trace_note!(
                "Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением."
            );
            println!("выбран токен {:?}", words[index]);
            lesson_trace::trace_note!("Останавливаем цикл после достижения условия завершения.");
            break;
        }
    }

    lesson_trace::trace_note!("Построение графика вынесено из основного кода урока.");
    lesson_trace::disable();
    plot_probabilities_of_possible_next_text_units(probabilities);
}

// Строим график по результатам урока.
fn plot_probabilities_of_possible_next_text_units(probabilities: [f64; 3]) {
    lesson_trace::trace_note!("Сравнение величин из этого урока.");
    lesson_trace::trace_note!("Передаём путь к каталогу текущего урока.");
    lesson_trace::trace_note!("Указываем имя SVG-файла.");
    lesson_trace::trace_note!("Указываем заголовок диаграммы.");
    lesson_trace::trace_note!("Указываем подпись вертикальной оси.");
    lesson_trace::trace_note!("Передаём ряды или значения для отрисовки графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
    let chart: std::path::PathBuf = lesson_visualization::bar_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Распределение следующего токена",
        "вероятность",
        &[
            ("токен 1", probabilities[0]),
            ("токен 2", probabilities[1]),
            ("токен 3", probabilities[2]),
        ],
    )
    .expect("не удалось сохранить график");
    lesson_trace::trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
