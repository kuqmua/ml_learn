// Урок 137. Соединять нулевую рамку, проход фильтра и уменьшение карты выбором локальных максимумов.
// Прослеживаем размеры 3x3 -> 5x5 -> 4x4 -> 2x2 и проверяем конкретные значения промежуточного и
// конечного результатов.

fn main() {
    let image = [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]];
    let filter = [[1.0, 0.0], [0.0, -1.0]];
    let mut padded = [[0.0; 5]; 5];
    for first_dimension_index in 0..3 {
        for second_dimension_index in 0..3 {
            padded[first_dimension_index + 1][second_dimension_index + 1] =
                image[first_dimension_index][second_dimension_index];
        }
    }
    let mut responses = [[0.0_f64; 4]; 4];
    for first_dimension_index in 0..4 {
        for second_dimension_index in 0..4 {
            for window_first_dimension_index in 0..2 {
                for window_second_dimension_index in 0..2 {
                    responses[first_dimension_index][second_dimension_index] += padded
                        [first_dimension_index + window_first_dimension_index]
                        [second_dimension_index + window_second_dimension_index]
                        * filter[window_first_dimension_index][window_second_dimension_index];
                }
            }
        }
    }
    let mut pooled = [[f64::NEG_INFINITY; 2]; 2];
    for first_dimension_index in 0..2 {
        for second_dimension_index in 0..2 {
            for window_first_dimension_index in 0..2 {
                for window_second_dimension_index in 0..2 {
                    pooled[first_dimension_index][second_dimension_index] =
                        pooled[first_dimension_index][second_dimension_index].max(
                            responses[2 * first_dimension_index + window_first_dimension_index]
                                [2 * second_dimension_index + window_second_dimension_index],
                        );
                }
            }
        }
    }
    println!(
        "Изображение 3x3={image:?}\nС рамкой 5x5={padded:?}\nОтклики 4x4={responses:?}\nМаксимумы 2x2={pooled:?}"
    );
    assert_eq!(responses[1][1], -4.0);
    assert_eq!(pooled, [[-1.0, 3.0], [7.0, 9.0]]);
}

// Чему учит этот урок:
// Учимся соединять нулевую рамку, проход фильтра и уменьшение карты выбором локальных максимумов.
// Прослеживаем размеры 3x3 -> 5x5 -> 4x4 -> 2x2 и проверяем конкретные значения промежуточного и
// конечного результатов.
