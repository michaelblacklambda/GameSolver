use std::collections::BTreeMap;

use itertools::Itertools;

// Performance could be significantly improved by knowing the length of the longest array
pub fn zip<S, T, U>(iter: S) -> S
where
    S: IntoIterator<Item = T> + FromIterator<T>,
    T: IntoIterator<Item = U> + FromIterator<U>,
{
    /* Associate element with it's indice, then flatten */
    let all_elements = iter
        .into_iter()
        .flat_map(|row| row.into_iter().enumerate())
        .collect_vec();

    /* Group by indice, sort, convert correct types */
    let transposed: S = all_elements
        .into_iter()
        .fold(
            BTreeMap::<usize, Vec<U>>::new(),
            |mut acc, (indice, elem)| {
                acc.entry(indice).or_insert_with(Vec::new).push(elem);
                acc
            },
        )
        .into_values()
        .into_iter()
        .map(|row| row.into_iter().collect::<T>())
        .collect();

    return transposed;
}
