//! Общая проверка приблизительного равенства дробных чисел для уроков.

/// Числа приблизительно равны, если модуль их разницы строго меньше допуска.
/// Например, `1e-10` означает допустимую разницу меньше `0.0000000001`.
/// Это абсолютный допуск: он не зависит от величины сравниваемых чисел.
/// Передавайте положительный конечный допуск. На самой границе возвращается `false`.
/// `NaN` и бесконечности не считаются приблизительно равными.
pub fn compare_2_floats_for_approximate_equality(
    first: f64,
    second: f64,
    absolute_tolerance: f64,
) -> bool {
    (first - second).abs() < absolute_tolerance
}

/// Проверяет приблизительное равенство с абсолютной разницей строго меньше `1e-2`.
pub fn check_f64_eq_1e_minus_2(first: f64, second: f64) -> bool {
    compare_2_floats_for_approximate_equality(first, second, 1e-2)
}

/// Проверяет приблизительное равенство с абсолютной разницей строго меньше `1e-6`.
pub fn check_f64_eq_1e_minus_6(first: f64, second: f64) -> bool {
    compare_2_floats_for_approximate_equality(first, second, 1e-6)
}

/// Проверяет приблизительное равенство с абсолютной разницей строго меньше `1e-7`.
pub fn check_f64_eq_1e_minus_7(first: f64, second: f64) -> bool {
    compare_2_floats_for_approximate_equality(first, second, 1e-7)
}

/// Проверяет приблизительное равенство с абсолютной разницей строго меньше `1e-8`.
pub fn check_f64_eq_1e_minus_8(first: f64, second: f64) -> bool {
    compare_2_floats_for_approximate_equality(first, second, 1e-8)
}

/// Проверяет приблизительное равенство с абсолютной разницей строго меньше `1e-9`.
pub fn check_f64_eq_1e_minus_9(first: f64, second: f64) -> bool {
    compare_2_floats_for_approximate_equality(first, second, 1e-9)
}

/// Проверяет приблизительное равенство с абсолютной разницей строго меньше `1e-10`.
pub fn check_f64_eq_1e_minus_10(first: f64, second: f64) -> bool {
    compare_2_floats_for_approximate_equality(first, second, 1e-10)
}

/// Проверяет приблизительное равенство с абсолютной разницей строго меньше `1e-12`.
pub fn check_f64_eq_1e_minus_12(first: f64, second: f64) -> bool {
    compare_2_floats_for_approximate_equality(first, second, 1e-12)
}

#[cfg(test)]
mod tests {
    use super::compare_2_floats_for_approximate_equality;

    #[test]
    fn tolerates_rounding_error_in_either_direction() {
        assert!(compare_2_floats_for_approximate_equality(
            0.1 + 0.2,
            0.3,
            1e-10
        ));
        assert!(compare_2_floats_for_approximate_equality(
            0.3,
            0.1 + 0.2,
            1e-10
        ));
        assert!(compare_2_floats_for_approximate_equality(-2.0, -2.0, 1e-12));
    }

    #[test]
    fn keeps_the_strict_absolute_tolerance_boundary() {
        // Эти дроби точно представимы в f64: тест не зависит от округления.
        assert!(compare_2_floats_for_approximate_equality(1.0, 1.125, 0.25));
        assert!(!compare_2_floats_for_approximate_equality(1.0, 1.25, 0.25));
        assert!(!compare_2_floats_for_approximate_equality(1.0, 1.5, 0.25));
        assert!(!compare_2_floats_for_approximate_equality(
            1_000_000.0,
            1_000_000.5,
            0.25
        ));
        assert!(compare_2_floats_for_approximate_equality(0.0, -0.125, 0.25));
    }

    #[test]
    fn rejects_nonfinite_values_and_nonpositive_tolerances() {
        for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            assert!(!compare_2_floats_for_approximate_equality(
                value, value, 1e-10
            ));
            assert!(!compare_2_floats_for_approximate_equality(
                value, 0.0, 1e-10
            ));
            assert!(!compare_2_floats_for_approximate_equality(
                0.0, value, 1e-10
            ));
        }
        for tolerance in [0.0, -1.0, f64::NAN] {
            assert!(!compare_2_floats_for_approximate_equality(
                1.0, 1.0, tolerance
            ));
        }
    }
}
