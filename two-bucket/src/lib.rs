use std::collections::HashSet;

#[derive(PartialEq, Eq, Debug)]
pub enum Bucket {
    One,
    Two,
}

/// A struct to hold your results in.
#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    /// The total number of "moves" it should take to reach the desired number of liters, including
    /// the first fill.
    pub moves: u8,
    /// Which bucket should end up with the desired number of liters? (Either "one" or "two")
    pub goal_bucket: Bucket,
    /// How many liters are left in the other bucket?
    pub other_bucket: u8,
}

/// Solve the bucket problem
pub fn solve(
    capacity_1: u8,
    capacity_2: u8,
    goal: u8,
    start_bucket: &Bucket,
) -> Option<BucketStats> {
    let mut queue = vec![(0, 0)];
    let mut set: HashSet<(u8, u8)> = HashSet::new();
    set.insert((0, 0));

    for moves in 1.. {
        if queue.is_empty() {
            break;
        }

        let mut candidates = Vec::with_capacity(queue.len());
        while let Some(p) = queue.pop() {
            // Fill 1
            if p.0 < capacity_1 {
                candidates.push((capacity_1, p.1));
            }
            // Fill 2
            if p.1 < capacity_2 {
                candidates.push((p.0, capacity_2));
            }
            // Empty 1
            if p.0 > 0 {
                candidates.push((0, p.1));
            }
            // Empty 2
            if p.1 > 0 {
                candidates.push((p.0, 0));
            }
            // Pour from 1 to 2
            if p.0 > 0 && p.1 < capacity_2 {
                let dif = p.0.min(capacity_2 - p.1);
                candidates.push((p.0 - dif, p.1 + dif));
            }
            // Pour from 2 to 1
            if p.1 > 0 && p.0 < capacity_1 {
                let dif = p.1.min(capacity_1 - p.0);
                candidates.push((p.0 + dif, p.1 - dif));
            }
        }
        for c in candidates {
            let opposite_case = match start_bucket {
                Bucket::One => c.0 == 0 && c.1 == capacity_2,
                Bucket::Two => c.0 == capacity_1 && c.1 == 0,
            };
            if !set.contains(&c) && !opposite_case {
                if c.0 == goal {
                    return Some(BucketStats {
                        goal_bucket: Bucket::One,
                        other_bucket: c.1,
                        moves,
                    });
                }
                if c.1 == goal {
                    return Some(BucketStats {
                        goal_bucket: Bucket::Two,
                        other_bucket: c.0,
                        moves,
                    });
                }
                set.insert(c);
                queue.push(c);
            }
        }
    }
    None
}
