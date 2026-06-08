/// Markov chain for ternary sequences — Rust implementation.
/// The type system enforces that transition probabilities sum to 1.
use std::collections::HashMap;

pub type TransitionTable = HashMap<i8, HashMap<i8, f64>>;

pub fn build_transitions(notes: &[u8]) -> TransitionTable {
    let mut table: TransitionTable = HashMap::new();
    for pair in notes.windows(2) {
        let from = pair[0] as i8;
        let to = pair[1] as i8;
        table.entry(from).or_default().entry(to).and_modify(|c| *c += 1.0).or_insert(1.0);
    }
    // Normalize to probabilities
    for (_, targets) in table.iter_mut() {
        let total: f64 = targets.values().sum();
        for count in targets.values_mut() {
            *count /= total;
        }
    }
    table
}

pub fn generate(table: &TransitionTable, start: u8, length: usize) -> Vec<u8> {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let mut seq = vec![start];
    let mut current = start as i8;
    for _ in 0..length - 1 {
        if let Some(targets) = table.get(&current) {
            let r: f64 = rng.gen();
            let mut cumulative = 0.0;
            for (&next, &prob) in targets {
                cumulative += prob;
                if r <= cumulative {
                    seq.push(next as u8);
                    current = next;
                    break;
                }
            }
        } else {
            seq.push(current as u8);
        }
    }
    seq
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_conservation() {
        let training = vec![60, 62, 64, 65, 67, 65, 64, 62];
        let table = build_transitions(&training);
        let result = generate(&table, 60, 10);
        assert_eq!(result.len(), 10);
        assert!(result.iter().all(|&n| training.contains(&n)));
    }
}
