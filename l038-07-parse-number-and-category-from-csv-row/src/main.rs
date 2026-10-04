// Урок 038. Разделять строку CSV на числовое поле и текстовую категорию.
// Проверяем наличие разделителя и возможность преобразовать первое поле в число.

fn main() {
    let row: &str = "3.5,red";
    let (number_text, category): (&str, &str) = row
        .split_once(',')
        .expect("в строке нет запятой между числом и категорией");
    let _: f64 = number_text
        .parse()
        .expect("первая колонка должна содержать число");
    let _: &str = category;

    for row in ["3.5,red", "oops,blue", "нет запятой"] {
        let parsed = row
            .split_once(',')
            .ok_or("нет разделителя")
            .and_then(|(number, category)| {
                number
                    .parse::<f64>()
                    .map(|value| (value, category))
                    .map_err(|_| "не число")
            });
        println!("Строка {row:?}: {parsed:?}");
        assert_eq!(parsed.is_ok(), row == "3.5,red");
    }
}

// Чему учит этот урок:
// Учимся разделять строку CSV на числовое поле и текстовую категорию.
// Проверяем наличие разделителя и возможность преобразовать первое поле в число.
