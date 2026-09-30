// Урок 20.4. Практика: прямой расчёт значений и обратный расчёт скоростей изменения.
// Связь с принятой терминологией: Прямой и обратный проходы вычислительного графа.
// Зачем здесь эта тема: Обратное распространение соединяет прямые значения, локальные производные и
//   накопление вкладов.
// Почему код устроен так: Строим небольшой граф с повторным использованием узла и проверяем
//   градиент вручную.
// Представь: Считаем граф вперёд для значения, потом назад для производных и сверяем результат
//   численно.
//
// Что повторяем вместе: прямой проход, обратное распространение, накопление градиентов.
// Зачем это нужно: Вычислительный граф хранит операции прямого прохода, а обратный проход применяет правило
//   цепочки к каждому ребру.
// Что показывает программа: Создаём пустой граф скалярных операций. Строим прямой проход: x, x², 2x² и
//   tanh(2x²). Идём по графу назад: правило цепочки распределяет градиент по каждому ребру.
// Что проверить при изменении примера: Сравни производные с конечными разностями, включая повторное
//   использование узла.
// Дополнительная практика: Сделай маленький граф скаляров с операциями +, × и нелинейностью; вычисли backward.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    lesson_trace::enable();
    lesson_trace::trace_note!("Учебные реализации математических операций для этого урока.");

    /// e^x по ряду Тейлора. Деление аргумента пополам ускоряет сходимость.
    /// Учебный аналог `f64::exp`; показывает вычисление ряда и может работать медленнее.
    /// При замене возможны небольшие отличия из-за точности и обработки крайних значений.
    /// Экспонента eˣ: ряд Тейлора 1 + x + x²/2! + x³/3! + … с уменьшением аргумента и восстановлением масштаба.
    fn approximate_e_to_power_by_summing_power_over_factorial_terms(value: f64) -> f64 {
        lesson_trace::trace_note!("Проверяем условие и выбираем соответствующую ветку алгоритма.");
        if value == f64::NEG_INFINITY || value < -745.0 {
            lesson_trace::trace_note!("Завершаем текущий расчёт и возвращаем найденное значение.");
            return 0.0;
        }
        lesson_trace::trace_note!("Проверяем условие и выбираем соответствующую ветку алгоритма.");
        if value == f64::INFINITY || value > 709.0 {
            lesson_trace::trace_note!("Завершаем текущий расчёт и возвращаем найденное значение.");
            return f64::INFINITY;
        }
        lesson_trace::trace_note!("Проверяем условие и выбираем соответствующую ветку алгоритма.");
        if value < 0.0 {
            lesson_trace::trace_note!("Завершаем текущий расчёт и возвращаем найденное значение.");
            return 1.0 / approximate_e_to_power_by_summing_power_over_factorial_terms(-value);
        }
        lesson_trace::trace_note!("Создаём изменяемое значение `reduced` для следующих операций.");
        let mut reduced: f64 = value;
        lesson_trace::trace_step!(reduced);
        lesson_trace::trace_note!(
            "Инициализируем изменяемый накопитель `halving_count` начальным состоянием."
        );
        let mut halving_count: i32 = 0;
        lesson_trace::trace_step!(halving_count);
        lesson_trace::trace_note!(
            "Уменьшаем аргумент до ≤0.5: на таком интервале ряд Тейлора для exp сходится быстро."
        );
        while reduced > 0.5 {
            lesson_trace::trace_note!("Масштабируем текущую величину делением.");
            reduced /= 2.0;
            lesson_trace::trace_step!(reduced);
            lesson_trace::trace_note!(
                "Прибавляем очередной вклад к ранее накопленному результату."
            );
            halving_count += 1;
            lesson_trace::trace_step!(halving_count);
        }
        lesson_trace::trace_note!("Создаём изменяемое значение `term` для следующих операций.");
        let mut term: f64 = 1.0;
        lesson_trace::trace_step!(term);
        lesson_trace::trace_note!("Создаём изменяемое значение `result` для следующих операций.");
        let mut result: f64 = 1.0;
        lesson_trace::trace_step!(result);
        lesson_trace::trace_note!(
            "Берём 30 членов ряда exp(y)=Σ y^k/k!; это предел приближения для учебных входов."
        );
        for term_index in 1..=30 {
            lesson_trace::trace_step!(term_index);
            lesson_trace::trace_note!("Умножаем накопленное значение на очередной множитель.");
            term *= reduced / term_index as f64;
            lesson_trace::trace_step!(term);
            lesson_trace::trace_note!(
                "Прибавляем очередной вклад к ранее накопленному результату."
            );
            result += term;
            lesson_trace::trace_step!(result);
        }
        lesson_trace::trace_note!(
            "Восстанавливаем exp(value): каждое возведение в квадрат отменяет одно деление аргумента на 2."
        );
        for _ in 0..halving_count {
            lesson_trace::trace_note!("Умножаем накопленное значение на очередной множитель.");
            result *= result;
            lesson_trace::trace_step!(result);
        }
        lesson_trace::trace_note!(
            "Используем ранее рассчитанное значение `result` в текущем выражении."
        );
        result
    }

    /// tanh(x) = (e^(2x)-1)/(e^(2x)+1), отдельная формула для отрицательных x.
    /// Учебный аналог `f64::tanh`; явная формула может работать медленнее и давать другое округление.
    /// Гиперболический тангенс tanh(x) = (e^(2x)−1) / (e^(2x)+1); используем симметрию и насыщение для устойчивости.
    fn calculate_tanh_as_e_to_twice_value_minus_one_divided_by_e_to_twice_value_plus_one(
        value: f64,
    ) -> f64 {
        lesson_trace::trace_note!("Проверяем условие и выбираем соответствующую ветку алгоритма.");
        if value < 0.0 {
            lesson_trace::trace_note!("Завершаем текущий расчёт и возвращаем найденное значение.");
            return -calculate_tanh_as_e_to_twice_value_minus_one_divided_by_e_to_twice_value_plus_one(-value);
        }
        lesson_trace::trace_note!("Проверяем условие и выбираем соответствующую ветку алгоритма.");
        if value > 20.0 {
            lesson_trace::trace_note!("Завершаем текущий расчёт и возвращаем найденное значение.");
            return 1.0;
        }
        lesson_trace::trace_note!("Умножаем значения и сохраняем результат в `exponential_value`.");
        let exponential_value: f64 =
            approximate_e_to_power_by_summing_power_over_factorial_terms(2.0 * value);
        lesson_trace::trace_step!(exponential_value);
        lesson_trace::trace_note!(
            "Составляем результат из вычисленных значений в указанном порядке."
        );
        (exponential_value - 1.0) / (exponential_value + 1.0)
    }

    lesson_trace::trace_note!(
        "Автоматически получаем стандартные реализации перечисленных трейтов для этого типа."
    );
    lesson_trace::trace_note!(
        "Описываем тип `Operation`, чтобы явно хранить состояние и допустимые варианты."
    );
    lesson_trace::trace_note!("Вариант входа хранит значение без операции над предками.");
    lesson_trace::trace_note!("Вариант сложения запоминает индексы обоих аргументов.");
    lesson_trace::trace_note!("Вариант умножения запоминает индексы обоих аргументов.");
    lesson_trace::trace_note!("Вариант tanh запоминает индекс входного узла.");
    #[derive(Clone, Copy, Debug)]
    enum Operation {
        Input,

        Add(usize, usize),

        Mul(usize, usize),

        Tanh(usize),
    }

    lesson_trace::trace_note!(
        "Описываем тип `Node`, чтобы явно хранить состояние и допустимые варианты."
    );
    lesson_trace::trace_note!("Поле `value` соответствующее значение в составе структуры.");
    lesson_trace::trace_note!(
        "Поле `rate_of_change` соответствующее значение в составе структуры."
    );
    lesson_trace::trace_note!("Поле `operation` соответствующее значение в составе структуры.");
    #[derive(Debug)]
    struct Node {
        value: f64,

        gradient: f64,

        operation: Operation,
    }

    lesson_trace::trace_note!(
        "Описываем тип `Graph`, чтобы явно хранить состояние и допустимые варианты."
    );
    #[derive(Debug)]
    struct Graph(Vec<Node>);

    lesson_trace::trace_note!("Группируем методы рядом с типом, к которому они относятся.");
    lesson_trace::trace_note!(
        "Объявляем повторно используемое вычисление `append_input_node`; параметры ниже задают его входы."
    );
    impl Graph {
        fn append_input_value_node_to_computation_graph(&mut self, value: f64) -> usize {
            lesson_trace::trace_note!(
                "Считаем количество элементов и сохраняем его в `node_index`."
            );
            let node_index: usize = self.0.len();
            lesson_trace::trace_step!(node_index);
            lesson_trace::trace_note!("Обновляем состояние объекта результатом текущей операции.");
            lesson_trace::trace_note!(
                "Используем ранее рассчитанное значение `value` в текущем выражении."
            );
            lesson_trace::trace_note!(
                "Заполняем поле `rate_of_change` соответствующим рассчитанным значением."
            );
            lesson_trace::trace_note!(
                "Заполняем поле `operation` соответствующим рассчитанным значением."
            );
            self.0.push(Node {
                value,

                gradient: 0.,

                operation: Operation::Input,
            });
            lesson_trace::trace_note!(
                "Используем ранее рассчитанное значение `node_index` в текущем выражении."
            );
            node_index
        }
    }

    lesson_trace::trace_note!("Шаг: Создаём пустой граф скалярных операций.");
    let mut graph: Graph = Graph(vec![]);
    lesson_trace::trace_step!(graph);
    lesson_trace::trace_note!("Шаг: Строим прямой проход: x, x², 2x² и tanh(2x²).");
    let input_index: usize = graph.append_input_value_node_to_computation_graph(2.);
    lesson_trace::trace_step!(input_index);
    lesson_trace::trace_note!("Умножение x на x создаёт узел с двумя ребрами к одному входу.");
    lesson_trace::trace_note!("Обновляем состояние объекта результатом текущей операции.");
    let squared_index: usize = graph.append_input_value_node_to_computation_graph(
        graph.0[input_index].value * graph.0[input_index].value,
    );
    lesson_trace::trace_step!(squared_index);
    lesson_trace::trace_note!("Обновляем состояние объекта результатом текущей операции.");
    graph.0[squared_index].operation = Operation::Mul(input_index, input_index);
    lesson_trace::trace_step!(graph);
    lesson_trace::trace_note!("Складываем полученный квадрат с самим собой.");
    let doubled_square_index: usize =
        graph.append_input_value_node_to_computation_graph(graph.0[squared_index].value * 2.0);
    lesson_trace::trace_step!(doubled_square_index);
    lesson_trace::trace_note!("Обновляем состояние объекта результатом текущей операции.");
    graph.0[doubled_square_index].operation = Operation::Add(squared_index, squared_index);
    lesson_trace::trace_step!(graph);
    lesson_trace::trace_note!(
        "Применяем tanh к результату и запоминаем его вход для обратного прохода."
    );
    lesson_trace::trace_note!("Обновляем состояние объекта результатом текущей операции.");
    let output_index: usize = graph.append_input_value_node_to_computation_graph(
        calculate_tanh_as_e_to_twice_value_minus_one_divided_by_e_to_twice_value_plus_one(
            graph.0[doubled_square_index].value,
        ),
    );
    lesson_trace::trace_step!(output_index);
    lesson_trace::trace_note!("Обновляем состояние объекта результатом текущей операции.");
    graph.0[output_index].operation = Operation::Tanh(doubled_square_index);
    lesson_trace::trace_step!(graph);
    lesson_trace::trace_note!(
        "Шаг: Идём по графу назад: правило цепочки распределяет градиент по каждому ребру."
    );
    graph.0[output_index].gradient = 1.;
    lesson_trace::trace_step!(graph);
    lesson_trace::trace_note!(
        "Повторяем следующий блок для каждого элемента указанной последовательности."
    );
    for node_index in (0..=output_index).rev() {
        lesson_trace::trace_step!(node_index);
        lesson_trace::trace_note!(
            "Сохраняем рассчитанное значение `incoming_loss_rate_of_change` для следующих операций."
        );
        lesson_trace::trace_note!(
            "Производную функции по параметру или вектор таких производных называют gradient."
        );
        let incoming_loss_rate_of_change: f64 = graph.0[node_index].gradient;
        lesson_trace::trace_step!(incoming_loss_rate_of_change);
        lesson_trace::trace_note!("Разбираем каждый возможный вариант значения отдельно.");
        lesson_trace::trace_note!("Обрабатываем этот вариант структуры данных отдельным правилом.");
        lesson_trace::trace_note!("Обрабатываем этот вариант структуры данных отдельным правилом.");
        lesson_trace::trace_note!("Обрабатываем этот вариант структуры данных отдельным правилом.");
        lesson_trace::trace_note!("Обрабатываем этот вариант структуры данных отдельным правилом.");
        match graph.0[node_index].operation {
            Operation::Input => {}

            Operation::Add(left_index, right_index) => {
                lesson_trace::trace_note!(
                    "Накапливаем вклад текущего шага в состояние модели или графа."
                );
                graph.0[left_index].gradient += incoming_loss_rate_of_change;
                lesson_trace::trace_step!(graph);
                lesson_trace::trace_note!(
                    "Накапливаем вклад текущего шага в состояние модели или графа."
                );
                graph.0[right_index].gradient += incoming_loss_rate_of_change;
                lesson_trace::trace_step!(graph);
            }

            Operation::Mul(left_index, right_index) => {
                lesson_trace::trace_note!(
                    "Накапливаем вклад текущего шага в состояние модели или графа."
                );
                graph.0[left_index].gradient +=
                    incoming_loss_rate_of_change * graph.0[right_index].value;
                lesson_trace::trace_step!(graph);
                lesson_trace::trace_note!(
                    "Накапливаем вклад текущего шага в состояние модели или графа."
                );
                graph.0[right_index].gradient +=
                    incoming_loss_rate_of_change * graph.0[left_index].value;
                lesson_trace::trace_step!(graph);
            }

            Operation::Tanh(left_index) => {
                lesson_trace::trace_note!("Производная tanh(z) равна 1 − tanh(z)².");
                let output_value: f64 = graph.0[node_index].value;
                lesson_trace::trace_step!(output_value);
                lesson_trace::trace_note!(
                    "Накапливаем вклад текущего шага в состояние модели или графа."
                );
                lesson_trace::trace_note!("Умножаем величины согласно используемой формуле.");
                graph.0[left_index].gradient +=
                    incoming_loss_rate_of_change * (1.0 - output_value * output_value);
                lesson_trace::trace_step!(graph);
            }
        }
    }
    lesson_trace::trace_note!("Шаг: Показываем значение функции и производную по входу.");
    lesson_trace::trace_note!(
        "Задаём шаблон строки: плейсхолдеры ниже заменятся рассчитанными значениями."
    );
    lesson_trace::trace_note!("Обновляем состояние объекта результатом текущей операции.");
    lesson_trace::trace_note!("Показываем накопленную производную результата по входу.");
    println!(
        "f(2)={}, f'(2)={}",
        graph.0[output_index].value, graph.0[input_index].gradient
    );

    lesson_trace::trace_note!("Построение графика вынесено из основного кода урока.");
    lesson_trace::disable();
    plot_output_rate_of_change_for_each_computation_node(graph);

    lesson_trace::trace_note!("Строим график по результатам урока.");
    fn plot_output_rate_of_change_for_each_computation_node(graph: Graph) {
        lesson_trace::trace_note!("Наглядное представление вычислений сводной практики.");
        lesson_trace::trace_note!("Настраиваем или преобразуем результат предыдущего шага.");
        lesson_trace::trace_note!("Просматриваем элементы коллекции по ссылке.");
        lesson_trace::trace_note!("Добавляем порядковый номер к каждому элементу.");
        lesson_trace::trace_note!("Преобразуем каждый элемент в новое значение.");
        lesson_trace::trace_note!("Собираем результаты в коллекцию.");
        let computation_graph_points: Vec<(f64, f64)> = graph
            .0
            .iter()
            .enumerate()
            .map(|(item_index, node)| (item_index as f64, node.gradient))
            .collect();
        lesson_trace::trace_note!(
            "Строим график по рассчитанным значениям и сохраняем его как SVG."
        );
        lesson_trace::trace_note!("Передаём путь к каталогу текущего урока.");
        lesson_trace::trace_note!("Указываем имя SVG-файла.");
        lesson_trace::trace_note!("Указываем заголовок графика.");
        lesson_trace::trace_note!("Указываем подпись горизонтальной оси.");
        lesson_trace::trace_note!("Указываем подпись вертикальной оси.");
        lesson_trace::trace_note!("Передаём ряды или значения для отрисовки графика.");
        lesson_trace::trace_note!("Указываем подпись этого ряда в легенде.");
        lesson_trace::trace_note!("Передаём рассчитанные координаты точек.");
        lesson_trace::trace_note!(
            "Прерываем пример с понятной ошибкой, если SVG не удалось записать."
        );
        let chart: std::path::PathBuf = lesson_visualization::line_chart(
            env!("CARGO_MANIFEST_DIR"),
            "lesson-chart",
            "Градиенты вычислительного графа",
            "номер узла",
            "градиент",
            &[lesson_visualization::Series {
                name: "обратный проход",

                points: &computation_graph_points,
            }],
        )
        .expect("не удалось сохранить график");
        lesson_trace::trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
        println!("график: {}", chart.display());
    }
}
