//! Учебная арифметика binary32 через целые числа. Методы Float32 не используют встроенные f32/f64.
//! Режим округления: к ближайшему, при равенстве расстояний — к чётному (ties to even).
//! Все операции с NaN возвращают один quiet NaN; флаги исключений не моделируются.

fn main() -> Result<(), &'static str> {
    // Примеры выполняются без вывода; значения можно посмотреть в отладчике.
    // Знак, 8 бит порядка и 23 бита дробной части заданы отдельно.
    let _number = Float32::try_from_parts(
        1,
        [1, 0, 0, 0, 0, 0, 0, 0],
        [1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    )?;
    let one_and_half = Float32::try_from_parts(
        0,
        [0, 1, 1, 1, 1, 1, 1, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    )?;
    let two_and_quarter = Float32::try_from_parts(
        0,
        [1, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    )?;
    let _sum = one_and_half.calc_sum_by_aligning_exponents(two_and_quarter);
    let _difference = one_and_half.calc_difference_by_negating_and_adding(two_and_quarter);
    let _product = one_and_half.calc_product_by_multiplying_significands(two_and_quarter);
    let _quotient = one_and_half.calc_quotient_by_dividing_significands(two_and_quarter);
    let one = Float32::try_from_parts(
        0,
        [0, 1, 1, 1, 1, 1, 1, 1],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    )?;
    let half_step = Float32::try_from_parts(
        0,
        [0, 1, 1, 0, 0, 1, 1, 1],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    )?;
    let _rounded = one.calc_sum_by_aligning_exponents(half_step);
    let smallest_normal = Float32::try_from_parts(
        0,
        [0, 0, 0, 0, 0, 0, 0, 1],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    )?;
    let two = Float32::try_from_parts(
        0,
        [1, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    )?;
    let _subnormal = smallest_normal.calc_quotient_by_dividing_significands(two);
    let _invalid = Float32::ZERO.calc_product_by_multiplying_significands(Float32::INFINITY);
    let _negative_zero = Float32::ZERO.negate_by_flipping_sign_bit();
    Ok(())
}

const FRACTION_MASK: u32 = 0x007f_ffff;
const HIDDEN_BIT: u64 = 1 << 23;

/// 32 отдельных бита, каждый хранится как u8 со значением 0 или 1.
/// Поля занимают 32 байта: это наглядное учебное представление, не компактный f32.
/// PartialEq намеренно не выводим: совпадение битов отличается от равенства чисел
/// (+0 и -0 равны численно, а NaN не равен даже самому себе).
#[derive(Clone, Copy, Debug)]
struct Float32 {
    sign: u8,
    exponent_bits: [u8; 8],
    fraction_bits: [u8; 23],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum FloatClass {
    Zero,
    Subnormal,
    Normal,
    Infinity,
    Nan,
}

impl Float32 {
    const ZERO: Self = Self {
        sign: 0,
        exponent_bits: [0, 0, 0, 0, 0, 0, 0, 0],
        fraction_bits: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    };
    const NEGATIVE_ZERO: Self = Self {
        sign: 1,
        exponent_bits: [0, 0, 0, 0, 0, 0, 0, 0],
        fraction_bits: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    };
    const INFINITY: Self = Self {
        sign: 0,
        exponent_bits: [1, 1, 1, 1, 1, 1, 1, 1],
        fraction_bits: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    };
    const NAN: Self = Self {
        sign: 0,
        exponent_bits: [1, 1, 1, 1, 1, 1, 1, 1],
        fraction_bits: [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    };

    // Знак, 8 бит порядка и 23 бита дробной части заданы отдельно.
    // Проверка запрещает значения 2..255: каждый элемент — именно бит.
    fn try_from_parts(sign: u8, exponent_bits: [u8; 8], fraction_bits: [u8; 23]) -> Result<Self, &'static str> {
        if sign > 1 || exponent_bits.iter().chain(fraction_bits.iter()).any(|bit| *bit > 1) {
            return Err("бит должен быть 0 или 1");
        }
        Ok(Self {
            sign,
            exponent_bits,
            fraction_bits,
        })
    }

    /// Собираем 8 бит порядка со смещением 127, ещё не показатель степени.
    fn exponent_as_u32(self) -> u32 {
        self.exponent_bits.iter().fold(0, |value, bit| (value << 1) | u32::from(*bit))
    }

    /// Собираем 23 бита дробной части; скрытая единица сюда не входит.
    fn fraction_as_u32(self) -> u32 {
        self.fraction_bits.iter().fold(0, |value, bit| (value << 1) | u32::from(*bit))
    }

    fn classify(self) -> FloatClass {
        match (self.exponent_as_u32(), self.fraction_as_u32()) {
            (0, 0) => FloatClass::Zero,
            (0, _) => FloatClass::Subnormal,
            (255, 0) => FloatClass::Infinity,
            (255, _) => FloatClass::Nan,
            _ => FloatClass::Normal,
        }
    }

    const fn negate_by_flipping_sign_bit(self) -> Self {
        Self { sign: self.sign ^ 1, ..self }
    }

    /// Выравниваем порядки, складываем значащие части, нормализуем и округляем.
    fn calc_sum_by_aligning_exponents(self, other: Self) -> Self {
        let left_class = self.classify();
        let right_class = other.classify();
        if left_class == FloatClass::Nan || right_class == FloatClass::Nan {
            return Self::NAN;
        }
        if left_class == FloatClass::Infinity || right_class == FloatClass::Infinity {
            if left_class == right_class && (self.sign == 1) != (other.sign == 1) {
                return Self::NAN; // +∞ + -∞ не определено.
            }
            return if left_class == FloatClass::Infinity { self } else { other };
        }
        if left_class == FloatClass::Zero && right_class == FloatClass::Zero {
            // Только -0 + -0 даёт -0 при выбранном режиме округления.
            return Self::signed_zero((self.sign == 1) && (other.sign == 1));
        }
        if left_class == FloatClass::Zero {
            return other;
        }
        if right_class == FloatClass::Zero {
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
            return Self::ZERO; // Точная компенсация x + (-x).
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
        if left_class == FloatClass::Nan || right_class == FloatClass::Nan {
            return Self::NAN;
        }
        if left_class == FloatClass::Infinity || right_class == FloatClass::Infinity {
            if left_class == FloatClass::Zero || right_class == FloatClass::Zero {
                return Self::NAN; // 0 * ∞.
            }
            return Self::signed_infinity(negative);
        }
        if left_class == FloatClass::Zero || right_class == FloatClass::Zero {
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
        if left_class == FloatClass::Nan
            || right_class == FloatClass::Nan
            || (left_class == FloatClass::Zero && right_class == FloatClass::Zero)
            || (left_class == FloatClass::Infinity && right_class == FloatClass::Infinity)
        {
            return Self::NAN;
        }
        if left_class == FloatClass::Infinity || right_class == FloatClass::Zero {
            return Self::signed_infinity(negative);
        }
        if left_class == FloatClass::Zero || right_class == FloatClass::Infinity {
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
        if negative { Self::NEGATIVE_ZERO } else { Self::ZERO }
    }

    fn signed_infinity(negative: bool) -> Self {
        Self {
            sign: u8::from(negative),
            ..Self::INFINITY
        }
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
        // Сразу записываем результат в три поля: от старшего разряда к младшему.
        Self {
            sign: u8::from(negative),
            exponent_bits: std::array::from_fn(|index| ((magnitude_bits >> (30 - index)) & 1) as u8),
            fraction_bits: std::array::from_fn(|index| ((magnitude_bits >> (22 - index)) & 1) as u8),
        }
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
