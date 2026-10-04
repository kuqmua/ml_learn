// Урок 071. Применять квадрат расстояния между координатами к сравнению объектов по признакам.
// Это первый шаг поиска похожих примеров: меньший результат означает более близкого кандидата.

use l004_01_calc_squared_point_dist_by_summing_squared_coord_diffs::calc_squared_point_dist_by_summing_squared_coord_diffs;

fn main() {
    let query: [f64; 2] = [1.0, 2.0];
    let candidates: [[f64; 2]; 2] = [[2.0, 2.0], [4.0, 6.0]];
    for candidate in candidates {
        let _: f64 = calc_squared_point_dist_by_summing_squared_coord_diffs(&query, &candidate)
            .expect("координаты должны быть конечными, а квадрат расстояния — помещаться в f64");
    }

    let distances = candidates.map(|candidate| {
        calc_squared_point_dist_by_summing_squared_coord_diffs(&query, &candidate).unwrap()
    });
    println!("Запрос={query:?}; кандидаты={candidates:?}; квадраты расстояний={distances:?}");
    assert!(distances[0] < distances[1]);
    println!("Первый кандидат ближе; извлекать корень для сравнения не требуется.");
}

// Чему учит этот урок:
// Учимся применять квадрат расстояния между координатами к сравнению объектов по признакам.
// Это первый шаг поиска похожих примеров: меньший результат означает более близкого кандидата.
