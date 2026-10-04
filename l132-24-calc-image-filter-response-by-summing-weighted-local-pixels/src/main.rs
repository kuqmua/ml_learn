// Урок 132. Получать отклик фильтра как сумму попарных произведений весов и пикселей участка.
// Один результат описывает локальный рисунок, на который реагируют выбранные веса.

fn main() {
    let patch: [[f64; 2]; 2] = [[1.0, 2.0], [3.0, 4.0]];
    let filter_weights: [[f64; 2]; 2] = [[1.0, 0.0], [0.0, -1.0]];
    let mut response: f64 = 0.0;
    for row in 0..2 {
        for column in 0..2 {
            response += patch[row][column] * filter_weights[row][column];
        }
    }

    // Выполняем вычисления из примера.
    let _ = (&patch, &response);

    println!("Участок={patch:?}; фильтр={filter_weights:?}; отклик={response}");
    assert_eq!(response, -3.0);
    let uniform = [[4.0_f64; 2]; 2];
    let uniform_response = (0..2)
        .flat_map(|row| {
            (0..2).map(move |column| uniform[row][column] * filter_weights[row][column])
        })
        .sum::<f64>();
    assert_eq!(uniform_response, 0.0);
    println!("На одинаковых пикселях отклик={uniform_response}: фильтр реагирует на разницу.");
}

// Чему учит этот урок:
// Учимся получать отклик фильтра как сумму попарных произведений весов и пикселей участка.
// Один результат описывает локальный рисунок, на который реагируют выбранные веса.
