// Урок 05.5. Практика: пересчёт вероятностей и сумма исходов с весами по вероятности.
// Связь с принятой терминологией: Условная вероятность, формула Байеса и ожидание.
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
    lesson_trace::trace_note!(
        "Описываем тип `PseudorandomGenerator`, чтобы явно хранить состояние и допустимые варианты."
    );
    #[derive(Debug)]
    struct PseudorandomGenerator(u64);

    lesson_trace::trace_note!("Группируем методы рядом с типом, к которому они относятся.");
    lesson_trace::trace_note!(
        "Объявляем повторно используемое вычисление `generate_random_number_between_zero_and_one`; параметры ниже задают его входы."
    );
    lesson_trace::trace_note!(
        "Случайное число от 0 до 1 задаёт долю единичного интервала; такую долю называют fraction."
    );
    impl PseudorandomGenerator {
        fn generate_random_number_between_zero_and_one(&mut self) -> f64 {
            lesson_trace::trace_note!(
                "Линейный конгруэнтный генератор: фиксированные множитель и +1 меняют состояние по mod 2⁶⁴."
            );
            lesson_trace::trace_note!(
                "Следующий шаг берёт старшие 53 бита и делит на 2⁵³, получая число из [0, 1)."
            );
            self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1);
            lesson_trace::trace_note!(
                "Составляем результат из вычисленных значений в указанном порядке."
            );
            (self.0 >> 11) as f64 / (1u64 << 53) as f64
        }
    }

    lesson_trace::trace_note!(
        "Шаг: Фиксируем начальное состояние генератора для повторяемого моделирования."
    );
    let mut generator: PseudorandomGenerator = PseudorandomGenerator(42);
    lesson_trace::trace_step!(generator);
    lesson_trace::trace_note!(
        "Инициализируем изменяемый накопитель `positive_test_count` начальным состоянием."
    );
    let mut positive_test_count: i32 = 0;
    lesson_trace::trace_step!(positive_test_count);
    lesson_trace::trace_note!(
        "Инициализируем изменяемый накопитель `true_positive_count` начальным состоянием."
    );
    let mut true_positive_count: i32 = 0;
    lesson_trace::trace_step!(true_positive_count);
    lesson_trace::trace_note!(
        "Инициализируем изменяемый накопитель `heads_count` начальным состоянием."
    );
    let mut heads_count: i32 = 0;
    lesson_trace::trace_step!(heads_count);
    lesson_trace::trace_note!(
        "100 000 псевдослучайных опытов уменьшают колебания оценённых частот."
    );
    lesson_trace::trace_note!("Это размер учебной симуляции, а не параметр формулы Байеса.");
    for _ in 0..100_000 {
        lesson_trace::trace_note!(
            "Для честной монеты событие «орёл» занимает половину интервала [0, 1)."
        );
        if generator.generate_random_number_between_zero_and_one() < 0.5 {
            lesson_trace::trace_note!(
                "Прибавляем очередной вклад к ранее накопленному результату."
            );
            heads_count += 1;
            lesson_trace::trace_step!(heads_count);
        }
        lesson_trace::trace_note!(
            "0.01 — заданная для примера распространённость: заболевание есть примерно у 1% людей."
        );
        let has_disease: bool = generator.generate_random_number_between_zero_and_one() < 0.01;
        lesson_trace::trace_step!(has_disease);
        lesson_trace::trace_note!(
            "Сохраняем рассчитанное значение `test_is_positive` для следующих операций."
        );
        let test_is_positive: bool = if has_disease {
            lesson_trace::trace_note!(
                "Для больного моделируем положительный тест с чувствительностью 90%."
            );
            generator.generate_random_number_between_zero_and_one() < 0.9
        } else {
            lesson_trace::trace_note!(
                "Обрабатываем случай, когда предыдущее условие не выполнено."
            );
            lesson_trace::trace_note!(
                "Для здорового моделируем ложноположительный тест с вероятностью 5%."
            );
            generator.generate_random_number_between_zero_and_one() < 0.05
        };
        lesson_trace::trace_step!(test_is_positive);
        lesson_trace::trace_note!("Проверяем условие и выбираем соответствующую ветку алгоритма.");
        if test_is_positive {
            lesson_trace::trace_note!(
                "Прибавляем очередной вклад к ранее накопленному результату."
            );
            positive_test_count += 1;
            lesson_trace::trace_step!(positive_test_count);
            lesson_trace::trace_note!(
                "Проверяем условие и выбираем соответствующую ветку алгоритма."
            );
            if has_disease {
                lesson_trace::trace_note!(
                    "Прибавляем очередной вклад к ранее накопленному результату."
                );
                true_positive_count += 1;
                lesson_trace::trace_step!(true_positive_count);
            }
        }
    }

    lesson_trace::trace_note!("Шаг: Сравниваем условную частоту болезни с формулой Байеса.");
    lesson_trace::trace_note!(
        "Задаём шаблон строки: плейсхолдеры ниже заменятся рассчитанными значениями."
    );
    lesson_trace::trace_note!("Возвращаем булев результат для этого случая.");
    lesson_trace::trace_note!("Составляем результат из вычисленных значений в указанном порядке.");
    lesson_trace::trace_note!("Используем подготовленное значение в следующем шаге примера.");
    lesson_trace::trace_note!("Формула Байеса учитывает и качество теста, и редкость болезни.");
    lesson_trace::trace_note!("Сохраняем результат этого шага в `prevalence`.");
    lesson_trace::trace_note!("Инициализируем значение `sensitivity` начальным состоянием.");
    lesson_trace::trace_note!("Инициализируем значение `specificity` начальным состоянием.");
    lesson_trace::trace_note!(
        "Умножаем значения и сохраняем результат в `true_positive_probability`."
    );
    lesson_trace::trace_note!("Возвращаем булев результат для этого случая.");
    lesson_trace::trace_note!("Делим значения, получая нормированную величину или среднее.");
    println!(
        "heads={heads_count}, P(ill|positive): simulation={:.3}, Bayes={:.3}",
        true_positive_count as f64 / positive_test_count as f64,
        (|| -> f64 {
            let prevalence: f64 = 0.01;
            lesson_trace::trace_step!(prevalence);
            lesson_trace::trace_step!(prevalence);

            let sensitivity: f64 = 0.9;
            lesson_trace::trace_step!(sensitivity);
            lesson_trace::trace_step!(sensitivity);

            let specificity: f64 = 0.95;
            lesson_trace::trace_step!(specificity);
            lesson_trace::trace_step!(specificity);

            let true_positive_probability: f64 = prevalence * sensitivity;
            lesson_trace::trace_step!(true_positive_probability);
            lesson_trace::trace_step!(true_positive_probability);

            true_positive_probability
                / (true_positive_probability + (1.0 - prevalence) * (1.0 - specificity))
        })()
    );

    lesson_trace::trace_note!("Построение графика вынесено из основного кода урока.");
    lesson_trace::disable();
    plot_simulated_and_calculated_disease_rates_after_positive_test(
        positive_test_count,
        true_positive_count,
    );
}

// Строим график по результатам урока.
fn plot_simulated_and_calculated_disease_rates_after_positive_test(
    positive_test_count: i32,
    true_positive_count: i32,
) {
    lesson_trace::trace_note!("Значения из этого урока на графике.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Настраиваем или преобразуем результат предыдущего шага.");
    let theoretical_probability_points: Vec<(f64, f64)> = [
        (0.0, 0.01 * 0.9 / (0.01 * 0.9 + 0.99 * 0.05)),
        (100000.0, 0.01 * 0.9 / (0.01 * 0.9 + 0.99 * 0.05)),
    ]
    .to_vec();
    lesson_trace::trace_note!("Собираем значения для `simulated_probability_points` в коллекцию.");
    lesson_trace::trace_note!("Используем подготовленное значение в следующем шаге примера.");
    lesson_trace::trace_note!("Вычисляем значение по указанной формуле.");
    lesson_trace::trace_note!("Настраиваем или преобразуем результат предыдущего шага.");
    let simulated_probability_points: Vec<(f64, f64)> = [(
        100000.0,
        true_positive_count as f64 / positive_test_count as f64,
    )]
    .to_vec();
    lesson_trace::trace_note!("Строим график по рассчитанным значениям и сохраняем его как SVG.");
    lesson_trace::trace_note!("Передаём путь к каталогу текущего урока.");
    lesson_trace::trace_note!("Указываем имя SVG-файла.");
    lesson_trace::trace_note!("Указываем заголовок графика.");
    lesson_trace::trace_note!("Указываем подпись горизонтальной оси.");
    lesson_trace::trace_note!("Указываем подпись вертикальной оси.");
    lesson_trace::trace_note!("Передаём ряды или значения для отрисовки графика.");
    lesson_trace::trace_note!("Добавляем ряд данных с подписью к графику.");
    lesson_trace::trace_note!("Указываем подпись этого ряда в легенде.");
    lesson_trace::trace_note!("Передаём рассчитанные координаты точек.");
    lesson_trace::trace_note!("Добавляем ряд данных с подписью к графику.");
    lesson_trace::trace_note!("Указываем подпись этого ряда в легенде.");
    lesson_trace::trace_note!("Передаём рассчитанные координаты точек.");
    lesson_trace::trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
    let chart: std::path::PathBuf = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Монте-Карло и формула Байеса",
        "число испытаний",
        "частота заболевания после положительного теста",
        &[
            lesson_visualization::Series {
                name: "теория",

                points: &theoretical_probability_points,
            },
            lesson_visualization::Series {
                name: "симуляция",

                points: &simulated_probability_points,
            },
        ],
    )
    .expect("не удалось сохранить график");
    lesson_trace::trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
