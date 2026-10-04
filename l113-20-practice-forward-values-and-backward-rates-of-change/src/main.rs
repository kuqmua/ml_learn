// Урок 20.4. Практика: прямой расчёт значений и обратный расчёт скоростей изменения.
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
// Что показывает программа: Создаём пустой список связанных действий над отдельными числами. Строим прямой проход: x, x², 2x² и
//   tanh(2x²). Идём по графу назад: правило цепочки распределяет градиент по каждому ребру.
// Что проверить при изменении примера: Сравни производные с конечными разностями, включая повторное
//   использование узла.
// Дополнительная практика: Сделай маленький граф скаляров с операциями +, × и нелинейностью; вычисли backward.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.

fn main() {
    /// e^x по ряду Тейлора. Деление аргумента пополам ускоряет сходимость.
    /// Учебный аналог `f64::exp`; показывает вычисление ряда и может работать медленнее.
    /// При замене возможны небольшие отличия из-за точности и обработки крайних значений.
    /// Экспонента eˣ: ряд Тейлора 1 + x + x²/2! + x³/3! + … с уменьшением аргумента и восстановлением масштаба.
    fn approximate_e_to_power_by_summing_power_over_factorial_terms(value: f64) -> f64 {
        if value == f64::NEG_INFINITY || value < -745.0 {
            return 0.0;
        }
        if value == f64::INFINITY || value > 709.0 {
            return f64::INFINITY;
        }
        if value < 0.0 {
            return 1.0 / approximate_e_to_power_by_summing_power_over_factorial_terms(-value);
        }
        let mut reduced: f64 = value;
        let mut halving_count: i32 = 0;
        while reduced > 0.5 {
            reduced /= 2.0;
            halving_count += 1;
        }
        let mut term: f64 = 1.0;
        let mut exponential_approximation: f64 = 1.0;
        for term_index in 1..=30 {
            term *= reduced / term_index as f64;
            exponential_approximation += term;
        }
        for _ in 0..halving_count {
            exponential_approximation *= exponential_approximation;
        }
        exponential_approximation
    }

    /// tanh(x) = (e^(2x)-1)/(e^(2x)+1), отдельная формула для отрицательных x.
    /// Учебный аналог `f64::tanh`; явная формула может работать медленнее и давать другое округление.
    /// Гиперболический тангенс tanh(x) = (e^(2x)−1) / (e^(2x)+1); используем симметрию и насыщение для устойчивости.
    fn calc_tanh(value: f64) -> f64 {
        if value < 0.0 {
            return -calc_tanh(-value);
        }
        if value > 20.0 {
            return 1.0;
        }
        let exponential_value: f64 =
            approximate_e_to_power_by_summing_power_over_factorial_terms(2.0 * value);
        (exponential_value - 1.0) / (exponential_value + 1.0)
    }

    #[derive(Clone, Copy, Debug)]
    enum Operation {
        Input,

        Add(usize, usize),

        Mul(usize, usize),

        TanhAsSignedSignalBoundedBetweenMinus1And1(usize),
    }

    #[derive(Debug)]
    struct Node {
        value: f64,

        output_rate_of_change_with_respect_to_node_value: f64,

        operation: Operation,
    }

    #[derive(Debug)]
    struct Graph(Vec<Node>);

    impl Graph {
        fn append_input_value_node_to_computation_graph(&mut self, value: f64) -> usize {
            let node_index: usize = self.0.len();
            self.0.push(Node {
                value,

                output_rate_of_change_with_respect_to_node_value: 0.0,

                operation: Operation::Input,
            });
            node_index
        }
    }

    let mut graph: Graph = Graph(vec![]);
    let input_index: usize = graph.append_input_value_node_to_computation_graph(2.0);
    let squared_index: usize = graph.append_input_value_node_to_computation_graph(
        graph.0[input_index].value * graph.0[input_index].value,
    );
    graph.0[squared_index].operation = Operation::Mul(input_index, input_index);
    let doubled_square_index: usize =
        graph.append_input_value_node_to_computation_graph(graph.0[squared_index].value * 2.0);
    graph.0[doubled_square_index].operation = Operation::Add(squared_index, squared_index);
    let output_index: usize = graph.append_input_value_node_to_computation_graph(calc_tanh(
        graph.0[doubled_square_index].value,
    ));
    graph.0[output_index].operation =
        Operation::TanhAsSignedSignalBoundedBetweenMinus1And1(doubled_square_index);
    graph.0[output_index].output_rate_of_change_with_respect_to_node_value = 1.0;
    for node_index in (0..=output_index).rev() {
        let incoming_loss_rate_of_change: f64 =
            graph.0[node_index].output_rate_of_change_with_respect_to_node_value;
        match graph.0[node_index].operation {
            Operation::Input => {}

            Operation::Add(left_index, right_index) => {
                graph.0[left_index].output_rate_of_change_with_respect_to_node_value +=
                    incoming_loss_rate_of_change;
                graph.0[right_index].output_rate_of_change_with_respect_to_node_value +=
                    incoming_loss_rate_of_change;
            }

            Operation::Mul(left_index, right_index) => {
                graph.0[left_index].output_rate_of_change_with_respect_to_node_value +=
                    incoming_loss_rate_of_change * graph.0[right_index].value;
                graph.0[right_index].output_rate_of_change_with_respect_to_node_value +=
                    incoming_loss_rate_of_change * graph.0[left_index].value;
            }

            Operation::TanhAsSignedSignalBoundedBetweenMinus1And1(left_index) => {
                let output_value: f64 = graph.0[node_index].value;
                graph.0[left_index].output_rate_of_change_with_respect_to_node_value +=
                    incoming_loss_rate_of_change * (1.0 - output_value * output_value);
            }
        }
    }
    let _ = (
        &(graph.0[output_index].value),
        &(graph.0[input_index].output_rate_of_change_with_respect_to_node_value),
    );

    // Выполняем вычисления из примера.
    let _ = graph;
}
