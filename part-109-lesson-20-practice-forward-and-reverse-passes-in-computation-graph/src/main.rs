// Сводная практика 20. Прямой и обратный проходы вычислительного графа.
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
    // Учебные реализации математических операций для этого урока.

    /// e^x по ряду Тейлора. Деление аргумента пополам ускоряет сходимость.
    fn approximate_exponential_with_taylor_series(value: f64) -> f64 {
        // Проверяем условие и выбираем соответствующую ветку алгоритма.
        if value == f64::NEG_INFINITY || value < -745.0 {
            // Завершаем текущий расчёт и возвращаем найденное значение.
            return 0.0;
        }
        // Проверяем условие и выбираем соответствующую ветку алгоритма.
        if value == f64::INFINITY || value > 709.0 {
            // Завершаем текущий расчёт и возвращаем найденное значение.
            return f64::INFINITY;
        }
        // Проверяем условие и выбираем соответствующую ветку алгоритма.
        if value < 0.0 {
            // Завершаем текущий расчёт и возвращаем найденное значение.
            return 1.0 / approximate_exponential_with_taylor_series(-value);
        }
        // Создаём изменяемое значение `reduced` для следующих операций.
        let mut reduced: f64 = value;
        lesson_trace::trace_step!(reduced);
        // Инициализируем изменяемый накопитель `halving_count` начальным состоянием.
        let mut halving_count: i32 = 0;
        lesson_trace::trace_step!(halving_count);
        // Уменьшаем аргумент до ≤0.5: на таком интервале ряд Тейлора для exp сходится быстро.
        while reduced > 0.5 {
            // Масштабируем текущую величину делением.
            reduced /= 2.0;
            lesson_trace::trace_step!(reduced);
            // Прибавляем очередной вклад к ранее накопленному результату.
            halving_count += 1;
            lesson_trace::trace_step!(halving_count);
        }
        // Создаём изменяемое значение `term` для следующих операций.
        let mut term: f64 = 1.0;
        lesson_trace::trace_step!(term);
        // Создаём изменяемое значение `result` для следующих операций.
        let mut result: f64 = 1.0;
        lesson_trace::trace_step!(result);
        // Берём 30 членов ряда exp(y)=Σ y^k/k!; это предел приближения для учебных входов.
        for term_index in 1..=30 {
            lesson_trace::trace_step!(term_index);
            // Умножаем накопленное значение на очередной множитель.
            term *= reduced / term_index as f64;
            lesson_trace::trace_step!(term);
            // Прибавляем очередной вклад к ранее накопленному результату.
            result += term;
            lesson_trace::trace_step!(result);
        }
        // Восстанавливаем exp(value): каждое возведение в квадрат отменяет одно деление аргумента на 2.
        for _ in 0..halving_count {
            // Умножаем накопленное значение на очередной множитель.
            result *= result;
            lesson_trace::trace_step!(result);
        }
        // Используем ранее рассчитанное значение `result` в текущем выражении.
        result
    }

    /// tanh(x) = (e^(2x)-1)/(e^(2x)+1), отдельная формула для отрицательных x.
    fn calculate_hyperbolic_tangent_from_exponentials(value: f64) -> f64 {
        // Проверяем условие и выбираем соответствующую ветку алгоритма.
        if value < 0.0 {
            // Завершаем текущий расчёт и возвращаем найденное значение.
            return -calculate_hyperbolic_tangent_from_exponentials(-value);
        }
        // Проверяем условие и выбираем соответствующую ветку алгоритма.
        if value > 20.0 {
            // Завершаем текущий расчёт и возвращаем найденное значение.
            return 1.0;
        }
        // Умножаем значения и сохраняем результат в `exponential_value`.
        let exponential_value: f64 = approximate_exponential_with_taylor_series(2.0 * value);
        lesson_trace::trace_step!(exponential_value);
        // Составляем результат из вычисленных значений в указанном порядке.
        (exponential_value - 1.0) / (exponential_value + 1.0)
    }

    // Автоматически получаем стандартные реализации перечисленных трейтов для этого типа.
    #[derive(Clone, Copy, Debug)]
    // Описываем тип `Operation`, чтобы явно хранить состояние и допустимые варианты.
    enum Operation {
        // Вариант входа хранит значение без операции над предками.
        Input,
        // Вариант сложения запоминает индексы обоих аргументов.
        Add(usize, usize),
        // Вариант умножения запоминает индексы обоих аргументов.
        Mul(usize, usize),
        // Вариант tanh запоминает индекс входного узла.
        Tanh(usize),
    }

    // Описываем тип `Node`, чтобы явно хранить состояние и допустимые варианты.
    #[derive(Debug)]
    struct Node {
        // Поле `value` соответствующее значение в составе структуры.
        value: f64,
        // Поле `rate_of_change` соответствующее значение в составе структуры.
        gradient: f64,
        // Поле `operation` соответствующее значение в составе структуры.
        operation: Operation,
    }

    // Описываем тип `Graph`, чтобы явно хранить состояние и допустимые варианты.
    #[derive(Debug)]
    struct Graph(Vec<Node>);

    // Группируем методы рядом с типом, к которому они относятся.
    impl Graph {
        // Объявляем повторно используемое вычисление `append_input_node`; параметры ниже задают его входы.
        fn append_input_value_node_to_computation_graph(&mut self, value: f64) -> usize {
            // Считаем количество элементов и сохраняем его в `node_index`.
            let node_index: usize = self.0.len();
            lesson_trace::trace_step!(node_index);
            // Обновляем состояние объекта результатом текущей операции.
            self.0.push(Node {
                // Используем ранее рассчитанное значение `value` в текущем выражении.
                value,
                // Заполняем поле `rate_of_change` соответствующим рассчитанным значением.
                gradient: 0.,
                // Заполняем поле `operation` соответствующим рассчитанным значением.
                operation: Operation::Input,
            });
            // Используем ранее рассчитанное значение `node_index` в текущем выражении.
            node_index
        }
    }

    // Шаг: Создаём пустой граф скалярных операций.
    let mut graph: Graph = Graph(vec![]);
    lesson_trace::trace_step!(graph);
    // Шаг: Строим прямой проход: x, x², 2x² и tanh(2x²).
    let input_index: usize = graph.append_input_value_node_to_computation_graph(2.);
    lesson_trace::trace_step!(input_index);
    // Умножение x на x создаёт узел с двумя ребрами к одному входу.
    let squared_index: usize =
        // Обновляем состояние объекта результатом текущей операции.
        graph.append_input_value_node_to_computation_graph(graph.0[input_index].value * graph.0[input_index].value);
    lesson_trace::trace_step!(squared_index);
    // Обновляем состояние объекта результатом текущей операции.
    graph.0[squared_index].operation = Operation::Mul(input_index, input_index);
    lesson_trace::trace_step!(graph);
    // Складываем полученный квадрат с самим собой.
    let doubled_square_index: usize =
        graph.append_input_value_node_to_computation_graph(graph.0[squared_index].value * 2.0);
    lesson_trace::trace_step!(doubled_square_index);
    // Обновляем состояние объекта результатом текущей операции.
    graph.0[doubled_square_index].operation = Operation::Add(squared_index, squared_index);
    lesson_trace::trace_step!(graph);
    // Применяем tanh к результату и запоминаем его вход для обратного прохода.
    let output_index: usize = graph.append_input_value_node_to_computation_graph(
        calculate_hyperbolic_tangent_from_exponentials(
            // Обновляем состояние объекта результатом текущей операции.
            graph.0[doubled_square_index].value,
        ),
    );
    lesson_trace::trace_step!(output_index);
    // Обновляем состояние объекта результатом текущей операции.
    graph.0[output_index].operation = Operation::Tanh(doubled_square_index);
    lesson_trace::trace_step!(graph);
    // Шаг: Идём по графу назад: правило цепочки распределяет градиент по каждому ребру.
    graph.0[output_index].gradient = 1.;
    lesson_trace::trace_step!(graph);
    // Повторяем следующий блок для каждого элемента указанной последовательности.
    for node_index in (0..=output_index).rev() {
        lesson_trace::trace_step!(node_index);
        // Сохраняем рассчитанное значение `incoming_loss_rate_of_change` для следующих операций.
        // Производную функции по параметру или вектор таких производных называют gradient.
        let incoming_loss_rate_of_change: f64 = graph.0[node_index].gradient;
        lesson_trace::trace_step!(incoming_loss_rate_of_change);
        // Разбираем каждый возможный вариант значения отдельно.
        match graph.0[node_index].operation {
            // Обрабатываем этот вариант структуры данных отдельным правилом.
            Operation::Input => {}
            // Обрабатываем этот вариант структуры данных отдельным правилом.
            Operation::Add(left_index, right_index) => {
                // Накапливаем вклад текущего шага в состояние модели или графа.
                graph.0[left_index].gradient += incoming_loss_rate_of_change;
                lesson_trace::trace_step!(graph);
                // Накапливаем вклад текущего шага в состояние модели или графа.
                graph.0[right_index].gradient += incoming_loss_rate_of_change;
                lesson_trace::trace_step!(graph);
            }
            // Обрабатываем этот вариант структуры данных отдельным правилом.
            Operation::Mul(left_index, right_index) => {
                // Накапливаем вклад текущего шага в состояние модели или графа.
                graph.0[left_index].gradient +=
                    incoming_loss_rate_of_change * graph.0[right_index].value;
                lesson_trace::trace_step!(graph);
                // Накапливаем вклад текущего шага в состояние модели или графа.
                graph.0[right_index].gradient +=
                    incoming_loss_rate_of_change * graph.0[left_index].value;
                lesson_trace::trace_step!(graph);
            }
            // Обрабатываем этот вариант структуры данных отдельным правилом.
            Operation::Tanh(left_index) => {
                // Производная tanh(z) равна 1 − tanh(z)².
                let output_value: f64 = graph.0[node_index].value;
                lesson_trace::trace_step!(output_value);
                // Накапливаем вклад текущего шага в состояние модели или графа.
                graph.0[left_index].gradient +=
                    // Умножаем величины согласно используемой формуле.
                    incoming_loss_rate_of_change * (1.0 - output_value * output_value);
                lesson_trace::trace_step!(graph);
            }
        }
    }
    // Шаг: Показываем значение функции и производную по входу.
    println!(
        // Задаём шаблон строки: плейсхолдеры ниже заменятся рассчитанными значениями.
        "f(2)={}, f'(2)={}",
        // Обновляем состояние объекта результатом текущей операции.
        graph.0[output_index].value,
        // Показываем накопленную производную результата по входу.
        graph.0[input_index].gradient
    );

    // Построение графика вынесено из основного кода урока.
    lesson_trace::disable();
    visualize_practice_forward_and_reverse_passes_in_computation_graph(graph);

    // Строим график по результатам урока.
    fn visualize_practice_forward_and_reverse_passes_in_computation_graph(graph: Graph) {
        // Наглядное представление вычислений сводной практики.
        let computation_graph_points: Vec<(f64, f64)> = graph
            // Настраиваем или преобразуем результат предыдущего шага.
            .0
            // Просматриваем элементы коллекции по ссылке.
            .iter()
            // Добавляем порядковый номер к каждому элементу.
            .enumerate()
            // Преобразуем каждый элемент в новое значение.
            .map(|(item_index, node)| (item_index as f64, node.gradient))
            // Собираем результаты в коллекцию.
            .collect();
        // Строим график по рассчитанным значениям и сохраняем его как SVG.
        let chart: std::path::PathBuf = lesson_visualization::line_chart(
            // Передаём путь к каталогу текущего урока.
            env!("CARGO_MANIFEST_DIR"),
            // Указываем имя SVG-файла.
            "lesson-chart",
            // Указываем заголовок графика.
            "Градиенты вычислительного графа",
            // Указываем подпись горизонтальной оси.
            "номер узла",
            // Указываем подпись вертикальной оси.
            "градиент",
            // Передаём ряды или значения для отрисовки графика.
            &[lesson_visualization::Series {
                // Указываем подпись этого ряда в легенде.
                name: "обратный проход",
                // Передаём рассчитанные координаты точек.
                points: &computation_graph_points,
            }],
        )
        // Прерываем пример с понятной ошибкой, если SVG не удалось записать.
        .expect("не удалось сохранить график");
        // Печатаем путь к созданному SVG, чтобы его можно было открыть.
        println!("график: {}", chart.display());
    }
}
