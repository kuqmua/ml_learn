// Сводная практика 05. Условная вероятность, формула Байеса и ожидание.
// Зачем здесь эта тема: Симуляция даёт независимую проверку условной вероятности и формулы Байеса.
// Почему код устроен так: Фиксируем seed и считаем частоты событий, чтобы результат можно было
//   воспроизвести и сравнить с теорией.
// Представь: Формула даёт ожидаемую долю положительных тестов; тысячи повторов должны дать близкую
//   частоту.
//
// Что повторяем вместе: условная вероятность, независимость, формула Байеса, математическое ожидание.
// Зачем это нужно: Вероятность результата теста зависит от базовой частоты события; моделирование помогает
//   проверить расчёт по формуле Байеса.
// Что показывает программа: Фиксируем начальное состояние генератора для повторяемого моделирования.
//   Моделируем монету, истинное заболевание и результат теста. Сравниваем условную частоту болезни с
//   формулой Байеса.
// Что проверить при изменении примера: Сравни частоты с теоретическими значениями и объясни влияние базовой
//   частоты события.
// Дополнительная практика: Смоделируй броски монеты и тест болезни с известной чувствительностью и
//   специфичностью.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    lesson_trace::enable();
    // Описываем тип `PseudorandomGenerator`, чтобы явно хранить состояние и допустимые варианты.
    #[derive(Debug)]
    struct PseudorandomGenerator(u64);

    // Группируем методы рядом с типом, к которому они относятся.
    impl PseudorandomGenerator {
        // Объявляем повторно используемое вычисление `generate_random_number_between_zero_and_one`; параметры ниже задают его входы.
        // Случайное число от 0 до 1 задаёт долю единичного интервала; такую долю называют fraction.
        fn generate_random_number_between_zero_and_one(&mut self) -> f64 {
            // Линейный конгруэнтный генератор: фиксированные множитель и +1 меняют состояние по mod 2⁶⁴.
            // Следующий шаг берёт старшие 53 бита и делит на 2⁵³, получая число из [0, 1).
            self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1);
            // Составляем результат из вычисленных значений в указанном порядке.
            (self.0 >> 11) as f64 / (1u64 << 53) as f64
        }
    }

    // Шаг: Фиксируем начальное состояние генератора для повторяемого моделирования.
    let mut generator: PseudorandomGenerator = PseudorandomGenerator(42);
    lesson_trace::trace_step!(generator);
    // Инициализируем изменяемый накопитель `positive_test_count` начальным состоянием.
    let mut positive_test_count: i32 = 0;
    lesson_trace::trace_step!(positive_test_count);
    // Инициализируем изменяемый накопитель `true_positive_count` начальным состоянием.
    let mut true_positive_count: i32 = 0;
    lesson_trace::trace_step!(true_positive_count);
    // Инициализируем изменяемый накопитель `heads_count` начальным состоянием.
    let mut heads_count: i32 = 0;
    lesson_trace::trace_step!(heads_count);
    // 100 000 псевдослучайных опытов уменьшают колебания оценённых частот.
    // Это размер учебной симуляции, а не параметр формулы Байеса.
    for _ in 0..100_000 {
        // Для честной монеты событие «орёл» занимает половину интервала [0, 1).
        if generator.generate_random_number_between_zero_and_one() < 0.5 {
            // Прибавляем очередной вклад к ранее накопленному результату.
            heads_count += 1;
            lesson_trace::trace_step!(heads_count);
        }
        // 0.01 — заданная для примера распространённость: заболевание есть примерно у 1% людей.
        let has_disease: bool = generator.generate_random_number_between_zero_and_one() < 0.01;
        lesson_trace::trace_step!(has_disease);
        // Сохраняем рассчитанное значение `test_is_positive` для следующих операций.
        let test_is_positive: bool = if has_disease {
            // Для больного моделируем положительный тест с чувствительностью 90%.
            generator.generate_random_number_between_zero_and_one() < 0.9
        // Обрабатываем случай, когда предыдущее условие не выполнено.
        } else {
            // Для здорового моделируем ложноположительный тест с вероятностью 5%.
            generator.generate_random_number_between_zero_and_one() < 0.05
        };
        lesson_trace::trace_step!(test_is_positive);
        // Проверяем условие и выбираем соответствующую ветку алгоритма.
        if test_is_positive {
            // Прибавляем очередной вклад к ранее накопленному результату.
            positive_test_count += 1;
            lesson_trace::trace_step!(positive_test_count);
            // Проверяем условие и выбираем соответствующую ветку алгоритма.
            if has_disease {
                // Прибавляем очередной вклад к ранее накопленному результату.
                true_positive_count += 1;
                lesson_trace::trace_step!(true_positive_count);
            }
        }
    }

    // Шаг: Сравниваем условную частоту болезни с формулой Байеса.
    println!(
        // Задаём шаблон строки: плейсхолдеры ниже заменятся рассчитанными значениями.
        "heads={heads_count}, P(ill|positive): simulation={:.3}, Bayes={:.3}",
        // Возвращаем булев результат для этого случая.
        true_positive_count as f64 / positive_test_count as f64,
        // Составляем результат из вычисленных значений в указанном порядке.
        (|| -> f64 {
            // Используем подготовленное значение в следующем шаге примера.
            /* Формула Байеса учитывает и качество теста, и редкость болезни. */
            // Сохраняем результат этого шага в `prevalence`.
            let prevalence: f64 = 0.01;
            lesson_trace::trace_step!(prevalence);
            lesson_trace::trace_step!(prevalence);
            // Инициализируем значение `sensitivity` начальным состоянием.
            let sensitivity: f64 = 0.9;
            lesson_trace::trace_step!(sensitivity);
            lesson_trace::trace_step!(sensitivity);
            // Инициализируем значение `specificity` начальным состоянием.
            let specificity: f64 = 0.95;
            lesson_trace::trace_step!(specificity);
            lesson_trace::trace_step!(specificity);
            // Умножаем значения и сохраняем результат в `true_positive_probability`.
            let true_positive_probability: f64 = prevalence * sensitivity;
            lesson_trace::trace_step!(true_positive_probability);
            lesson_trace::trace_step!(true_positive_probability);
            // Возвращаем булев результат для этого случая.
            true_positive_probability
                // Делим значения, получая нормированную величину или среднее.
                / (true_positive_probability + (1.0 - prevalence) * (1.0 - specificity))
        })()
    );

    // Построение графика вынесено из основного кода урока.
    lesson_trace::disable();
    visualize_practice_conditional_probability_bayes_and_expected_value(
        positive_test_count,
        true_positive_count,
    );
}

// Строим график по результатам урока.
fn visualize_practice_conditional_probability_bayes_and_expected_value(
    positive_test_count: i32,
    true_positive_count: i32,
) {
    // Значения из этого урока на графике.
    let theoretical_probability_points: Vec<(f64, f64)> = [
        // Добавляем пару значений для сравнения или построения графика.
        (0.0, 0.01 * 0.9 / (0.01 * 0.9 + 0.99 * 0.05)),
        // Добавляем пару значений для сравнения или построения графика.
        (100000.0, 0.01 * 0.9 / (0.01 * 0.9 + 0.99 * 0.05)),
    ]
    // Настраиваем или преобразуем результат предыдущего шага.
    .to_vec();
    // Собираем значения для `simulated_probability_points` в коллекцию.
    let simulated_probability_points: Vec<(f64, f64)> = [(
        // Используем подготовленное значение в следующем шаге примера.
        100000.0,
        // Вычисляем значение по указанной формуле.
        true_positive_count as f64 / positive_test_count as f64,
    )]
    // Настраиваем или преобразуем результат предыдущего шага.
    .to_vec();
    // Строим график по рассчитанным значениям и сохраняем его как SVG.
    let chart: std::path::PathBuf = lesson_visualization::line_chart(
        // Передаём путь к каталогу текущего урока.
        env!("CARGO_MANIFEST_DIR"),
        // Указываем имя SVG-файла.
        "lesson-chart",
        // Указываем заголовок графика.
        "Монте-Карло и формула Байеса",
        // Указываем подпись горизонтальной оси.
        "число испытаний",
        // Указываем подпись вертикальной оси.
        "частота заболевания после положительного теста",
        // Передаём ряды или значения для отрисовки графика.
        &[
            // Добавляем ряд данных с подписью к графику.
            lesson_visualization::Series {
                // Указываем подпись этого ряда в легенде.
                name: "теория",
                // Передаём рассчитанные координаты точек.
                points: &theoretical_probability_points,
            },
            // Добавляем ряд данных с подписью к графику.
            lesson_visualization::Series {
                // Указываем подпись этого ряда в легенде.
                name: "симуляция",
                // Передаём рассчитанные координаты точек.
                points: &simulated_probability_points,
            },
        ],
    )
    // Прерываем пример с понятной ошибкой, если SVG не удалось записать.
    .expect("не удалось сохранить график");
    // Печатаем путь к созданному SVG, чтобы его можно было открыть.
    println!("график: {}", chart.display());
}
