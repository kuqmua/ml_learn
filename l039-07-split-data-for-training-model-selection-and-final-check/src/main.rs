// Урок 039. Разделять данные на части для обучения, выбора решения и окончательной проверки.
// Разные роли наборов помогают не подбирать модель по тем же ответам, на которых заявляется
// итоговое качество.

fn main() {
    let rows: [i32; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let training_data: &[i32] = &rows[..6];
    let validation: &[i32] = &rows[6..8];
    let test: &[i32] = &rows[8..];

    // Выполняем вычисления из примера.
    let _ = (&training_data, &validation, &test);

    println!("Обучение={training_data:?}; выбор настроек={validation:?}; итоговый тест={test:?}");
    assert!(
        training_data
            .iter()
            .all(|x| !validation.contains(x) && !test.contains(x))
    );
    assert!(validation.iter().all(|x| !test.contains(x)));
    assert_eq!(
        training_data.len() + validation.len() + test.len(),
        rows.len()
    );
}

// Чему учит этот урок:
// Учимся разделять данные на части для обучения, выбора решения и окончательной проверки.
// Разные роли наборов помогают не подбирать модель по тем же ответам, на которых заявляется
// итоговое качество.
