// Урок 012. Ищем два числа x и y, которые подходят сразу под два уравнения.
// Запишем уравнения коротко:
//   a*x + b*y = u
//   c*x + d*y = v
// Здесь a, b — row1_coef1, row1_coef2; c, d — row2_coef1, row2_coef2.
// Числа u, v — right_hand_side1, right_hand_side2.
//
// Шаг 1. Убираем y: умножаем первое уравнение на d, второе — на b.
//   a*d*x + b*d*y = u*d
//   b*c*x + b*d*y = v*b
// Вычитаем второе уравнение из первого. Одинаковые слагаемые с y сокращаются:
//   a*d*x - b*c*x + (b*d*y - b*d*y) = u*d - v*b
//   a*d*x - b*c*x + 0 = u*d - v*b
// Выносим x за скобки:
//   (a*d - b*c)*x = u*d - v*b
// Множитель a*d - b*c называют определителем (determinant).
// Если он не равен нулю, делим обе части на него:
//   x = (u*d - v*b) / (a*d - b*c)
//
// Шаг 2. Убираем x: умножаем второе уравнение на a, первое — на c.
//   a*c*x + a*d*y = a*v
//   a*c*x + b*c*y = c*u
// Из умноженного второго уравнения вычитаем умноженное первое:
//   (a*c*x - a*c*x) + a*d*y - b*c*y = a*v - c*u
//   0 + (a*d - b*c)*y = a*v - c*u
// Если определитель не равен нулю:
//   y = (a*v - c*u) / (a*d - b*c)
//
// Первый пример: 2*x + y = 5 и x - y = 1.
// Для x умножаем первое уравнение на -1, второе — на 1:
//   -2*x - y = -5
//      x - y =  1
// Вычитаем: (-2*x - y) - (x - y) = -5 - 1.
// Раскрываем скобки: -2*x - y - x + y = -6.
// Слагаемые -y + y сокращаются: -3*x = -6, значит x = 2.
// Для y умножаем второе уравнение на 2, первое — на 1:
//   2*x - 2*y = 2
//   2*x +   y = 5
// Вычитаем: (2*x - 2*y) - (2*x + y) = 2 - 5.
// Раскрываем скобки: 2*x - 2*y - 2*x - y = -3.
// Слагаемые 2*x - 2*x сокращаются: -3*y = -3, значит y = 1.
// Проверяем исходные уравнения: 2*2 + 1 = 5 и 2 - 1 = 1.
//
// Если определитель равен нулю, делить на него нельзя.
// Например, x + y = 3 и 2*x + 2*y = 6 повторяют одно условие:
// после исключения неизвестной получается 0 = 0; решений бесконечно много.
// Но x + y = 3 и 2*x + 2*y = 7 противоречат друг другу:
// вычитание удвоенного первого уравнения из второго даёт 0 = 1; решений нет.
// Уравнения с обоими нулевыми коэффициентами проверяем отдельно:
// 0*x + 0*y = 0 не ограничивает ответ, а 0*x + 0*y = 1 невозможно.

fn main() {
    let cases: [(&str, [f64; 4], [f64; 2]); 5] = [
        ("одно решение", [2.0, 1.0, 1.0, -1.0], [5.0, 1.0]),
        ("бесконечно много решений", [1.0, 1.0, 2.0, 2.0], [3.0, 6.0]),
        ("решений нет", [1.0, 1.0, 2.0, 2.0], [3.0, 7.0]),
        ("бесконечно много решений", [0.0, 0.0, 0.0, 0.0], [0.0, 0.0]),
        ("решений нет", [0.0, 0.0, 0.0, 0.0], [1.0, 0.0]),
    ];
    for (
        description,
        [row1_coef1, row1_coef2, row2_coef1, row2_coef2],
        [right_hand_side1, right_hand_side2],
    ) in cases
    {
        // Умножаем ВСЁ первое уравнение на коэффициент при y из второго.
        // В первом примере: 2*x + y = 5 превращается в -2*x - y = -5.
        let row1_multiplier: f64 = row2_coef2;
        let row1_x_coef: f64 = row1_coef1 * row1_multiplier;
        let row1_y_coef: f64 = row1_coef2 * row1_multiplier;
        let row1_right: f64 = right_hand_side1 * row1_multiplier;

        // Умножаем ВСЁ второе уравнение на коэффициент при y из первого.
        // В первом примере множитель равен 1: остаётся x - y = 1.
        let row2_multiplier: f64 = row1_coef2;
        let row2_x_coef: f64 = row2_coef1 * row2_multiplier;
        let row2_y_coef: f64 = row2_coef2 * row2_multiplier;
        let row2_right: f64 = right_hand_side2 * row2_multiplier;

        // Вычитаем второе полученное уравнение из первого, часть за частью.
        // (-2 - 1)*x + (-1 - (-1))*y = -5 - 1.
        let x_coef_after_subtraction: f64 = row1_x_coef - row2_x_coef;
        let y_coef_after_subtraction: f64 = row1_y_coef - row2_y_coef;
        let right_after_eliminating_y: f64 = row1_right - row2_right;
        assert_eq!(y_coef_after_subtraction, 0.0);

        // Получилось -3*x + 0*y = -6, то есть -3*x = -6.
        // Именно оставшийся коэффициент при x называют определителем.
        let determinant: f64 = x_coef_after_subtraction;

        // Аналогично убираем x, чтобы найти y.
        // Умножаем второе исходное уравнение на коэффициент при x из первого.
        // В первом примере: 2*x - 2*y = 2.
        let row2_multiplier: f64 = row1_coef1;
        let row2_x_coef: f64 = row2_coef1 * row2_multiplier;
        let row2_y_coef: f64 = row2_coef2 * row2_multiplier;
        let row2_right: f64 = right_hand_side2 * row2_multiplier;

        // Умножаем первое исходное уравнение на коэффициент при x из второго.
        // В первом примере: 2*x + y = 5.
        let row1_multiplier: f64 = row2_coef1;
        let row1_x_coef: f64 = row1_coef1 * row1_multiplier;
        let row1_y_coef: f64 = row1_coef2 * row1_multiplier;
        let row1_right: f64 = right_hand_side1 * row1_multiplier;

        // Теперь вычитаем умноженное первое уравнение из умноженного второго.
        // (2 - 2)*x + (-2 - 1)*y = 2 - 5, то есть -3*y = -3.
        let x_coef_after_subtraction: f64 = row2_x_coef - row1_x_coef;
        let y_coef_after_subtraction: f64 = row2_y_coef - row1_y_coef;
        let right_after_eliminating_x: f64 = row2_right - row1_right;
        assert_eq!(x_coef_after_subtraction, 0.0);
        assert_eq!(y_coef_after_subtraction, determinant);

        if determinant != 0.0 {
            // Делим правую часть на оставшийся коэффициент: x = -6 / -3 = 2.
            let x: f64 = right_after_eliminating_y / determinant;
            // y = -3 / -3 = 1.
            let y: f64 = right_after_eliminating_x / y_coef_after_subtraction;

            // Подставляем ответы в оба исходных уравнения.
            assert_eq!(row1_coef1 * x + row1_coef2 * y, right_hand_side1);
            assert_eq!(row2_coef1 * x + row2_coef2 * y, right_hand_side2);
        } else {
            // После вычитания слева ноль. Справа тоже должен быть ноль,
            // иначе уравнения противоречат друг другу.
            let impossible_zero_row: bool =
                (row1_coef1 == 0.0 && row1_coef2 == 0.0 && right_hand_side1 != 0.0)
                    || (row2_coef1 == 0.0 && row2_coef2 == 0.0 && right_hand_side2 != 0.0);
            let actual: &str = if right_after_eliminating_y == 0.0
                && right_after_eliminating_x == 0.0
                && !impossible_zero_row
            {
                "бесконечно много решений"
            } else {
                "решений нет"
            };
            assert_eq!(actual, description);
        }
    }
}
