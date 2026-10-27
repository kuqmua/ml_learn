//! Учебная арифметика binary32 через целые числа. Методы Float32 не используют встроенные f32/f64.
//! Режим округления: к ближайшему, при равенстве расстояний — к чётному (ties to even).
//! Все операции с NaN возвращают один quiet NaN с битом знака 0 (PositiveNan); флаги исключений не моделируются.
fn main() -> Result<(), &'static str> {
    // Примеры выполняются без вывода; значения можно посмотреть в отладчике.
    // Знак, 8 бит порядка и 23 бита дробной части заданы отдельно.
    // Для Normal: число = множитель знака (+1 или −1) × (1 + дробная часть) × 2^(порядок − 127).
    // Бит знака 0 означает плюс, 1 — минус. Порядок читаем как двоичное целое.
    // Биты дробной части имеют веса 1/2, 1/4, 1/8, ...; ведущая 1 хранится неявно.
    // Формула выше не применяется к PositiveZero, NegativeZero, Subnormal, PositiveInfinity, NegativeInfinity, PositiveNan и NegativeNan.
    // Разбираем число −3.25 по каждому биту. Индексы массивов начинаются с 0.
    // Бит 1 включает вес своей позиции, бит 0 не добавляет ничего.
    let _number = Float32::try_from((
        // sign — знак всего числа: 0 означает плюс, 1 означает минус.
        1,
        // exponent_bits — восемь битов, из которых складываем записанный порядок.
        // Это ещё не степень двойки: для Normal из полученной суммы вычитаем 127.
        [
            1, // exponent_bits[0]: вес 128; вклад 1 × 128 = 128.
            0, // exponent_bits[1]: вес 64; вклад 0 × 64 = 0.
            0, // exponent_bits[2]: вес 32; вклад 0 × 32 = 0.
            0, // exponent_bits[3]: вес 16; вклад 0 × 16 = 0.
            0, // exponent_bits[4]: вес 8; вклад 0 × 8 = 0.
            0, // exponent_bits[5]: вес 4; вклад 0 × 4 = 0.
            0, // exponent_bits[6]: вес 2; вклад 0 × 2 = 0.
            0, // exponent_bits[7]: вес 1; вклад 0 × 1 = 0.
        ],
        // Сумма = 128. Степень = 128 − 127 = 1. Значит, дальше умножаем на 2.
        // fraction_bits — 23 бита после точки в двоичной системе.
        // Каждый следующий вес вдвое меньше предыдущего. Все значения ниже — десятичные.
        [
            1, // fraction_bits[0]: вес 1/2 = 0.5; прибавляем 0.5.
            0, // fraction_bits[1]: вес 1/4 = 0.25; прибавляем 0.
            1, // fraction_bits[2]: вес 1/8 = 0.125; прибавляем 0.125.
            0, // fraction_bits[3]: вес 1/16 = 0.0625; прибавляем 0.
            0, // fraction_bits[4]: вес 1/32 = 0.03125; прибавляем 0.
            0, // fraction_bits[5]: вес 1/64 = 0.015625; прибавляем 0.
            0, // fraction_bits[6]: вес 1/128 = 0.0078125; прибавляем 0.
            0, // fraction_bits[7]: вес 1/256 = 0.00390625; прибавляем 0.
            0, // fraction_bits[8]: вес 1/512 = 0.001953125; прибавляем 0.
            0, // fraction_bits[9]: вес 1/1024 = 0.0009765625; прибавляем 0.
            0, // fraction_bits[10]: вес 1/2048 = 0.00048828125; прибавляем 0.
            0, // fraction_bits[11]: вес 1/4096 = 0.000244140625; прибавляем 0.
            0, // fraction_bits[12]: вес 1/8192 = 0.0001220703125; прибавляем 0.
            0, // fraction_bits[13]: вес 1/16384 = 0.00006103515625; прибавляем 0.
            0, // fraction_bits[14]: вес 1/32768 = 0.000030517578125; прибавляем 0.
            0, // fraction_bits[15]: вес 1/65536 = 0.0000152587890625; прибавляем 0.
            0, // fraction_bits[16]: вес 1/131072 = 0.00000762939453125; прибавляем 0.
            0, // fraction_bits[17]: вес 1/262144 = 0.000003814697265625; прибавляем 0.
            0, // fraction_bits[18]: вес 1/524288 = 0.0000019073486328125; прибавляем 0.
            0, // fraction_bits[19]: вес 1/1048576 = 0.00000095367431640625; прибавляем 0.
            0, // fraction_bits[20]: вес 1/2097152 = 0.000000476837158203125; прибавляем 0.
            0, // fraction_bits[21]: вес 1/4194304 = 0.0000002384185791015625; прибавляем 0.
            0, // fraction_bits[22]: вес 1/8388608 = 0.00000011920928955078125; прибавляем 0.
        ],
        // Сумма включённых весов дроби: 0.5 + 0.125 = 0.625.
        // Для Normal перед дробью подразумевается 1: получаем 1 + 0.625 = 1.625.
        // Эта дополнительная 1 не занимает место в массиве из 23 битов.
        // Применяем множитель из порядка и знак: 1.625 × 2 × (−1) = −3.25.
        // При порядке 0 или 255 действуют другие правила; они описаны в README.
    ))?;
    // 1.5 = +(1 + 1/2) × 2^0; класс Normal.
    let one_and_half = Float32::try_from((
        // 0 — положительное число.
        0,
        // 01111111 в двоичной системе — это 127 в десятичной системе; степень: 127 − 127 = 0.
        [0, 1, 1, 1, 1, 1, 1, 1],
        // Дробь 0.1 в двоичной системе = 0.5; вместе с неявной 1 получаем 1.5.
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    ))?;
    // 2.25 = +(1 + 1/8) × 2^1; класс Normal.
    let two_and_quarter = Float32::try_from((
        // 0 — положительное число.
        0,
        // 10000000 в двоичной системе — это 128 в десятичной системе; степень: 128 − 127 = 1.
        [1, 0, 0, 0, 0, 0, 0, 0],
        // Дробь 0.001 в двоичной системе = 1/8 = 0.125; 1.125 × 2 = 2.25.
        [0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    ))?;
    // 1.5 + 2.25 = 3.75, точно; класс Normal.
    let _sum = one_and_half.calc_sum_by_aligning_exponents(two_and_quarter);
    // 1.5 − 2.25 = −0.75, точно; класс Normal.
    let _difference = one_and_half.calc_difference_by_negating_and_adding(two_and_quarter);
    // 1.5 × 2.25 = 3.375, точно; класс Normal.
    let _product = one_and_half.calc_product_by_multiplying_significands(two_and_quarter);
    // 1.5 / 2.25 = 2/3; после округления binary32: 0.666666686534881591796875, класс Normal.
    let _quotient = one_and_half.calc_quotient_by_dividing_significands(two_and_quarter);
    // 1.0 = +(1 + 0) × 2^0; класс Normal.
    let one = Float32::try_from((
        // 0 — положительное число.
        0,
        // 01111111 в двоичной системе — это 127 в десятичной системе; степень: 127 − 127 = 0.
        [0, 1, 1, 1, 1, 1, 1, 1],
        // Все нули: дробь равна 0, но неявная ведущая 1 остаётся.
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    ))?;
    // 2^−24 = 0.000000059604644775390625; класс Normal.
    let half_step = Float32::try_from((
        // 0 — положительное число.
        0,
        // 01100111 в двоичной системе — это 103 в десятичной системе; степень: 103 − 127 = −24.
        [0, 1, 1, 0, 0, 1, 1, 1],
        // Дробь 0; значение равно 1 × 2^−24 — половине шага между 1.0 и следующим большим binary32.
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    ))?;
    // 1 + 2^−24 находится ровно посередине между соседними binary32; округляется к 1.0 (чётный младший бит).
    let _rounded = one.calc_sum_by_aligning_exponents(half_step);
    // 2^−126 ≈ 1.17549435 × 10^−38 — наименьшее положительное Normal.
    let smallest_normal = Float32::try_from((
        // 0 — положительное число.
        0,
        // 00000001 в двоичной системе — это 1 в десятичной системе; степень: 1 − 127 = −126.
        [0, 0, 0, 0, 0, 0, 0, 1],
        // Дробь 0; неявная 1 даёт значение 1 × 2^−126.
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    ))?;
    // 2.0 = +(1 + 0) × 2^1; класс Normal.
    let two = Float32::try_from((
        // 0 — положительное число.
        0,
        // 10000000 в двоичной системе — это 128 в десятичной системе; степень: 128 − 127 = 1.
        [1, 0, 0, 0, 0, 0, 0, 0],
        // Дробь 0; значение равно 1 × 2 = 2.0.
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    ))?;
    // 2^−126 / 2 = 2^−127 ≈ 5.87747175 × 10^−39; класс Subnormal: порядок 0, дробь 0.1 в двоичной системе, без неявной 1.
    let _subnormal = smallest_normal.calc_quotient_by_dividing_significands(two);
    // 0 × бесконечность = NaN: не число, класс PositiveNan (бит знака 0). Это допустимая битовая запись, не ошибка конструктора.
    let _invalid = Float32::POSITIVE_ZERO.calc_product_by_multiplying_significands(Float32::POSITIVE_INFINITY);
    // −0.0: знак 1, порядок и дробь нулевые; класс NegativeZero. У +0.0 отдельный класс PositiveZero.
    let _negative_zero = Float32::POSITIVE_ZERO.negate_by_flipping_sign_bit();
    // Категория числа 1.5 — FloatClass::Normal; здесь хранится категория, а не число.
    let _class = one_and_half.classify();
    // Err("бит должен быть 0 или 1"): знак 2 недопустим; Float32 не создаётся.
    let _invalid_bits = Float32::try_from((2, [0; 8], [0; 23]));
    // 0x3f800000 = биты 0 | 01111111 | 00000000000000000000000 → число 1.0, класс Normal.
    let _one_from_encoded_bits = Float32::from(0x3f80_0000_u32);
    // NaN с битом знака 1; это не отрицательное число, а вариант битовой записи NaN.
    let _nan_with_sign_bit_one = Float32::NEGATIVE_NAN;
    Ok(())
}
const FRACTION_MASK: u32 = 0x007f_ffff;
const HIDDEN_BIT: u64 = 1 << 23;
/// 32 отдельных бита, каждый хранится как u8 со значением 0 или 1.
/// Биты занимают 32 байта; class хранится дополнительно. Это не компактный f32.
/// PartialEq намеренно не выводим: совпадение битов отличается от равенства чисел
/// (+0 и -0 равны численно, а NaN не равен даже самому себе).
#[derive(Clone, Copy, Debug)]
struct Float32 {
    sign: u8,
    exponent_bits: [u8; 8],
    fraction_bits: [u8; 23],
    // Категория числа; TryFrom и From определяют её по битам автоматически.
    class: FloatClass,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum FloatClass {
    PositiveZero, // +0.0: знак 0, все биты порядка и дроби равны 0.
    NegativeZero, // −0.0: знак 1, все биты порядка и дроби равны 0.
    Subnormal,
    Normal,
    PositiveInfinity, // +бесконечность: знак 0, порядок из единиц, дробь из нулей.
    NegativeInfinity, // −бесконечность: знак 1, порядок из единиц, дробь из нулей.
    PositiveNan, // NaN с битом знака 0; это не положительное число.
    NegativeNan, // NaN с битом знака 1; это не отрицательное число.
}
impl FloatClass {
    // Оба значения бита знака означают NaN и одинаково обрабатываются арифметикой.
    fn is_either_nan(self) -> bool {
        matches!(self, Self::PositiveNan | Self::NegativeNan)
    }
    // Проверяем обе бесконечности; их знак учитываем отдельно в самой операции.
    fn is_either_infinity(self) -> bool {
        matches!(self, Self::PositiveInfinity | Self::NegativeInfinity)
    }
    // В арифметике иногда нужно проверить оба нуля, независимо от их знака.
    fn is_either_zero(self) -> bool {
        matches!(self, Self::PositiveZero | Self::NegativeZero)
    }
}
// В отдельных u8 можно передать 2 или 255 вместо бита, поэтому нужен TryFrom.
impl TryFrom<(u8, [u8; 8], [u8; 23])> for Float32 {
    type Error = &'static str;
    fn try_from((sign, exponent_bits, fraction_bits): (u8, [u8; 8], [u8; 23])) -> Result<Self, Self::Error> {
        if sign > 1 || exponent_bits.iter().chain(fraction_bits.iter()).any(|bit| *bit > 1) {
            return Err("бит должен быть 0 или 1");
        }
        let exponent_is_zero = exponent_bits == [0; 8];
        let exponent_is_all_ones = exponent_bits == [1; 8];
        let fraction_is_zero = fraction_bits == [0; 23];
        let class = match (exponent_is_zero, exponent_is_all_ones, fraction_is_zero) {
            (true, _, true) => if sign == 0 { FloatClass::PositiveZero } else { FloatClass::NegativeZero },
            (true, _, false) => FloatClass::Subnormal,
            (_, true, true) => if sign == 0 { FloatClass::PositiveInfinity } else { FloatClass::NegativeInfinity },
            (_, true, false) => if sign == 0 { FloatClass::PositiveNan } else { FloatClass::NegativeNan },
            _ => FloatClass::Normal,
        };
        Ok(Self {
            sign,
            exponent_bits,
            fraction_bits,
            class,
        })
    }
}
// Любая последовательность 32 бит допустима, включая NaN: здесь ошибки быть не может.
// Это чтение БИТОВОЙ ЗАПИСИ, а не преобразование целого значения в вещественное!
// Например, 0x3f80_0000 кодирует 1.0, а целое 1 кодирует самое маленькое subnormal.
impl From<u32> for Float32 {
    fn from(encoded_bits: u32) -> Self {
        let sign = (encoded_bits >> 31) as u8;
        let exponent_bits = std::array::from_fn(|index| ((encoded_bits >> (30 - index)) & 1) as u8);
        let fraction_bits = std::array::from_fn(|index| ((encoded_bits >> (22 - index)) & 1) as u8);
        Self::try_from((sign, exponent_bits, fraction_bits)).expect("извлечение отдельных битов всегда даёт только 0 или 1")
    }
}
impl Float32 {
    const POSITIVE_ZERO: Self = Self {
        class: FloatClass::PositiveZero,
        sign: 0,
        exponent_bits: [0, 0, 0, 0, 0, 0, 0, 0],
        fraction_bits: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    };
    const NEGATIVE_ZERO: Self = Self {
        class: FloatClass::NegativeZero,
        sign: 1,
        exponent_bits: [0, 0, 0, 0, 0, 0, 0, 0],
        fraction_bits: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    };
    const POSITIVE_INFINITY: Self = Self {
        class: FloatClass::PositiveInfinity,
        sign: 0,
        exponent_bits: [1, 1, 1, 1, 1, 1, 1, 1],
        fraction_bits: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    };
    const NEGATIVE_INFINITY: Self = Self {
        sign: 1,
        class: FloatClass::NegativeInfinity,
        ..Self::POSITIVE_INFINITY
    };
    const POSITIVE_NAN: Self = Self {
        class: FloatClass::PositiveNan,
        sign: 0,
        exponent_bits: [1, 1, 1, 1, 1, 1, 1, 1],
        fraction_bits: [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    };
    const NEGATIVE_NAN: Self = Self {
        sign: 1,
        class: FloatClass::NegativeNan,
        ..Self::POSITIVE_NAN
    };
    /// Собираем 8 бит порядка со смещением 127, ещё не показатель степени.
    fn exponent_as_u32(self) -> u32 {
        self.exponent_bits.iter().fold(0, |value, bit| (value << 1) | u32::from(*bit))
    }
    /// Собираем 23 бита дробной части; скрытая единица сюда не входит.
    fn fraction_as_u32(self) -> u32 {
        self.fraction_bits.iter().fold(0, |value, bit| (value << 1) | u32::from(*bit))
    }
    // Категория уже вычислена при создании; здесь просто читаем поле.
    fn classify(self) -> FloatClass {
        self.class
    }
    const fn negate_by_flipping_sign_bit(self) -> Self {
        // У нулей, бесконечностей и NaN вместе с битом знака меняется и сохранённое название класса.
        let class = match self.class {
            FloatClass::PositiveZero => FloatClass::NegativeZero,
            FloatClass::NegativeZero => FloatClass::PositiveZero,
            FloatClass::PositiveInfinity => FloatClass::NegativeInfinity,
            FloatClass::NegativeInfinity => FloatClass::PositiveInfinity,
            FloatClass::PositiveNan => FloatClass::NegativeNan,
            FloatClass::NegativeNan => FloatClass::PositiveNan,
            other_class => other_class,
        };
        Self { sign: self.sign ^ 1, class, ..self }
    }
    /// Выравниваем порядки, складываем значащие части, нормализуем и округляем.
    fn calc_sum_by_aligning_exponents(self, other: Self) -> Self {
        let left_class = self.classify();
        let right_class = other.classify();
        if left_class.is_either_nan() || right_class.is_either_nan() {
            return Self::POSITIVE_NAN;
        }
        if left_class.is_either_infinity() || right_class.is_either_infinity() {
            if left_class.is_either_infinity() && right_class.is_either_infinity() && self.sign != other.sign {
                return Self::POSITIVE_NAN; // +∞ + -∞ не определено.
            }
            return if left_class.is_either_infinity() { self } else { other };
        }
        if left_class.is_either_zero() && right_class.is_either_zero() {
            // Только -0 + -0 даёт -0 при выбранном режиме округления.
            return Self::signed_zero((self.sign == 1) && (other.sign == 1));
        }
        if left_class.is_either_zero() {
            return other;
        }
        if right_class.is_either_zero() {
            return self;
        }
        // Сначала ставим больший по модулю операнд: при вычитании результат неотрицателен.
        let (larger, smaller) = if (self.exponent_bits, self.fraction_bits) >= (other.exponent_bits, other.fraction_bits) {
            (self, other)
        } else {
            (other, self)
        };
        let (larger_significand, larger_exponent) = larger.normalized_significand_and_exponent();
        let (smaller_significand, smaller_exponent) = smaller.normalized_significand_and_exponent();
        // Три младших бита оставляем для округления: guard, round, sticky.
        // Sticky сообщает, были ли единицы среди отброшенных разрядов.
        let aligned_smaller = shift_right_with_sticky_bit(smaller_significand << 3, (larger_exponent - smaller_exponent) as u32);
        let significand = if larger.sign == smaller.sign {
            (larger_significand << 3) + aligned_smaller
        } else {
            (larger_significand << 3) - aligned_smaller
        };
        if significand == 0 {
            return Self::POSITIVE_ZERO; // Точная компенсация x + (-x).
        }
        Self::round_and_pack(larger.sign == 1, significand, larger_exponent - 23 - 3)
    }
    fn calc_difference_by_negating_and_adding(self, other: Self) -> Self {
        self.calc_sum_by_aligning_exponents(other.negate_by_flipping_sign_bit())
    }
    fn calc_product_by_multiplying_significands(self, other: Self) -> Self {
        let left_class = self.classify();
        let right_class = other.classify();
        let negative = (self.sign == 1) ^ (other.sign == 1);
        if left_class.is_either_nan() || right_class.is_either_nan() {
            return Self::POSITIVE_NAN;
        }
        if left_class.is_either_infinity() || right_class.is_either_infinity() {
            if left_class.is_either_zero() || right_class.is_either_zero() {
                return Self::POSITIVE_NAN; // 0 * ∞.
            }
            return Self::signed_infinity(negative);
        }
        if left_class.is_either_zero() || right_class.is_either_zero() {
            return Self::signed_zero(negative);
        }
        let (left_significand, left_exponent) = self.normalized_significand_and_exponent();
        let (right_significand, right_exponent) = other.normalized_significand_and_exponent();
        // 24 * 24 бита помещаются в u64 без потери точности.
        // (a * 2^(ea-23)) * (b * 2^(eb-23)) = (a*b) * 2^(ea+eb-46).
        Self::round_and_pack(negative, left_significand * right_significand, left_exponent + right_exponent - 46)
    }
    fn calc_quotient_by_dividing_significands(self, other: Self) -> Self {
        let left_class = self.classify();
        let right_class = other.classify();
        let negative = (self.sign == 1) ^ (other.sign == 1);
        if left_class.is_either_nan()
            || right_class.is_either_nan()
            || (left_class.is_either_zero() && right_class.is_either_zero())
            || (left_class.is_either_infinity() && right_class.is_either_infinity())
        {
            return Self::POSITIVE_NAN;
        }
        if left_class.is_either_infinity() || right_class.is_either_zero() {
            return Self::signed_infinity(negative);
        }
        if left_class.is_either_zero() || right_class.is_either_infinity() {
            return Self::signed_zero(negative);
        }
        let (numerator_significand, numerator_exponent) = self.normalized_significand_and_exponent();
        let (denominator_significand, denominator_exponent) = other.normalized_significand_and_exponent();
        // Делим целые числа с запасом дробных разрядов. Числитель занимает максимум 56 бит.
        let scaled_numerator = numerator_significand << 32;
        let quotient = scaled_numerator / denominator_significand;
        let remainder = scaled_numerator % denominator_significand;
        // Частное имеет 32–33 значащих бита; младший sticky заведомо попадёт в округление.
        let quotient_with_sticky = quotient | u64::from(remainder != 0);
        Self::round_and_pack(negative, quotient_with_sticky, numerator_exponent - denominator_exponent - 32)
    }
    fn signed_zero(negative: bool) -> Self {
        if negative { Self::NEGATIVE_ZERO } else { Self::POSITIVE_ZERO }
    }
    fn signed_infinity(negative: bool) -> Self {
        if negative { Self::NEGATIVE_INFINITY } else { Self::POSITIVE_INFINITY }
    }
    /// Для конечного ненулевого x: |x| = significand * 2^(exponent - 23).
    /// Старший бит significand всегда на позиции 23, в том числе для subnormal.
    fn normalized_significand_and_exponent(self) -> (u64, i32) {
        if self.exponent_as_u32() != 0 {
            (HIDDEN_BIT | u64::from(self.fraction_as_u32()), self.exponent_as_u32() as i32 - 127)
        } else {
            let fraction = u64::from(self.fraction_as_u32());
            let shift = fraction.leading_zeros() - 40;
            (fraction << shift, -126 - shift as i32)
        }
    }
    /// Упаковываем magnitude * 2^power с одним округлением, включая область subnormal.
    /// magnitude содержит точные биты либо младший sticky, который будет отброшен.
    fn round_and_pack(negative: bool, magnitude: u64, power: i32) -> Self {
        if magnitude == 0 {
            return Self::signed_zero(negative);
        }
        let highest_bit_index = 63 - magnitude.leading_zeros() as i32;
        let mut exponent = highest_bit_index + power;
        // Для normal оставляем 24 значащих бита. Для subnormal шаг всегда 2^-149.
        let output_power = (exponent - 23).max(-149);
        let discarded_bit_count = output_power - power;
        let mut rounded_significand = if discarded_bit_count > 0 {
            round_right_to_nearest_even(magnitude, discarded_bit_count as u32)
        } else {
            magnitude << (-discarded_bit_count as u32)
        };
        let magnitude_bits = if exponent < -126 {
            // Перенос округления может превратить subnormal в normal:
            // бит 23 становится единицей, то есть младшим битом поля порядка.
            rounded_significand as u32
        } else {
            if rounded_significand >= (HIDDEN_BIT << 1) {
                rounded_significand >>= 1;
                exponent += 1;
            }
            if exponent > 127 {
                return Self::signed_infinity(negative);
            }
            (((exponent + 127) as u32) << 23) | (rounded_significand as u32 & FRACTION_MASK)
        };
        // From разбирает готовые 32 бита и одновременно определяет класс результата.
        Self::from((u32::from(negative) << 31) | magnitude_bits)
    }
}
// Обычный >> забывает отброшенные единицы. Здесь все они объединяются через OR в бит 0.
fn shift_right_with_sticky_bit(value: u64, shift: u32) -> u64 {
    if shift == 0 {
        value
    } else if shift >= 64 {
        u64::from(value != 0)
    } else {
        (value >> shift) | u64::from(value & ((1_u64 << shift) - 1) != 0)
    }
}
fn round_right_to_nearest_even(value: u64, shift: u32) -> u64 {
    if shift > 64 {
        return 0;
    }
    if shift == 64 {
        // Ровно половина между 0 и 1 округляется к чётному 0.
        return u64::from(value > (1_u64 << 63));
    }
    let kept_bits = value >> shift;
    let discarded_bits = value & ((1_u64 << shift) - 1);
    let halfway = 1_u64 << (shift - 1);
    let round_up = discarded_bits > halfway || (discarded_bits == halfway && kept_bits & 1 != 0);
    kept_bits + u64::from(round_up)
}
