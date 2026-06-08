/// Performs binary search on a sorted slice. Returns `Some(index)` if found.
pub fn binary_search<T: Ord>(slice: &[T], target: &T) -> Option<usize> {
    let mut lo = 0usize;
    let mut hi = slice.len();
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        match slice[mid].cmp(target) {
            std::cmp::Ordering::Equal => return Some(mid),
            std::cmp::Ordering::Less => lo = mid + 1,
            std::cmp::Ordering::Greater => hi = mid,
        }
    }
    None
}

/// Sorts a mutable slice in-place using merge sort and returns a reference to
/// the sorted slice.
pub fn merge_sort<T: Ord + Clone>(slice: &mut [T]) {
    let len = slice.len();
    if len <= 1 {
        return;
    }
    let mid = len / 2;
    let mut left = slice[..mid].to_vec();
    let mut right = slice[mid..].to_vec();
    merge_sort(&mut left);
    merge_sort(&mut right);

    let (mut i, mut j, mut k) = (0, 0, 0);
    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            slice[k] = left[i].clone();
            i += 1;
        } else {
            slice[k] = right[j].clone();
            j += 1;
        }
        k += 1;
    }
    while i < left.len() {
        slice[k] = left[i].clone();
        i += 1;
        k += 1;
    }
    while j < right.len() {
        slice[k] = right[j].clone();
        j += 1;
        k += 1;
    }
}

/// Removes consecutive duplicates from a sorted slice, returning a new `Vec`.
pub fn dedup_sorted<T: Eq + Clone>(slice: &[T]) -> Vec<T> {
    if slice.is_empty() {
        return vec![];
    }
    let mut result = vec![slice[0].clone()];
    for item in &slice[1..] {
        if result.last() != Some(item) {
            result.push(item.clone());
        }
    }
    result
}

/// Flattens a nested `Vec<Vec<T>>` into a single `Vec<T>`.
pub fn flatten<T>(nested: Vec<Vec<T>>) -> Vec<T> {
    nested.into_iter().flatten().collect()
}

/// Groups consecutive equal elements into sub-vectors.
pub fn group_consecutive<T: Eq + Clone>(slice: &[T]) -> Vec<Vec<T>> {
    if slice.is_empty() {
        return vec![];
    }
    let mut groups: Vec<Vec<T>> = vec![vec![slice[0].clone()]];
    for item in &slice[1..] {
        if groups.last().unwrap().last() == Some(item) {
            groups.last_mut().unwrap().push(item.clone());
        } else {
            groups.push(vec![item.clone()]);
        }
    }
    groups
}

/// Returns a `Vec` containing only the unique elements of the input, preserving
/// order of first occurrence.
pub fn unique_preserve_order<T: Eq + std::hash::Hash + Clone>(slice: &[T]) -> Vec<T> {
    let mut seen = std::collections::HashSet::new();
    slice
        .iter()
        .filter(|item| seen.insert((*item).clone()))
        .cloned()
        .collect()
}

/// Transposes a 2D matrix (Vec of Vecs). Returns `None` if rows have different
/// lengths.
pub fn transpose<T: Clone>(matrix: &[Vec<T>]) -> Option<Vec<Vec<T>>> {
    if matrix.is_empty() {
        return Some(vec![]);
    }
    let cols = matrix[0].len();
    if matrix.iter().any(|row| row.len() != cols) {
        return None;
    }
    Some(
        (0..cols)
            .map(|col| matrix.iter().map(|row| row[col].clone()).collect())
            .collect(),
    )
}
