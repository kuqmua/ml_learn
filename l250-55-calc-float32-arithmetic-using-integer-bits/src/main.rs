fn main() -> Result<(), &'static str> {
    //! Устройство binary32: знак, 8 бит порядка, 23 бита дробной части и категория числа.
    /// 32 отдельных бита, каждый хранится как u8 со значением 0 или 1.
    /// Биты занимают 32 байта; категория вычисляется методом classify(). Это не компактный f32.
    /// PartialEq намеренно не выводим: совпадение битов отличается от равенства чисел
    /// (+0 и -0 равны численно, а NaN не равен даже самому себе).
    #[derive(Clone, Copy, Debug)]
    struct Float32 {
        sign: u8,
        exponent_bits: [u8; 8],
        // Хранимые биты мантиссы: дробная часть без неявной ведущей единицы.
        mantissa_bits: [u8; 23],
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
    // В отдельных u8 можно передать 2 или 255 вместо бита, поэтому нужен TryFrom.
    impl TryFrom<(u8, [u8; 8], [u8; 23])> for Float32 {
        type Error = &'static str;
        fn try_from((sign, exponent_bits, mantissa_bits): (u8, [u8; 8], [u8; 23])) -> Result<Self, Self::Error> {
            if sign > 1 || exponent_bits.iter().chain(mantissa_bits.iter()).any(|bit| *bit > 1) {
                return Err("бит должен быть 0 или 1");
            }
            Ok(Self {
                sign,
                exponent_bits,
                mantissa_bits,
            })
        }
    }
    impl Float32 {
        // Собираем массивы из целых значений полей, без преобразования во float.
        fn from_integer_fields(sign: u8, exponent: u32, mantissa: u32) -> Self {
            let exponent_bits = std::array::from_fn(|index| {
                ((exponent / 2_u32.pow(7 - index as u32)) % 2) as u8
            });
            let mantissa_bits = std::array::from_fn(|index| {
                ((mantissa / 2_u32.pow(22 - index as u32)) % 2) as u8
            });
            Self { sign, exponent_bits, mantissa_bits }
        }

        fn add(self, other: Self) -> Self {
            // Массив читаем как двоичное целое. Например, [1, 0, 1] → 5.
            fn bits_as_integer(bits: &[u8]) -> u32 {
                let mut value = 0;
                for bit in bits {
                    value = value * 2 + u32::from(*bit);
                }
                value
            }
            // Делим на степень двойки, сохраняя признак отброшенных единиц.
            // Этот признак в младшем бите нужен для правильного округления.
            fn divide_with_sticky(value: u64, places: u32) -> u64 {
                if places == 0 {
                    return value;
                }
                if places >= 64 {
                    return u64::from(value != 0);
                }
                let divisor = 2_u64.pow(places);
                let quotient = value / divisor;
                if value % divisor != 0 { quotient | 1 } else { quotient }
            }
            let left_exponent = bits_as_integer(&self.exponent_bits);
            let right_exponent = bits_as_integer(&other.exponent_bits);
            let left_fraction = bits_as_integer(&self.mantissa_bits);
            let right_fraction = bits_as_integer(&other.mantissa_bits);
            // Порядок 255 обозначает NaN или бесконечность.
            if (left_exponent == 255 && left_fraction != 0)
                || (right_exponent == 255 && right_fraction != 0)
            {
                return Self::from_integer_fields(0, 255, 4_194_304); // Quiet NaN.
            }
            if left_exponent == 255 && right_exponent == 255 && self.sign != other.sign {
                return Self::from_integer_fields(0, 255, 4_194_304); // +∞ + −∞ = NaN.
            }
            if left_exponent == 255 { return self; }
            if right_exponent == 255 { return other; }
            // У Normal добавляем скрытую единицу на позиции 23: 2^23 = 8388608.
            // Здесь мантисса целая: её дробная точка мысленно на 23 позиции справа.
            let left_mantissa = u64::from(left_fraction)
                + if left_exponent == 0 { 0 } else { 8_388_608 };
            let right_mantissa = u64::from(right_fraction)
                + if right_exponent == 0 { 0 } else { 8_388_608 };
            // Subnormal и Normal с полем порядка 1 имеют одну степень −126.
            let left_exponent = left_exponent.max(1);
            let right_exponent = right_exponent.max(1);
            if left_mantissa == 0 && right_mantissa == 0 {
                // Только −0 + −0 даёт −0; остальные сочетания дают +0.
                return Self::from_integer_fields(self.sign & other.sign, 0, 0);
            }
            // Ставим больший по модулю операнд первым, чтобы вычитать без отрицательных u64.
            let (large, small, mut exponent, small_exponent, sign, small_sign) =
                if (left_exponent, left_mantissa) >= (right_exponent, right_mantissa) {
                    (left_mantissa, right_mantissa, left_exponent, right_exponent, self.sign, other.sign)
                } else {
                    (right_mantissa, left_mantissa, right_exponent, left_exponent, other.sign, self.sign)
                };
            // Умножение на 8 оставляет три младших бита для округления.
            let large = large * 8;
            let small = divide_with_sticky(small * 8, exponent - small_exponent);
            let mut sum = if sign == small_sign { large + small } else { large - small };
            if sum == 0 {
                return Self::from_integer_fields(0, 0, 0);
            }
            // Если мантисса достигла 2, делим её на 2 и увеличиваем порядок.
            if sum >= 16_777_216 * 8 {
                sum = divide_with_sticky(sum, 1);
                exponent += 1;
            }
            // После вычитания восстанавливаем ведущую единицу, пока степень выше −126.
            while sum < 8_388_608 * 8 && exponent > 1 {
                sum *= 2;
                exponent -= 1;
            }
            // Округляем к ближайшему; при равенстве расстояний — к чётному.
            let mut mantissa = sum / 8;
            let discarded = sum % 8;
            if discarded > 4 || (discarded == 4 && mantissa % 2 == 1) {
                mantissa += 1;
            }
            if mantissa >= 16_777_216 {
                mantissa /= 2;
                exponent += 1;
            }
            if exponent >= 255 {
                return Self::from_integer_fields(sign, 255, 0); // Переполнение → бесконечность.
            }
            if mantissa < 8_388_608 {
                // До скрытой единицы не дошли: результат остаётся Subnormal.
                Self::from_integer_fields(sign, 0, mantissa as u32)
            } else {
                // В том числе переход Subnormal → Normal: поле порядка становится 1.
                // Скрытую единицу убираем из хранимых 23 битов.
                Self::from_integer_fields(sign, exponent, (mantissa - 8_388_608) as u32)
            }
        }

        fn classify(&self) -> FloatClass {
            let exponent_is_zero = self.exponent_bits == [0; 8];
            let exponent_is_all_ones = self.exponent_bits == [1; 8];
            let mantissa_is_zero = self.mantissa_bits == [0; 23];
            match (exponent_is_zero, exponent_is_all_ones, mantissa_is_zero) {
                (true, _, true) => if self.sign == 0 { FloatClass::PositiveZero } else { FloatClass::NegativeZero },
                (true, _, false) => FloatClass::Subnormal,
                (_, true, true) => if self.sign == 0 { FloatClass::PositiveInfinity } else { FloatClass::NegativeInfinity },
                (_, true, false) => if self.sign == 0 { FloatClass::PositiveNan } else { FloatClass::NegativeNan },
                // Порядок от 1 до 254: не все биты нулевые и не все единичные.
                (false, false, true) => FloatClass::Normal, // Все биты мантиссы нулевые.
                (false, false, false) => FloatClass::Normal, // В мантиссе есть единицы.
            }
        }
        fn to_f32(self) -> f32 {
            // Особые категории не вычисляются по формуле нормального числа.
            match self.classify() {
                FloatClass::PositiveZero => return 0.0,
                FloatClass::NegativeZero => return -0.0,
                FloatClass::PositiveInfinity => return f32::INFINITY,
                FloatClass::NegativeInfinity => return f32::NEG_INFINITY,
                // Здесь показываем категорию NaN, не сохраняем его payload.
                FloatClass::PositiveNan => return f32::NAN,
                FloatClass::NegativeNan => return -f32::NAN,
                FloatClass::Normal | FloatClass::Subnormal => {}
            }
            // 1. Знак: бит 1 превращается в множитель −1.
            let sign_multiplier = if self.sign == 0 { 1.0_f32 } else { -1.0_f32 };
            let stored_exponent = i32::from(self.exponent_bits[0]) * 128
                + i32::from(self.exponent_bits[1]) * 64
                + i32::from(self.exponent_bits[2]) * 32
                + i32::from(self.exponent_bits[3]) * 16
                + i32::from(self.exponent_bits[4]) * 8
                + i32::from(self.exponent_bits[5]) * 4
                + i32::from(self.exponent_bits[6]) * 2
                + i32::from(self.exponent_bits[7]) * 1;

            let mantissa_fraction = f32::from(self.mantissa_bits[0]) * 0.5
                + f32::from(self.mantissa_bits[1]) * 0.25
                + f32::from(self.mantissa_bits[2]) * 0.125
                + f32::from(self.mantissa_bits[3]) * 0.0625
                + f32::from(self.mantissa_bits[4]) * 0.03125
                + f32::from(self.mantissa_bits[5]) * 0.015625
                + f32::from(self.mantissa_bits[6]) * 0.0078125
                + f32::from(self.mantissa_bits[7]) * 0.00390625
                + f32::from(self.mantissa_bits[8]) * 0.001953125
                + f32::from(self.mantissa_bits[9]) * 0.0009765625
                + f32::from(self.mantissa_bits[10]) * 0.00048828125
                + f32::from(self.mantissa_bits[11]) * 0.000244140625
                + f32::from(self.mantissa_bits[12]) * 0.0001220703125
                + f32::from(self.mantissa_bits[13]) * 0.00006103515625
                + f32::from(self.mantissa_bits[14]) * 0.000030517578125
                + f32::from(self.mantissa_bits[15]) * 0.0000152587890625
                + f32::from(self.mantissa_bits[16]) * 0.00000762939453125
                + f32::from(self.mantissa_bits[17]) * 0.000003814697265625
                + f32::from(self.mantissa_bits[18]) * 0.0000019073486328125
                + f32::from(self.mantissa_bits[19]) * 9.5367431640625E-7
                + f32::from(self.mantissa_bits[20]) * 4.76837158203125E-7
                + f32::from(self.mantissa_bits[21]) * 2.384185791015625E-7
                + f32::from(self.mantissa_bits[22]) * 1.1920928955078125E-7;

            // 4. У Normal добавляем скрытую 1 и вычитаем смещение 127.
            // В примере: mantissa = 1 + 0.625 = 1.625; exponent = 128 − 127 = 1.
            // У Subnormal скрытой 1 нет, а степень всегда −126.
            let (mantissa, exponent) = match self.classify() {
                FloatClass::Normal => (1.0 + mantissa_fraction, stored_exponent - 127),
                FloatClass::Subnormal => (mantissa_fraction, -126),
                _ => unreachable!("нули, бесконечности и NaN обработаны в начале to_f32"),
            };
            // 5. Именно масштаб 2^exponent задаёт положение двоичной точки.
            // powi возводит число в целую степень: 2^1 = 2.
            // В примере: −1 × 1.625 × 2 = −3.25.
            let scale = 2.0_f32.powi(exponent);
            sign_multiplier * mantissa * scale
        }
    }
    // Пример выполняется без вывода; значения можно посмотреть в отладчике.
    // Знак, 8 бит порядка и 23 бита дробной части заданы отдельно.
    // Для Normal: число = множитель знака (+1 или −1) × (1 + дробная часть) × 2^(порядок − 127).
    // Бит знака 0 означает плюс, 1 — минус. Порядок читаем как двоичное целое.
    // Биты дробной части имеют веса 1/2, 1/4, 1/8, ...; ведущая 1 хранится неявно.
    // Формула выше не применяется к PositiveZero, NegativeZero, Subnormal, PositiveInfinity, NegativeInfinity, PositiveNan и NegativeNan.
    // Разбираем число −3.25 по каждому биту. Индексы массивов начинаются с 0.
    // Бит 1 включает вес своей позиции, бит 0 не добавляет ничего.
    let number = Float32::try_from((
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
        // mantissa_bits — 23 бита после точки в двоичной системе.
        // Каждый следующий вес вдвое меньше предыдущего. Все значения ниже — десятичные.
        [
            1, // mantissa_bits[0]: вес 1/2 = 0.5; прибавляем 0.5.
            0, // mantissa_bits[1]: вес 1/4 = 0.25; прибавляем 0.
            1, // mantissa_bits[2]: вес 1/8 = 0.125; прибавляем 0.125.
            0, // mantissa_bits[3]: вес 1/16 = 0.0625; прибавляем 0.
            0, // mantissa_bits[4]: вес 1/32 = 0.03125; прибавляем 0.
            0, // mantissa_bits[5]: вес 1/64 = 0.015625; прибавляем 0.
            0, // mantissa_bits[6]: вес 1/128 = 0.0078125; прибавляем 0.
            0, // mantissa_bits[7]: вес 1/256 = 0.00390625; прибавляем 0.
            0, // mantissa_bits[8]: вес 1/512 = 0.001953125; прибавляем 0.
            0, // mantissa_bits[9]: вес 1/1024 = 0.0009765625; прибавляем 0.
            0, // mantissa_bits[10]: вес 1/2048 = 0.00048828125; прибавляем 0.
            0, // mantissa_bits[11]: вес 1/4096 = 0.000244140625; прибавляем 0.
            0, // mantissa_bits[12]: вес 1/8192 = 0.0001220703125; прибавляем 0.
            0, // mantissa_bits[13]: вес 1/16384 = 0.00006103515625; прибавляем 0.
            0, // mantissa_bits[14]: вес 1/32768 = 0.000030517578125; прибавляем 0.
            0, // mantissa_bits[15]: вес 1/65536 = 0.0000152587890625; прибавляем 0.
            0, // mantissa_bits[16]: вес 1/131072 = 0.00000762939453125; прибавляем 0.
            0, // mantissa_bits[17]: вес 1/262144 = 0.000003814697265625; прибавляем 0.
            0, // mantissa_bits[18]: вес 1/524288 = 0.0000019073486328125; прибавляем 0.
            0, // mantissa_bits[19]: вес 1/1048576 = 0.00000095367431640625; прибавляем 0.
            0, // mantissa_bits[20]: вес 1/2097152 = 0.000000476837158203125; прибавляем 0.
            0, // mantissa_bits[21]: вес 1/4194304 = 0.0000002384185791015625; прибавляем 0.
            0, // mantissa_bits[22]: вес 1/8388608 = 0.00000011920928955078125; прибавляем 0.
        ],
        // Сумма включённых весов дроби: 0.5 + 0.125 = 0.625.
        // Для Normal перед дробью подразумевается 1: получаем 1 + 0.625 = 1.625.
        // Эта дополнительная 1 не занимает место в массиве из 23 битов.
        // Применяем множитель из порядка и знак: 1.625 × 2 × (−1) = −3.25.
        // При порядке 0 или 255 действуют другие правила; они описаны в README.
    ))?;
    // Теперь из учебной структуры получаем встроенное число f32.
    let _value: f32 = number.to_f32(); // −3.25
    // −3.25 + 1.0 = −2.25.
    let one = Float32::try_from((0, [0, 1, 1, 1, 1, 1, 1, 1], [0; 23]))?;
    let _sum = number.add(one);

    // Самое большое положительное Subnormal + минимальный положительный шаг.
    let largest_subnormal = Float32::try_from((0, [0; 8], [1; 23]))?;
    let smallest_subnormal = Float32::try_from((
        0,
        [0; 8],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    ))?;
    // Результат: порядок [0, 0, 0, 0, 0, 0, 0, 1], мантисса [0; 23].
    // classify() вернёт Normal — это самое маленькое положительное нормальное число.
    let _smallest_normal = largest_subnormal.add(smallest_subnormal);
    Ok(())
}
