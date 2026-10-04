// Урок 167. Преобразовывать представление позиции в вектор запроса с помощью матрицы весов.
// Так задаём числовые признаки, по которым эта позиция будет искать подходящую информацию.

fn main() {
    let text_unit: [f64; 2] = [1.0, 2.0];
    let query_weights: [[f64; 2]; 2] = [[1.0, 0.0], [0.0, 0.5]];
    let query: [f64; 2] = [
        query_weights[0][0] * text_unit[0] + query_weights[0][1] * text_unit[1],
        query_weights[1][0] * text_unit[0] + query_weights[1][1] * text_unit[1],
    ];

    // Выполняем вычисления из примера.
    let _ = query;

    println!(
        "Представление позиции={text_unit:?}; матрица запроса={query_weights:?}; запрос={query:?}"
    );
    assert_eq!(query, [1.0, 1.0]);
    let original_score = text_unit[0] + text_unit[1];
    let query_score = query[0] + query[1];
    println!(
        "Совпадение с ключом [1,1]: без преобразования={original_score}, с преобразованием={query_score}"
    );
    assert_ne!(original_score, query_score);
}

// Чему учит этот урок:
// Учимся преобразовывать представление позиции в вектор запроса с помощью матрицы весов.
// Так задаём числовые признаки, по которым эта позиция будет искать подходящую информацию.
