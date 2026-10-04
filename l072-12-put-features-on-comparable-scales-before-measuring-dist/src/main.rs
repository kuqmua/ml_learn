// Урок 12.2. Приведение признаков к сопоставимым масштабам перед измерением расстояния.
// Зачем здесь эта тема: Признак с большим числовым диапазоном может захватить расстояние независимо
//   от полезности.
// Почему код устроен так: Масштабируем координаты по train и сравниваем порядок соседей до и после.
// Представь: Разность возраста на 30 лет численно перекроет разность доли 0,2, если признаки не
//   привести к сравнимому масштабу.
//
// Что изучаем: Масштабирование признаков.
// Зачем это нужно: Признак с большими единицами измерения может захватить расстояние. Масштабируем
// координаты перед сравнением соседей.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
use l004_01_calc_squared_point_dist_by_summing_squared_coord_diffs::calc_squared_point_dist_by_summing_squared_coord_diffs;

fn main() {
    let scale: [f64; 2] = [1.0, 1000.0];
    assert!(
        scale.iter().all(|&value| value > 0.0),
        "масштабы должны быть положительными"
    );

    let example1: [f64; 2] = [1.0, 1000.0];
    let scaled_example1: [f64; 2] = [example1[0] / scale[0], example1[1] / scale[1]];
    let example2: [f64; 2] = [2.0, 1010.0];
    let scaled_example2: [f64; 2] = [example2[0] / scale[0], example2[1] / scale[1]];

    // Выполняем вычисления из примера.
    let _ = (
        calc_squared_point_dist_by_summing_squared_coord_diffs(&example1, &example2).unwrap(),
        calc_squared_point_dist_by_summing_squared_coord_diffs(&scaled_example1, &scaled_example2)
            .unwrap(),
    );
}
