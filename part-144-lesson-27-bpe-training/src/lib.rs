//! Обучение byte-level BPE.

use std::collections::BTreeMap;

/// Учебный byte-level BPE: ID 0..=255 обозначают одиночные байты.
#[derive(Debug)]
pub struct BytePairEncoding {
    /// Содержимое каждого токена нужно для обратного преобразования.
    pub pieces: Vec<Vec<u8>>,
    /// Порядок слияний важен при кодировании новых строк.
    pub merges: Vec<(usize, usize)>,
}

impl BytePairEncoding {
    /// Учит пары только на переданном обучающем корпусе.
    pub fn train_from_corpus(corpus: &[&str], merge_count: usize) -> Self {
        // Начальный словарь покрывает любой UTF-8 текст.
        let mut pieces = (0..=255).map(|byte| vec![byte as u8]).collect::<Vec<_>>();
        let mut rows = corpus
            .iter()
            .map(|text| text.bytes().map(usize::from).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let mut merges = Vec::new();
        for _ in 0..merge_count {
            // Частоты считаем только у соседних токенов внутри одной строки.
            let mut frequencies = BTreeMap::<(usize, usize), usize>::new();
            for row in &rows {
                for pair in row.windows(2) {
                    *frequencies.entry((pair[0], pair[1])).or_default() += 1;
                }
            }
            // BTreeMap делает выбор при равных частотах воспроизводимым.
            let Some((&pair, _)) = frequencies
                .iter()
                .max_by_key(|(pair, count)| (*count, std::cmp::Reverse(**pair)))
            else {
                break;
            };
            let token_identifier = pieces.len();
            let mut joined = pieces[pair.0].clone();
            joined.extend_from_slice(&pieces[pair.1]);
            pieces.push(joined);
            merges.push(pair);
            // Заменяем выбранную пару во всём обучающем корпусе.
            for row in &mut rows {
                *row = merge_pair(row, pair, token_identifier);
            }
        }
        Self { pieces, merges }
    }

    /// Применяет сохранённые слияния к новому тексту в порядке обучения.
    pub fn encode(&self, text: &str) -> Vec<usize> {
        let mut token_identifiers = text.bytes().map(usize::from).collect::<Vec<_>>();
        for (offset, &pair) in self.merges.iter().enumerate() {
            token_identifiers = merge_pair(&token_identifiers, pair, 256 + offset);
        }
        token_identifiers
    }

    /// Восстанавливает байты и проверяет корректность UTF-8.
    pub fn decode(&self, token_identifiers: &[usize]) -> Result<String, String> {
        let mut bytes = Vec::new();
        for &token_identifier in token_identifiers {
            let piece = self
                .pieces
                .get(token_identifier)
                .ok_or("неизвестный ID токена")?;
            bytes.extend_from_slice(piece);
        }
        String::from_utf8(bytes).map_err(|error| error.to_string())
    }
}

// Слияния не перекрываются: каждую исходную позицию используем ровно один раз.
fn merge_pair(
    token_identifiers: &[usize],
    pair: (usize, usize),
    new_identifier: usize,
) -> Vec<usize> {
    let mut result = Vec::new();
    let mut index = 0;
    while index < token_identifiers.len() {
        if token_identifiers.get(index) == Some(&pair.0)
            && token_identifiers.get(index + 1) == Some(&pair.1)
        {
            result.push(new_identifier);
            index += 2;
        } else {
            result.push(token_identifiers[index]);
            index += 1;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::BytePairEncoding;
    #[test]
    fn roundtrip_and_unseen_utf8() {
        let model = BytePairEncoding::train_from_corpus(&["мама мыла", "мама дома"], 12);
        for text in ["мама", "кот 🐈", "", "\0"] {
            assert_eq!(model.decode(&model.encode(text)).unwrap(), text);
        }
    }
}
