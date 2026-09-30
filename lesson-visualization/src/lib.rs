//! SVG-графики для учебных примеров.

/// Подписанный ряд координат для линейного графика или диаграммы рассеяния.
pub struct Series<'a> {
    /// Подпись ряда в легенде.
    pub name: &'a str,
    /// Пары координат (x, y), рассчитанные в уроке.
    pub points: &'a [(f64, f64)],
}

/// Строит линейный SVG-график и возвращает путь к файлу.
pub fn line_chart(
    // Путь к пакету урока определяет место сохранения SVG.
    lesson_directory: &str,
    // Безопасное имя выходного файла без расширения.
    name: &str,
    // Заголовок, видимый над рисунком.
    title: &str,
    // Название величины на горизонтальной оси.
    horizontal_axis_label: &str,
    // Название величины на вертикальной оси.
    vertical_axis_label: &str,
    // Наборы точек с подписями для легенды.
    series: &[Series<'_>],
) -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
    // Передаём точки в общую отрисовку с соединяющими линиями.
    draw_chart(
        lesson_directory,
        name,
        title,
        horizontal_axis_label,
        vertical_axis_label,
        series,
        true,
    )
}

/// Строит диаграмму рассеяния и возвращает путь к SVG.
pub fn scatter_chart(
    // Путь к пакету урока определяет место сохранения SVG.
    lesson_directory: &str,
    // Безопасное имя выходного файла без расширения.
    name: &str,
    // Заголовок, видимый над рисунком.
    title: &str,
    // Название величины на горизонтальной оси.
    horizontal_axis_label: &str,
    // Название величины на вертикальной оси.
    vertical_axis_label: &str,
    // Наборы точек с подписями для легенды.
    series: &[Series<'_>],
) -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
    // Передаём точки в общую отрисовку без соединяющих линий.
    draw_chart(
        lesson_directory,
        name,
        title,
        horizontal_axis_label,
        vertical_axis_label,
        series,
        false,
    )
}

// Общая отрисовка: линии включаются только для линейного графика.
fn draw_chart(
    // Путь к пакету урока определяет место сохранения SVG.
    lesson_directory: &str,
    // Безопасное имя выходного файла без расширения.
    name: &str,
    // Заголовок, видимый над рисунком.
    title: &str,
    // Название величины на горизонтальной оси.
    horizontal_axis_label: &str,
    // Название величины на вертикальной оси.
    vertical_axis_label: &str,
    // Наборы точек с подписями для легенды.
    series: &[Series<'_>],
    // Соединять точки линией или показывать их отдельно.
    connect_points: bool,
) -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
    // Проверяем имя файла и подготавливаем каталог visualizations.
    let path = output_path(lesson_directory, name)?;
    // Собираем только конечные координаты для выбора диапазона осей.
    let points: Vec<(f64, f64)> = series
        // Просматриваем элементы коллекции по ссылке.
        .iter()
        // Объединяем вложенные последовательности.
        .flat_map(|series| series.points.iter().copied())
        // Оставляем элементы, отвечающие условию.
        .filter(|(horizontal_value, vertical_value)| {
            horizontal_value.is_finite() && vertical_value.is_finite()
        })
        // Собираем результаты в коллекцию.
        .collect();
    // Пустому графику невозможно назначить диапазон осей.
    if points.is_empty() {
        // Прерываем вычисление и возвращаем причину ошибки.
        return Err("нет конечных точек для графика".into());
    }
    // Определяем диапазоны осей с небольшими полями.
    let ((horizontal_minimum, horizontal_maximum), (vertical_minimum, vertical_maximum)) =
        bounds(&points);
    // Создаём SVG-холст заданного размера.
    let root = plotters::prelude::IntoDrawingArea::into_drawing_area(
        plotters::prelude::SVGBackend::new(&path, (900, 560)),
    );
    // Делаем фон графика белым.
    root.fill(&plotters::prelude::WHITE)?;
    // Настраиваем область построения, заголовок и место для подписей.
    let mut chart = plotters::prelude::ChartBuilder::on(&root)
        // Задаём заголовок графика.
        .caption(title, ("sans-serif", 24))
        // Оставляем внешнее поле вокруг рисунка.
        .margin(20)
        // Выделяем место для подписей горизонтальной оси.
        .x_label_area_size(48)
        // Выделяем место для подписей вертикальной оси.
        .y_label_area_size(65)
        // Устанавливаем числовые диапазоны обеих осей.
        .build_cartesian_2d(
            horizontal_minimum..horizontal_maximum,
            vertical_minimum..vertical_maximum,
        )?;
    chart
        // Настраиваем оси и координатную сетку.
        .configure_mesh()
        // Подписываем горизонтальную ось.
        .x_desc(horizontal_axis_label)
        // Подписываем вертикальную ось.
        .y_desc(vertical_axis_label)
        // Наносим подготовленные элементы на график.
        .draw()?;
    // Рисуем каждый подписанный ряд своим цветом.
    for (index, item) in series.iter().enumerate() {
        // Номер ряда определяет цвет из палитры Plotters.
        let color = <plotters::prelude::Palette99 as plotters::prelude::Palette>::pick(index);
        // Для временного ряда или функции соединяем соседние точки.
        if connect_points {
            chart
                // Рисуем следующий ряд значений.
                .draw_series(plotters::prelude::LineSeries::new(
                    item.points
                        // Просматриваем элементы коллекции по ссылке.
                        .iter()
                        // Берём собственные пары чисел из ссылок на точки.
                        .copied()
                        // Оставляем элементы, отвечающие условию.
                        .filter(|(horizontal_value, vertical_value)| {
                            horizontal_value.is_finite() && vertical_value.is_finite()
                        }),
                    // Окрашиваем линию цветом текущего ряда.
                    &color,
                ))?
                // Добавляем название ряда в легенду.
                .label(item.name)
                // Показываем обозначение ряда в легенде.
                .legend(move |(horizontal_value, vertical_value)| {
                    // Вычисляем значение по указанной формуле.
                    plotters::prelude::PathElement::new(
                        vec![
                            (horizontal_value, vertical_value),
                            (horizontal_value + 20, vertical_value),
                        ],
                        <plotters::prelude::Palette99 as plotters::prelude::Palette>::pick(index),
                    )
                });
        }
        // Маркеры сохраняют видимость отдельных наблюдений.
        let dots = chart.draw_series(
            item.points
                // Просматриваем элементы коллекции по ссылке.
                .iter()
                // Берём собственные пары чисел из ссылок на точки.
                .copied()
                // Оставляем элементы, отвечающие условию.
                .filter(|(horizontal_value, vertical_value)| {
                    horizontal_value.is_finite() && vertical_value.is_finite()
                })
                // Преобразуем каждый элемент в новое значение.
                .map(|point| {
                    plotters::prelude::Circle::new(
                        point,
                        4,
                        plotters::prelude::Color::filled(&color),
                    )
                }),
        )?;
        // Для отдельных наблюдений показываем в легенде маркер точки.
        if !connect_points {
            // Подписываем маркер точек в легенде.
            dots.label(item.name)
                // Показываем обозначение ряда в легенде.
                .legend(move |(horizontal_value, vertical_value)| {
                    plotters::prelude::Circle::new(
                        (horizontal_value + 10, vertical_value),
                        4,
                        plotters::prelude::Color::filled(
                            &<plotters::prelude::Palette99 as plotters::prelude::Palette>::pick(
                                index,
                            ),
                        ),
                    )
                });
        }
    }
    // Легенда нужна, когда сравниваются несколько рядов.
    if series.len() > 1 {
        // Рисуем легенду с рамкой после добавления рядов.
        chart
            .configure_series_labels()
            .border_style(plotters::prelude::BLACK)
            .draw()?;
    }
    // Завершаем запись SVG на диск.
    root.present()?;
    // Завершаем использование холста перед возвратом пути.
    drop(chart);
    // Завершаем использование холста перед возвратом пути.
    drop(root);
    // Возвращаем путь к сохранённому графику.
    Ok(path)
}

/// Строит столбчатую диаграмму из подписанных значений.
pub fn bar_chart(
    // Путь к пакету урока определяет место сохранения SVG.
    lesson_directory: &str,
    // Безопасное имя выходного файла без расширения.
    name: &str,
    // Заголовок, видимый над рисунком.
    title: &str,
    // Название величины на вертикальной оси.
    vertical_axis_label: &str,
    // Подписанные значения для столбцов.
    values: &[(&str, f64)],
) -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
    // Проверяем имя файла и подготавливаем каталог visualizations.
    let path = output_path(lesson_directory, name)?;
    // Пустые и нечисловые данные невозможно показать на диаграмме.
    if values.is_empty()
        || values
            .iter()
            .any(|(_, element_value)| !element_value.is_finite())
    {
        // Прерываем вычисление и возвращаем причину ошибки.
        return Err("для столбцов нужны конечные значения".into());
    }
    // Нижняя граница цвета или вертикальной оси.
    let minimum_value = values
        .iter()
        .map(|(_, element_value)| *element_value)
        .fold(0.0_f64, f64::min);
    // Верхняя граница цвета или вертикальной оси.
    let maximum_value = values
        .iter()
        .map(|(_, element_value)| *element_value)
        .fold(0.0_f64, f64::max);
    // Добавляем поле, чтобы крайний столбец не касался рамки.
    let axis_padding = (maximum_value - minimum_value).max(1.0) * 0.1;
    // Создаём SVG-холст заданного размера.
    let root = plotters::prelude::IntoDrawingArea::into_drawing_area(
        plotters::prelude::SVGBackend::new(&path, (900, 560)),
    );
    // Делаем фон графика белым.
    root.fill(&plotters::prelude::WHITE)?;
    // Настраиваем область построения, заголовок и место для подписей.
    let mut chart = plotters::prelude::ChartBuilder::on(&root)
        // Задаём заголовок графика.
        .caption(title, ("sans-serif", 24))
        // Оставляем внешнее поле вокруг рисунка.
        .margin(20)
        // Выделяем место для подписей горизонтальной оси.
        .x_label_area_size(90)
        // Выделяем место для подписей вертикальной оси.
        .y_label_area_size(65)
        // Устанавливаем числовые диапазоны обеих осей.
        .build_cartesian_2d(
            -0.5..values.len() as f64 - 0.5,
            minimum_value - axis_padding..maximum_value + axis_padding,
        )?;
    chart
        // Настраиваем оси и координатную сетку.
        .configure_mesh()
        // Убираем вертикальные линии сетки между столбцами.
        .disable_x_mesh()
        // Число подписей соответствует числу столбцов.
        .x_labels(values.len())
        // Заменяем числовые позиции подписями категорий.
        .x_label_formatter(&|input_value| {
            // Сохраняем результат этого шага в `index`.
            let index = input_value.round();
            // Подписываем только целые позиции существующих категорий.
            if (input_value - index).abs() > 0.15 || index < 0.0 {
                // Завершаем вычисление с полученным результатом.
                return String::new();
            }
            values
                // Берём название категории по номеру столбца.
                .get(index as usize)
                // Преобразуем каждый элемент в новое значение.
                .map(|(label, _)| (*label).to_string())
                // Вне диапазона категорий оставляем пустую подпись.
                .unwrap_or_default()
        })
        // Подписываем вертикальную ось.
        .y_desc(vertical_axis_label)
        // Наносим подготовленные элементы на график.
        .draw()?;
    // Строим один столбец для каждого подписанного значения.
    for (item_index, (_, value)) in values.iter().enumerate() {
        // Рисуем цветной прямоугольник на соответствующих координатах.
        chart.draw_series(std::iter::once(plotters::prelude::Rectangle::new(
            // Задаём значения следующей строки или последовательности.
            [
                (item_index as f64 - 0.35, 0.0),
                (item_index as f64 + 0.35, *value),
            ],
            // Задаём именованное поле или параметр.
            plotters::prelude::Color::filled(
                &<plotters::prelude::Palette99 as plotters::prelude::Palette>::pick(item_index),
            ),
        )))?;
    }
    // Завершаем запись SVG на диск.
    root.present()?;
    // Завершаем использование холста перед возвратом пути.
    drop(chart);
    // Завершаем использование холста перед возвратом пути.
    drop(root);
    // Возвращаем путь к сохранённому графику.
    Ok(path)
}

/// Показывает значения матрицы цветом: светлая ячейка — меньшее значение.
pub fn heatmap(
    // Путь к пакету урока определяет место сохранения SVG.
    lesson_directory: &str,
    // Безопасное имя выходного файла без расширения.
    name: &str,
    // Заголовок, видимый над рисунком.
    title: &str,
    // Прямоугольная таблица чисел для окрашивания ячеек.
    values: &[Vec<f64>],
) -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
    // Проверяем имя файла и подготавливаем каталог visualizations.
    let path = output_path(lesson_directory, name)?;
    // Количество строк задаёт высоту сетки.
    let rows = values.len();
    // Длина первой строки задаёт ширину сетки.
    let columns = values.first().map_or(0, Vec::len);
    // Проверяем непустую прямоугольную форму и конечность ячеек.
    if rows == 0
        // Задаём преобразование для элементов коллекции.
        || columns == 0
        // Задаём преобразование для элементов коллекции.
        || values
            // Просматриваем элементы коллекции по ссылке.
            .iter()
            // Отклоняем рваные строки и нечисловые значения.
            .any(|row| row.len() != columns || row.iter().any(|element_value| !element_value.is_finite()))
    {
        // Прерываем вычисление и возвращаем причину ошибки.
        return Err("матрица должна быть прямоугольной и содержать конечные значения".into());
    }
    // Нижняя граница цвета или вертикальной оси.
    let minimum_value = values
        // Просматриваем элементы коллекции по ссылке.
        .iter()
        // Объединяем все ячейки матрицы в один поток чисел.
        .flatten()
        // Берём собственные пары чисел из ссылок на точки.
        .copied()
        // Находим наименьшее значение для цветовой шкалы.
        .fold(f64::INFINITY, f64::min);
    // Верхняя граница цвета или вертикальной оси.
    let maximum_value = values
        // Просматриваем элементы коллекции по ссылке.
        .iter()
        // Объединяем все ячейки матрицы в один поток чисел.
        .flatten()
        // Берём собственные пары чисел из ссылок на точки.
        .copied()
        // Находим наибольшее значение для цветовой шкалы.
        .fold(f64::NEG_INFINITY, f64::max);
    // Создаём SVG-холст заданного размера.
    let root = plotters::prelude::IntoDrawingArea::into_drawing_area(
        plotters::prelude::SVGBackend::new(&path, (700, 620)),
    );
    // Делаем фон графика белым.
    root.fill(&plotters::prelude::WHITE)?;
    // Настраиваем область построения, заголовок и место для подписей.
    let mut chart = plotters::prelude::ChartBuilder::on(&root)
        // Задаём заголовок графика.
        .caption(title, ("sans-serif", 24))
        // Оставляем внешнее поле вокруг рисунка.
        .margin(30)
        // Выделяем место для подписей горизонтальной оси.
        .x_label_area_size(40)
        // Выделяем место для подписей вертикальной оси.
        .y_label_area_size(40)
        // Устанавливаем числовые диапазоны обеих осей.
        .build_cartesian_2d(0.0..columns as f64, 0.0..rows as f64)?;
    chart
        // Настраиваем оси и координатную сетку.
        .configure_mesh()
        // Скрываем сетку поверх цветных ячеек.
        .disable_mesh()
        // Подписываем горизонтальную ось.
        .x_desc("Столбец")
        // Подписываем вертикальную ось.
        .y_desc("Строка")
        // Наносим подготовленные элементы на график.
        .draw()?;
    // Проходим все ячейки в исходном порядке строк.
    for (row_index, row) in values.iter().enumerate() {
        // Рисуем каждую ячейку своим цветом и числом.
        for (column_index, &value) in row.iter().enumerate() {
            // Нормируем значение ячейки в диапазон от нуля до единицы.
            // Долю положения между минимумом и максимумом называют fraction диапазона.
            let position_within_value_range = if maximum_value == minimum_value {
                // Для одинаковых ячеек используем середину цветовой шкалы.
                0.5
            // Нормируем разные значения по минимуму и максимуму.
            } else {
                // Вычисляем значение по указанной формуле.
                (value - minimum_value) / (maximum_value - minimum_value)
            };
            // Номер ряда определяет цвет из палитры Plotters.
            let color = plotters::prelude::RGBColor(
                // Вычисляем значение по указанной формуле.
                (35.0 + 180.0 * position_within_value_range) as u8,
                // Вычисляем значение по указанной формуле.
                (80.0 + 90.0 * (1.0 - position_within_value_range)) as u8,
                // Вычисляем значение по указанной формуле.
                (220.0 - 160.0 * position_within_value_range) as u8,
            );
            // Номер столбца задаёт положение ячейки по горизонтали.
            let input_value = column_index as f64;
            // Разворачиваем ось строк так, чтобы первая была сверху.
            let second_input_value = (rows - row_index - 1) as f64;
            // Рисуем цветной прямоугольник на соответствующих координатах.
            chart.draw_series(std::iter::once(plotters::prelude::Rectangle::new(
                // Задаём значения следующей строки или последовательности.
                [
                    (input_value, second_input_value),
                    (input_value + 1.0, second_input_value + 1.0),
                ],
                // Заливаем прямоугольник выбранным цветом.
                plotters::prelude::Color::filled(&color),
            )))?;
            // Пишем числовое значение в центре ячейки.
            chart.draw_series(std::iter::once(plotters::prelude::Text::new(
                // Показываем значение с двумя знаками после запятой.
                format!("{value:.2}"),
                // Добавляем пару значений для сравнения или построения графика.
                (input_value + 0.5, second_input_value + 0.5),
                // Добавляем пару значений для сравнения или построения графика.
                plotters::prelude::IntoFont::into_font(("sans-serif", 16))
                    // Белый текст читается поверх окрашенной ячейки.
                    .color(&plotters::prelude::WHITE)
                    // Выравниваем подпись по центру ячейки.
                    .pos(plotters::style::text_anchor::Pos::new(
                        plotters::style::text_anchor::HPos::Center,
                        plotters::style::text_anchor::VPos::Center,
                    )),
            )))?;
        }
    }
    // Завершаем запись SVG на диск.
    root.present()?;
    // Завершаем использование холста перед возвратом пути.
    drop(chart);
    // Завершаем использование холста перед возвратом пути.
    drop(root);
    // Возвращаем путь к сохранённому графику.
    Ok(path)
}

// Создаёт отдельный каталог для файлов, полученных при запуске урока.
fn output_path(
    lesson_directory: &str,
    name: &str,
) -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
    // Проверяем имя файла до обращения к файловой системе.
    if name.is_empty()
        // Задаём преобразование для элементов коллекции.
        || !name
            // Просматриваем символы имени по одному.
            .chars()
            // Разрешаем только буквы ASCII, цифры, дефис и подчёркивание.
            .all(|character| character.is_ascii_alphanumeric() || character == '-' || character == '_')
    {
        // Прерываем вычисление и возвращаем причину ошибки.
        return Err("недопустимое имя графика".into());
    }
    // Все графики урока хранятся в его каталоге visualizations.
    let output_directory = std::path::Path::new(lesson_directory).join("visualizations");
    // Создаём каталог при первом запуске урока.
    std::fs::create_dir_all(&output_directory)?;
    // Добавляем к проверенному имени расширение SVG.
    Ok(output_directory.join(format!("{name}.svg")))
}

// Добавляет свободное поле вокруг крайних точек графика.
fn bounds(points: &[(f64, f64)]) -> ((f64, f64), (f64, f64)) {
    // Начинаем поиск крайних координат среди точек.
    let (
        mut horizontal_minimum,
        mut horizontal_maximum,
        mut vertical_minimum,
        mut vertical_maximum,
    ) = (
        // Задаём именованное поле или параметр.
        f64::INFINITY,
        // Задаём именованное поле или параметр.
        f64::NEG_INFINITY,
        // Задаём именованное поле или параметр.
        f64::INFINITY,
        // Задаём именованное поле или параметр.
        f64::NEG_INFINITY,
    );
    // Обновляем границы обеих осей для каждой точки.
    for &(input_value, second_input_value) in points {
        // Обновляем значение результатом текущего вычисления.
        horizontal_minimum = horizontal_minimum.min(input_value);
        // Обновляем значение результатом текущего вычисления.
        horizontal_maximum = horizontal_maximum.max(input_value);
        // Обновляем значение результатом текущего вычисления.
        vertical_minimum = vertical_minimum.min(second_input_value);
        // Обновляем значение результатом текущего вычисления.
        vertical_maximum = vertical_maximum.max(second_input_value);
    }
    // Сохраняем результат этого шага в `horizontal_axis_padding`.
    let horizontal_axis_padding = (horizontal_maximum - horizontal_minimum).max(1.0) * 0.05;
    // Сохраняем результат этого шага в `vertical_axis_padding`.
    let vertical_axis_padding = (vertical_maximum - vertical_minimum).max(1.0) * 0.1;
    (
        // Добавляем пару значений для сравнения или построения графика.
        (
            horizontal_minimum - horizontal_axis_padding,
            horizontal_maximum + horizontal_axis_padding,
        ),
        // Добавляем пару значений для сравнения или построения графика.
        (
            vertical_minimum - vertical_axis_padding,
            vertical_maximum + vertical_axis_padding,
        ),
    )
}
