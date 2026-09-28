//! SVG-графики для учебных примеров.

use plotters::prelude::*;
use plotters::style::text_anchor::{HPos, Pos, VPos};
use std::error::Error;
use std::path::{Path, PathBuf};

pub struct Series<'a> {
    pub name: &'a str,
    pub points: &'a [(f64, f64)],
}

pub fn line_chart(
    lesson_dir: &str,
    name: &str,
    title: &str,
    x_label: &str,
    y_label: &str,
    series: &[Series<'_>],
) -> Result<PathBuf, Box<dyn Error>> {
    draw_chart(lesson_dir, name, title, x_label, y_label, series, true)
}

pub fn scatter_chart(
    lesson_dir: &str,
    name: &str,
    title: &str,
    x_label: &str,
    y_label: &str,
    series: &[Series<'_>],
) -> Result<PathBuf, Box<dyn Error>> {
    draw_chart(lesson_dir, name, title, x_label, y_label, series, false)
}

fn draw_chart(
    lesson_dir: &str,
    name: &str,
    title: &str,
    x_label: &str,
    y_label: &str,
    series: &[Series<'_>],
    connect_points: bool,
) -> Result<PathBuf, Box<dyn Error>> {
    let path = output_path(lesson_dir, name)?;
    let points: Vec<(f64, f64)> = series
        .iter()
        .flat_map(|s| s.points.iter().copied())
        .filter(|(x, y)| x.is_finite() && y.is_finite())
        .collect();
    if points.is_empty() {
        return Err("нет конечных точек для графика".into());
    }
    let ((x_min, x_max), (y_min, y_max)) = bounds(&points);
    let root = SVGBackend::new(&path, (900, 560)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption(title, ("sans-serif", 24))
        .margin(20)
        .x_label_area_size(48)
        .y_label_area_size(65)
        .build_cartesian_2d(x_min..x_max, y_min..y_max)?;
    chart
        .configure_mesh()
        .x_desc(x_label)
        .y_desc(y_label)
        .draw()?;
    for (index, item) in series.iter().enumerate() {
        let color = Palette99::pick(index);
        if connect_points {
            chart
                .draw_series(LineSeries::new(
                    item.points
                        .iter()
                        .copied()
                        .filter(|(x, y)| x.is_finite() && y.is_finite()),
                    &color,
                ))?
                .label(item.name)
                .legend(move |(x, y)| {
                    PathElement::new(vec![(x, y), (x + 20, y)], Palette99::pick(index))
                });
        }
        let dots = chart.draw_series(
            item.points
                .iter()
                .copied()
                .filter(|(x, y)| x.is_finite() && y.is_finite())
                .map(|point| Circle::new(point, 4, color.filled())),
        )?;
        if !connect_points {
            dots.label(item.name)
                .legend(move |(x, y)| Circle::new((x + 10, y), 4, Palette99::pick(index).filled()));
        }
    }
    if series.len() > 1 {
        chart.configure_series_labels().border_style(BLACK).draw()?;
    }
    root.present()?;
    drop(chart);
    drop(root);
    Ok(path)
}

pub fn bars(
    lesson_dir: &str,
    name: &str,
    title: &str,
    y_label: &str,
    values: &[(&str, f64)],
) -> Result<PathBuf, Box<dyn Error>> {
    let path = output_path(lesson_dir, name)?;
    if values.is_empty() || values.iter().any(|(_, v)| !v.is_finite()) {
        return Err("для столбцов нужны конечные значения".into());
    }
    let min = values.iter().map(|(_, v)| *v).fold(0.0_f64, f64::min);
    let max = values.iter().map(|(_, v)| *v).fold(0.0_f64, f64::max);
    let pad = (max - min).max(1.0) * 0.1;
    let root = SVGBackend::new(&path, (900, 560)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption(title, ("sans-serif", 24))
        .margin(20)
        .x_label_area_size(90)
        .y_label_area_size(65)
        .build_cartesian_2d(-0.5..values.len() as f64 - 0.5, min - pad..max + pad)?;
    chart
        .configure_mesh()
        .disable_x_mesh()
        .x_labels(values.len())
        .x_label_formatter(&|x| {
            let index = x.round();
            if (x - index).abs() > 0.15 || index < 0.0 {
                return String::new();
            }
            values
                .get(index as usize)
                .map(|(label, _)| (*label).to_string())
                .unwrap_or_default()
        })
        .y_desc(y_label)
        .draw()?;
    for (i, (_, value)) in values.iter().enumerate() {
        chart.draw_series(std::iter::once(Rectangle::new(
            [(i as f64 - 0.35, 0.0), (i as f64 + 0.35, *value)],
            Palette99::pick(i).filled(),
        )))?;
    }
    root.present()?;
    drop(chart);
    drop(root);
    Ok(path)
}

/// Показывает значения матрицы цветом: светлая ячейка — меньшее значение.
pub fn heatmap(
    lesson_dir: &str,
    name: &str,
    title: &str,
    values: &[Vec<f64>],
) -> Result<PathBuf, Box<dyn Error>> {
    let path = output_path(lesson_dir, name)?;
    let rows = values.len();
    let columns = values.first().map_or(0, Vec::len);
    if rows == 0
        || columns == 0
        || values
            .iter()
            .any(|row| row.len() != columns || row.iter().any(|v| !v.is_finite()))
    {
        return Err("матрица должна быть прямоугольной и содержать конечные значения".into());
    }
    let min = values
        .iter()
        .flatten()
        .copied()
        .fold(f64::INFINITY, f64::min);
    let max = values
        .iter()
        .flatten()
        .copied()
        .fold(f64::NEG_INFINITY, f64::max);
    let root = SVGBackend::new(&path, (700, 620)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption(title, ("sans-serif", 24))
        .margin(30)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0.0..columns as f64, 0.0..rows as f64)?;
    chart
        .configure_mesh()
        .disable_mesh()
        .x_desc("Столбец")
        .y_desc("Строка")
        .draw()?;
    for (row_index, row) in values.iter().enumerate() {
        for (column_index, &value) in row.iter().enumerate() {
            let t = if max == min {
                0.5
            } else {
                (value - min) / (max - min)
            };
            let color = RGBColor(
                (35.0 + 180.0 * t) as u8,
                (80.0 + 90.0 * (1.0 - t)) as u8,
                (220.0 - 160.0 * t) as u8,
            );
            let x = column_index as f64;
            let y = (rows - row_index - 1) as f64;
            chart.draw_series(std::iter::once(Rectangle::new(
                [(x, y), (x + 1.0, y + 1.0)],
                color.filled(),
            )))?;
            chart.draw_series(std::iter::once(Text::new(
                format!("{value:.2}"),
                (x + 0.5, y + 0.5),
                ("sans-serif", 16)
                    .into_font()
                    .color(&WHITE)
                    .pos(Pos::new(HPos::Center, VPos::Center)),
            )))?;
        }
    }
    root.present()?;
    drop(chart);
    drop(root);
    Ok(path)
}

fn output_path(lesson_dir: &str, name: &str) -> Result<PathBuf, Box<dyn Error>> {
    if name.is_empty()
        || !name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
    {
        return Err("недопустимое имя графика".into());
    }
    let dir = Path::new(lesson_dir).join("visualizations");
    std::fs::create_dir_all(&dir)?;
    Ok(dir.join(format!("{name}.svg")))
}

fn bounds(points: &[(f64, f64)]) -> ((f64, f64), (f64, f64)) {
    let (mut x_min, mut x_max, mut y_min, mut y_max) = (
        f64::INFINITY,
        f64::NEG_INFINITY,
        f64::INFINITY,
        f64::NEG_INFINITY,
    );
    for &(x, y) in points {
        x_min = x_min.min(x);
        x_max = x_max.max(x);
        y_min = y_min.min(y);
        y_max = y_max.max(y);
    }
    let x_pad = (x_max - x_min).max(1.0) * 0.05;
    let y_pad = (y_max - y_min).max(1.0) * 0.1;
    (
        (x_min - x_pad, x_max + x_pad),
        (y_min - y_pad, y_max + y_pad),
    )
}
