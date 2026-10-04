// Урок 035. Составлять повторные выборки с повторением элементов и считать их средние.
// Заданные наборы индексов показывают, как результат зависит от состава выборки; полноценного
// случайного бутстрэпа здесь нет.

fn main() {
    let values: [f64; 3] = [2.0, 4.0, 6.0];
    assert!(!values.is_empty(), "исходная выборка не должна быть пустой");
    let resamples: [[usize; 3]; 4] = [[0, 1, 2], [0, 0, 2], [1, 2, 2], [0, 1, 1]];
    for indices in resamples {
        assert!(
            !indices.is_empty(),
            "повторная выборка не должна быть пустой"
        );
        assert!(
            indices.iter().all(|&index| index < values.len()),
            "индекс выходит за границы исходной выборки"
        );
        let mut sum_of_resampled_values: f64 = 0.0;
        for index in indices {
            sum_of_resampled_values += values[index];
        }
        let _: f64 = sum_of_resampled_values / indices.len() as f64;
    }

    // Выполняем вычисления из примера.
    let _ = (&values, &resamples);

    let means = resamples
        .map(|indices| indices.iter().map(|&i| values[i]).sum::<f64>() / indices.len() as f64);
    println!("Исходные значения={values:?}; средние повторных выборок={means:?}");
    assert!(means.iter().any(|&value| value != means[0]));
}

// Чему учит этот урок:
// Учимся составлять повторные выборки с повторением элементов и считать их средние.
// Заданные наборы индексов показывают, как результат зависит от состава выборки; полноценного
// случайного бутстрэпа здесь нет.
