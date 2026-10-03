// Урок 155. Каждому слову уже сопоставлен список чисел. Сравниваем два таких списка:
// умножаем числа на одинаковых местах и складываем результаты.
// Большой положительный ответ может означать похожие направления, но зависит и от длин списков-стрелок.
// Для сравнения только направлений нужно дополнительно разделить ответ на обе длины.

use l001_01_multiply_matching_coords_then_add_results::multiply_matching_coords_then_add_results;

fn main() {
    let first_word_vec: [f64; 2] = [0.8, 0.2];
    let second_word_vec: [f64; 2] = [0.7, 0.3];
    let _: f64 = multiply_matching_coords_then_add_results(&first_word_vec, &second_word_vec)
        .expect("представления должны иметь одинаковое число координат");

    // Выполняем вычисления из примера.
    let _ = (first_word_vec, second_word_vec);
}
