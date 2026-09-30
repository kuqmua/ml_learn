//! Короткий вывод промежуточных шагов при запуске учебных примеров.

static ENABLED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
static PRINTED: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
static COUNTS: std::sync::OnceLock<
    std::sync::Mutex<std::collections::HashMap<(&'static str, u32), usize>>,
> = std::sync::OnceLock::new();

const MAX_LINES: usize = 240;
const MAX_VALUE_CHARS: usize = 180;

/// Включает трассировку для `cargo run`; вызовы функций из тестов остаются тихими.
pub fn enable() {
    ENABLED.store(true, std::sync::atomic::Ordering::Relaxed);
    println!("Промежуточные шаги (повторы 1–4, затем 8, 64, 512, …):");
}

/// Останавливает трассировку перед построением графика: его сетка не относится к расчёту примера.
pub fn disable() {
    ENABLED.store(false, std::sync::atomic::Ordering::Relaxed);
}

/// Показывает значение после вычисления, ограничивая вывод длинных циклов и коллекций.
pub fn show<T: std::fmt::Debug + ?Sized>(file: &'static str, line: u32, label: &str, value: &T) {
    if !ENABLED.load(std::sync::atomic::Ordering::Relaxed) {
        return;
    }
    let counts = COUNTS.get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()));
    let occurrence = {
        let mut counts = counts.lock().expect("счётчик шагов");
        let count = counts.entry((file, line)).or_default();
        *count += 1;
        *count
    };
    if occurrence > 4 && !(occurrence.is_power_of_two() && occurrence.trailing_zeros() % 3 == 0) {
        return;
    }
    let printed = PRINTED.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    if printed == MAX_LINES {
        println!("  … дальнейшие промежуточные шаги скрыты, итоговый вывод остаётся виден");
    }
    if printed >= MAX_LINES {
        return;
    }
    let mut display = LimitedDebug::new(MAX_VALUE_CHARS);
    let _ = std::fmt::Write::write_fmt(&mut display, format_args!("{value:?}"));
    let suffix = if display.clipped { "…" } else { "" };
    let part = file
        .split('/')
        .find_map(|segment| {
            let mut parts = segment.splitn(3, '-');
            let lesson = parts.next()?;
            let block = parts.next()?;
            parts.next()?;
            if lesson.len() == 4
                && lesson.starts_with('l')
                && lesson[1..].bytes().all(|byte| byte.is_ascii_digit())
                && !block.is_empty()
                && block.bytes().all(|byte| byte.is_ascii_digit())
            {
                segment.get(..lesson.len() + 1 + block.len())
            } else {
                None
            }
        })
        .unwrap_or("урок");
    if occurrence == 1 {
        println!("  [{part}] {label} = {}{suffix}", display.text);
    } else {
        println!(
            "  [{part}] {label} (повтор {occurrence}) = {}{suffix}",
            display.text
        );
    }
}

struct LimitedDebug {
    text: String,
    chars: usize,
    limit: usize,
    clipped: bool,
}

impl LimitedDebug {
    fn new(limit: usize) -> Self {
        Self {
            text: String::new(),
            chars: 0,
            limit,
            clipped: false,
        }
    }
}

impl std::fmt::Write for LimitedDebug {
    fn write_str(&mut self, text: &str) -> std::fmt::Result {
        for character in text.chars() {
            if self.chars >= self.limit {
                self.clipped = true;
                return Err(std::fmt::Error);
            }
            self.text.push(character);
            self.chars += 1;
        }
        Ok(())
    }
}

/// Печатает имя переменной вместе со значением после её вычисления.
#[macro_export]
macro_rules! trace_step {
    ($value:expr) => {
        $crate::show(file!(), line!(), stringify!($value), &$value)
    };
}
