// Урок 132. Получать отклик фильтра как сумму попарных произведений весов и пикселей участка.
// Один результат описывает локальный рисунок, на который реагируют выбранные веса.

fn main() {
    let patch: [[f64; 2]; 2] = [[1.0, 2.0], [3.0, 4.0]];
    let filter_weights: [[f64; 2]; 2] = [[1.0, 0.0], [0.0, -1.0]];
    let mut response: f64 = 0.0;
    for first_dimension_index in 0..2 {
        for second_dimension_index in 0..2 {
            response += patch[first_dimension_index][second_dimension_index]
                * filter_weights[first_dimension_index][second_dimension_index];
        }
    }

    // Выполняем вычисления из примера.
    let _ = (&patch, &response);

    println!("Участок={patch:?}; фильтр={filter_weights:?}; отклик={response}");
    assert_eq!(response, -3.0);
    let uniform = [[4.0_f64; 2]; 2];
    let uniform_response = (0..2)
        .flat_map(|first_dimension_index| {
            (0..2).map(move |second_dimension_index| {
                uniform[first_dimension_index][second_dimension_index]
                    * filter_weights[first_dimension_index][second_dimension_index]
            })
        })
        .sum::<f64>();
    assert_eq!(uniform_response, 0.0);
    println!("На одинаковых пикселях отклик={uniform_response}: фильтр реагирует на разницу.");
}

// Чему учит этот урок:
// Учимся получать отклик фильтра как сумму попарных произведений весов и пикселей участка.
// Один результат описывает локальный рисунок, на который реагируют выбранные веса.
