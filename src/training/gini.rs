pub fn gini_index<L>(labels: &[L]) -> f32
where
    L: Eq + Copy,
{
    let mut classes = [const {None}; 256];
    let mut counts = [0u32; 256];
    let mut n_unique = 0;
    let mut total = 0u32;

    // Count frequency of unique labels/classes
    for &label in labels {
        let mut found = false;
        for i in 0..n_unique {
            if let Some(existing) = classes[i] {
                if existing == label {
                    counts[i] += 1;
                    found = true;
                    break;
                }
            }
        }
        if !found && n_unique < 256 {
            classes[n_unique] = Some(label);
            counts[n_unique] = 1;
            n_unique += 1;
        }
        total += 1;
    }

    if total == 0 {
        return 0.0;
    }

    // Gini index
    let mut sum_p_sq = 0.0;
    let total_f = total as f32;
    for &count in &counts[..n_unique] {
        let p = count as f32 / total_f;
        sum_p_sq += p * p;
    }
    1.0 - sum_p_sq
}

pub fn information_gain_gini<L>(
    parent: &[L],
    left: &[L],
    right: &[L],
) -> f32
where
    L: Eq + Copy,
{
    let n_left = left.len() as f32;
    let n_right = right.len() as f32;
    let n_total = n_left + n_right;

    if n_total == 0.0 {
        return 0.0;
    }

    let parent = gini_index(parent);
    let left = gini_index(left);
    let right = gini_index(right);

    // this is supposed to be a weighted average between the two children.
    let child_gini = (n_left / n_total) * left + (n_right / n_total) * right;

    parent - child_gini
}