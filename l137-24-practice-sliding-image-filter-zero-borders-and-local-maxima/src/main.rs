// Урок 137. Соединять нулевую рамку, проход фильтра и уменьшение карты выбором локальных максимумов.
// Прослеживаем размеры 3x3 -> 5x5 -> 4x4 -> 2x2 и проверяем конкретные значения промежуточного и
// конечного результатов.

fn main() {
    let image = [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]];
    let filter = [[1.0, 0.0], [0.0, -1.0]];
    let mut padded = [[0.0; 5]; 5];
    for row in 0..3 {
        for col in 0..3 {
            padded[row + 1][col + 1] = image[row][col];
        }
    }
    let mut responses = [[0.0_f64; 4]; 4];
    for row in 0..4 {
        for col in 0..4 {
            for r in 0..2 {
                for c in 0..2 {
                    responses[row][col] += padded[row + r][col + c] * filter[r][c];
                }
            }
        }
    }
    let mut pooled = [[f64::NEG_INFINITY; 2]; 2];
    for row in 0..2 {
        for col in 0..2 {
            for r in 0..2 {
                for c in 0..2 {
                    pooled[row][col] = pooled[row][col].max(responses[2 * row + r][2 * col + c]);
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
