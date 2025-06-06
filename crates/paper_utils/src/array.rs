pub fn flatten<I, T>(iter: I) -> Vec<T>
where
    I: IntoIterator<Item = Vec<T>>,
{
    iter.into_iter().flatten().collect()
}

pub fn flatten_array<I, T, const N: usize>(iter: I) -> [T; N]
where
    T: Default + Copy,
    I: IntoIterator<Item = [T; N]>,
{
    let mut result = [Default::default(); N];
    let mut index = 0;

    for item in iter {
        for &value in &item {
            if index < N {
                result[index] = value;
                index += 1;
            } else {
                break;
            }
        }
    }

    result
}
