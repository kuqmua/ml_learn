// Урок 26.3. Управляемый выход сигнала: умножение ограниченного сигнала на долю, задаваемую второй ветвью.
// Зачем здесь эта тема: Сигналу может требоваться пропускать или подавлять найденный фильтром
//   признак.
// Почему код устроен так: Умножаем ветку tanh на сигмоидальные ворота и смотрим влияние gate.
// Представь: Если gate близок к нулю, выход ветки фильтра почти подавляется.
// Одна ветка tanh создаёт сигнал, другая sigmoid управляет его пропусканием.

/// Сигмоида: 1 / (1 + e^(−score)); число от 0 до 1 — вероятность класса или доля пропускаемого сигнала.

fn calc_sigmoid_as_one_divided_by_one_plus_e_to_neg_score(value: f64) -> f64 {
    1.0 / (1.0 + (-value).exp())
}
/// Управляемая активация WaveNet: tanh(filter)·sigmoid(gate).
fn calc_gated_signal_as_tanh_times_sigmoid_share(filter: f64, raw_gate_score: f64) -> f64 {
    calc_tanh(filter) * calc_sigmoid_as_one_divided_by_one_plus_e_to_neg_score(raw_gate_score)
}
fn main() {
    let filter: f64 = 1.5;

    let signal_with_gate_almost_closed: f64 =
        calc_gated_signal_as_tanh_times_sigmoid_share(filter, -5.0);
    assert!(
        calc_gated_signal_as_tanh_times_sigmoid_share(filter, 5.0,)
            > signal_with_gate_almost_closed
    );
    assert!(signal_with_gate_almost_closed >= 0.0);

    // Выполняем вычисления из примера.
    let _ = filter;
}

/// tanh сохраняет знак, равен 0 при нулевом входе и насыщается к −1 или 1.
fn calc_tanh(input: f64) -> f64 {
    input.tanh()
}

// Чему учит этот урок:
// Учимся умножать ограниченный сигнал на долю пропускания между 0 и 1.
// Так можно ослаблять или почти полностью пропускать сигнал, управляя отдельной числовой оценкой.
