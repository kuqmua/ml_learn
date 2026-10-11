// Урок 137. Соединять нулевую рамку, проход фильтра и уменьшение карты выбором локальных максимумов.
// Прослеживаем размеры 3x3 -> 5x5 -> 4x4 -> 2x2 и проверяем конкретные значения промежуточного и
// конечного результатов.

fn main() {
    let image = [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]];
    let filter = [[1.0, 0.0], [0.0, -1.0]];
    let mut padded = [[0.0; 5]; 5];
    for row_index in 0..3 {
        for column_index in 0..3 {
            padded[row_index + 1][column_index + 1] = image[row_index][column_index];
        }
    }
    let mut responses = [[0.0_f64; 4]; 4];
    for row_index in 0..4 {
        for column_index in 0..4 {
            for window_row_index in 0..2 {
                for window_column_index in 0..2 {
                    responses[row_index][column_index] += padded[row_index + window_row_index]
                        [column_index + window_column_index]
                        * filter[window_row_index][window_column_index];
                }
            }
        }
    }
    let mut pooled = [[f64::NEG_INFINITY; 2]; 2];
    for row_index in 0..2 {
        for column_index in 0..2 {
            for window_row_index in 0..2 {
                for window_column_index in 0..2 {
                    pooled[row_index][column_index] = pooled[row_index][column_index].max(
                        responses[2 * row_index + window_row_index]
                            [2 * column_index + window_column_index],
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
