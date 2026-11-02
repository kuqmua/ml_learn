use crate::rng::Rng;
use candle_core::{Device, Result, Tensor};
use std::collections::{BTreeMap, BTreeSet};

pub(crate) const TEXT: &str = include_str!("../data/input.txt");

pub(crate) struct Vocabulary {
    pub(crate) chars: Vec<char>,
    ids: BTreeMap<char, u32>,
}
impl Vocabulary {
    pub(crate) fn new(text: &str) -> Self {
        let chars: Vec<char> = text.chars().collect::<BTreeSet<char>>().into_iter().collect::<Vec<char>>();
        let ids: BTreeMap<char, u32> = chars
            .iter()
            .enumerate()
            .map(|(id, &c): (usize, &char)| -> (char, u32) { (c, id as u32) })
            .collect::<BTreeMap<char, u32>>();
        Self { chars, ids }
    }
    pub(crate) fn encode(&self, text: &str) -> Result<Vec<u32>> {
        text.chars()
            .map(|c: char| -> Result<u32> {
                self.ids
                    .get(&c)
                    .copied()
                    .ok_or_else(|| -> candle_core::Error { candle_core::Error::Msg(format!("Символ {c:?} отсутствует в словаре")) })
            })
            .collect::<Result<Vec<u32>>>()
    }
    pub(crate) fn decode(&self, ids: &[u32]) -> Result<String> {
        ids.iter()
            .map(|&id: &u32| -> Result<char> {
                self.chars
                    .get(id as usize)
                    .copied()
                    .ok_or_else(|| -> candle_core::Error { candle_core::Error::Msg(format!("ID {id} вне словаря")) })
            })
            .collect::<Result<String>>()
    }
}

// Split ДО выбора окон: ни одно train-окно не пересекает начало validation.
pub(crate) fn split(ids: &[u32]) -> (&[u32], &[u32]) {
    ids.split_at(ids.len() * 9 / 10)
}

pub(crate) fn batch(data: &[u32], b: usize, t: usize, rng: &mut Rng) -> Result<(Tensor, Tensor)> {
    if b == 0 || t == 0 || data.len() <= t {
        candle_core::bail!("Для batch нужны B>0 и 0<T<длины split");
    }
    let mut x: Vec<u32> = Vec::<u32>::with_capacity(b * t);
    let mut y: Vec<u32> = Vec::<u32>::with_capacity(b * t);
    for _ in 0usize..b {
        let start: usize = rng.index(data.len() - t);
        x.extend_from_slice(&data[start..start + t]);
        y.extend_from_slice(&data[start + 1..start + t + 1]);
    }
    Ok((Tensor::from_vec(x, (b, t), &Device::Cpu)?, Tensor::from_vec(y, (b, t), &Device::Cpu)?))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn unicode_roundtrip_and_shifted_batches_stay_in_split() -> Result<()> {
        let v: Vocabulary = Vocabulary::new("я🦀а\n");
        assert_eq!(v.decode(&v.encode("🦀я\n")?)?, "🦀я\n");
        assert!(v.encode("?").is_err());
        assert!(v.decode(&[99]).is_err());
        let ids: Vec<u32> = (0u32..100).collect::<Vec<u32>>();
        let (train, val): (&[u32], &[u32]) = split(&ids);
        [train, val].into_iter().try_for_each(|data: &[u32]| -> Result<()> {
            let (x, y): (Tensor, Tensor) = batch(data, 32, 4, &mut Rng::new(2))?;
            x.to_vec2::<u32>()?.iter().zip(y.to_vec2::<u32>()?).for_each(|(x, y): (&Vec<u32>, Vec<u32>)| {
                x.iter().zip(y).for_each(|(&a, b): (&u32, u32)| {
                    assert_eq!(a + 1, b);
                    assert!(data.contains(&a) && data.contains(&b));
                });
            });
            Ok(())
        })?;
        assert!(batch(&[0, 1], 1, 2, &mut Rng::new(0)).is_err());
        Ok(())
    }
}
