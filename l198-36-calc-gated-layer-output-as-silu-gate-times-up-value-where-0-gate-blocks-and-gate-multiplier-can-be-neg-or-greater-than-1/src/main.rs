// Урок 36.5. Выход управляемого слоя: умножение двух ветвей с плавным управлением вкладом.
// Связь с принятой терминологией: Применение SwiGLU к gate и up проекциям полносвязного слоя.
// Зачем здесь эта тема: Полносвязный блок может управлять потоком признаков через отдельные ветви.
// Почему код устроен так: Считаем gate и up отдельно, затем перемножаем после нелинейности SwiGLU.
// Представь: Если gate подавлен, даже большой сигнал ветки up почти не проходит в выход.
// Одна проекция открывает gate, другая несёт значения; затем идёт выходная проекция.

use l198_36_calc_gated_layer_output_as_silu_gate_times_up_value_where_0_gate_blocks_and_gate_multiplier_can_be_neg_or_greater_than_1::calc_gated_layer_output_as_silu_gate_times_up_value_where_0_gate_blocks_and_gate_multiplier_can_be_neg_or_greater_than_1;

fn main() {
    let input: [f64; 2] = [1.0, -2.0];
    let gate: f64 = input[0] - input[1];
    let up_projection: f64 = input[0] + input[1];

    let down: f64 =
        calc_gated_layer_output_as_silu_gate_times_up_value_where_0_gate_blocks_and_gate_multiplier_can_be_neg_or_greater_than_1(
            gate,
            up_projection,
        ) * 0.5;
    assert!(down.is_finite());
}
