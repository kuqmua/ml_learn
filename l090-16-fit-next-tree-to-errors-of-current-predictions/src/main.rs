// Урок 090. Бустинг для регрессии с квадратичной ошибкой.
// Предсказываем аренду по площади квартиры на ВЫДУМАННЫХ учебных данных.
// Новое дерево предсказывает поправку к общей сумме, а не цену квартиры.
// Здесь заранее фиксируем порог дерева, чтобы отдельно разобрать обучение листьев.
// В полноценном бустинге обычно подбирают и разбиения каждого нового дерева.
fn main() {
    // Одинаковый индекс означает одну квартиру в обоих массивах.
    // Площадь — входной признак x, аренда — правильный ответ y.
    // apartment_areas_square_meters — Площади восьми квартир в м²; по ним дерево выбирает левый или правый лист.
    let apartment_areas_square_meters: [f64; 8] = [20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0, 90.0];
    // actual_monthly_rents_thousand_rubles — Известная аренда каждой квартиры в тыс. рублей в месяц — правильные ответы для обучения.
    let actual_monthly_rents_thousand_rubles: [f64; 8] = [20.0, 24.0, 28.0, 32.0, 48.0, 52.0, 56.0, 60.0];
    // Что здесь называют деревом: правило «площадь квартиры ≤ 55 м²?» с двумя вариантами ответа.
    // Для квартир до 55 м² включительно правило выдаёт одну поправку к аренде, для остальных — другую.
    // Эти две группы называют листьями дерева. Поправки мы вычислим ниже по ошибкам прогнозов.
    // Например, сначала прогнозируем всем аренду 40 тыс. рублей, а средняя настоящая аренда
    // квартир первой группы равна 26 тыс. рублей. Нужная поправка: 26 − 40 = −14 тыс. рублей.
    // fraction_of_tree_correction_to_apply — какую часть вычисленной поправки применить к прогнозу.
    // 0.5 означает половину: прибавим не −14, а −7. Новый прогноз: 40 + 0.5 × (−14) = 33.
    // При значении 1.0 применили бы всю поправку и получили 26; при 0.0 оставили бы прогноз 40.
    // Это число выбираем сами; в этом примере берём половину, чтобы показать постепенные изменения.
    let fraction_of_tree_correction_to_apply = 0.5;
    // area_split_threshold_square_meters — Граница площади в м²: квартиры с площадью ≤ 55 идут в левый лист, остальные — в правый.
    let area_split_threshold_square_meters = 55.0;
    // Пока деревьев нет, всем отвечаем одним числом — средним по обучающим ответам.
    // Именно среднее минимизирует сумму квадратов ошибок постоянного прогноза.
    // initial_mean_rent_thousand_rubles — Начальный общий прогноз аренды: среднее всех ответов, здесь 40 тыс. рублей.
    let initial_mean_rent_thousand_rubles = actual_monthly_rents_thousand_rubles.iter().sum::<f64>() / actual_monthly_rents_thousand_rubles.len() as f64;
    // predicted_monthly_rents_thousand_rubles — Текущие прогнозы аренды для восьми квартир; меняются после добавления каждого дерева.
    let mut predicted_monthly_rents_thousand_rubles = [initial_mean_rent_thousand_rubles; 8];
    // mean_squared_error_before_training — Средний квадрат ошибки до первого дерева, в (тыс. рублей)²; здесь 216.
    // actual_monthly_rent_thousand_rubles внутри map — ссылка на правильную аренду одной квартиры; map вычисляет квадрат её ошибки.
    let mean_squared_error_before_training = actual_monthly_rents_thousand_rubles
        .iter()
        .map(|actual_monthly_rent_thousand_rubles| (actual_monthly_rent_thousand_rubles - initial_mean_rent_thousand_rubles).powi(2))
        .sum::<f64>()
        / actual_monthly_rents_thousand_rubles.len() as f64;
    // mean_squared_error_after_each_round — История MSE: сначала ошибка без деревьев, затем ошибка после каждого раунда.
    let mut mean_squared_error_after_each_round = vec![mean_squared_error_before_training];
    // Повторяем обучение три раза; символ _ означает, что номер раунда не используется.
    for _ in 1..=3 {
        // Остаток имеет знак! +20: надо повысить прогноз на 20; −20: понизить.
        // Это НЕ квадрат ошибки и НЕ модуль: без знака потеряем направление поправки.
        // На каждом раунде считаем остатки заново от всей текущей суммы прогнозов.
        // actual_minus_predicted_rents_thousand_rubles — Восемь остатков «ответ − прогноз» в тыс. рублей; apartment_index внутри from_fn — индекс квартиры от 0 до 7.
        let actual_minus_predicted_rents_thousand_rubles: [f64; 8] = std::array::from_fn(|apartment_index| {
            actual_monthly_rents_thousand_rubles[apartment_index] - predicted_monthly_rents_thousand_rubles[apartment_index]
        });
        // Обучаем два листа: собираем остатки квартир по ПРИЗНАКУ, а не по индексу.
        // Среднее остатка — лучшая постоянная поправка группы по квадратичной ошибке.
        // sum_of_left_leaf_rent_residuals — Сумма остатков квартир левого листа в тыс. рублей; на каждом раунде собирается заново.
        let mut sum_of_left_leaf_rent_residuals = 0.0;
        // sum_of_right_leaf_rent_residuals — Сумма остатков квартир правого листа в тыс. рублей; на каждом раунде собирается заново.
        let mut sum_of_right_leaf_rent_residuals = 0.0;
        // apartments_in_left_leaf_count — Количество квартир, попавших в левый лист; нужно для вычисления среднего остатка.
        let mut apartments_in_left_leaf_count = 0;
        // apartments_in_right_leaf_count — Количество квартир, попавших в правый лист; нужно для вычисления среднего остатка.
        let mut apartments_in_right_leaf_count = 0;
        // apartment_index — индекс текущей квартиры от 0 до 7; связывает её площадь, ответ и прогноз.
        for apartment_index in 0..apartment_areas_square_meters.len() {
            if apartment_areas_square_meters[apartment_index] <= area_split_threshold_square_meters {
                sum_of_left_leaf_rent_residuals += actual_minus_predicted_rents_thousand_rubles[apartment_index];
                apartments_in_left_leaf_count += 1;
            } else {
                sum_of_right_leaf_rent_residuals += actual_minus_predicted_rents_thousand_rubles[apartment_index];
                apartments_in_right_leaf_count += 1;
            }
        }
        assert!(
            apartments_in_left_leaf_count > 0 && apartments_in_right_leaf_count > 0,
            "порог должен давать два непустых листа"
        );
        // left_leaf_rent_correction_thousand_rubles — Средний остаток левого листа — предлагаемая деревом поправка в тыс. рублей до умножения на скорость.
        let left_leaf_rent_correction_thousand_rubles = sum_of_left_leaf_rent_residuals / apartments_in_left_leaf_count as f64;
        // right_leaf_rent_correction_thousand_rubles — Средний остаток правого листа — предлагаемая деревом поправка в тыс. рублей до умножения на скорость.
        let right_leaf_rent_correction_thousand_rubles = sum_of_right_leaf_rent_residuals / apartments_in_right_leaf_count as f64;
        // Бустинг складывает модели: F_new(x) = F_old(x) + fraction_of_tree_correction_to_apply * tree(x).
        // Не заменяем predicted_monthly_rents_thousand_rubles ответом нового дерева: он всего лишь поправка.
        // apartment_index — индекс текущей квартиры от 0 до 7; связывает её площадь, ответ и прогноз.
        for apartment_index in 0..apartment_areas_square_meters.len() {
            // tree_rent_correction_for_apartment — Ответ выбранного листа для текущей квартиры: поправка дерева в тыс. рублей со своим знаком.
            let tree_rent_correction_for_apartment = if apartment_areas_square_meters[apartment_index] <= area_split_threshold_square_meters {
                left_leaf_rent_correction_thousand_rubles
            } else {
                right_leaf_rent_correction_thousand_rubles
            };
            // rent_correction_after_learning_rate — Фактически добавляемая поправка в тыс. рублей: скорость обучения × ответ дерева.
            let rent_correction_after_learning_rate = fraction_of_tree_correction_to_apply * tree_rent_correction_for_apartment;
            predicted_monthly_rents_thousand_rubles[apartment_index] += rent_correction_after_learning_rate;
        }
        // Проверяем всю сумму моделей по исходным ответам, а не по остаткам дерева.
        // mean_squared_error_after_current_tree — Средний квадрат ошибки всей модели после текущего дерева, в (тыс. рублей)².
        let mean_squared_error_after_current_tree = actual_monthly_rents_thousand_rubles
            .iter()
            .zip(predicted_monthly_rents_thousand_rubles)
            // actual_monthly_rent_thousand_rubles — ссылка на правильную аренду; predicted_monthly_rent_thousand_rubles — текущий прогноз той же квартиры, оба в тыс. рублей.
            .map(|(actual_monthly_rent_thousand_rubles, predicted_monthly_rent_thousand_rubles)| {
                (actual_monthly_rent_thousand_rubles - predicted_monthly_rent_thousand_rubles).powi(2)
            })
            .sum::<f64>()
            / actual_monthly_rents_thousand_rubles.len() as f64;
        // mean_squared_error_before_current_tree — Последняя сохранённая MSE — ошибка до текущего обновления, для сравнения с новой.
        let mean_squared_error_before_current_tree = mean_squared_error_after_each_round[mean_squared_error_after_each_round.len() - 1];
        assert!(
            mean_squared_error_after_current_tree < mean_squared_error_before_current_tree,
            "в этом примере каждый раунд уменьшает MSE"
        );
        mean_squared_error_after_each_round.push(mean_squared_error_after_current_tree);
    }
    // Эти числа рассчитаны вручную в README и защищают пример от случайных ошибок.
    assert_eq!(
        predicted_monthly_rents_thousand_rubles,
        [27.75, 27.75, 27.75, 27.75, 52.25, 52.25, 52.25, 52.25]
    );
    assert_eq!(mean_squared_error_after_each_round, [216.0, 69.0, 32.25, 23.0625]);
}
