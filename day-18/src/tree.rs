use std::fmt::Display;

#[derive(Clone)]
pub struct Tree<T, const N: usize> {
    inner: [Option<T>; N],
}

impl<T, const N: usize> Default for Tree<T, N>
where
    T: Default + Copy,
{
    fn default() -> Self {
        Self {
            inner: [Default::default(); N],
        }
    }
}

impl<T, const N: usize> From<[Option<T>; N]> for Tree<T, N> {
    fn from(array: [Option<T>; N]) -> Self {
        Self { inner: array }
    }
}

impl<T> Display for Tree<T, 32>
where
    T: Display,
{
    // TODO: bugged
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        let mut s = String::new();
        let mut i = 3i32;
        let mut idx = 0;

        while i >= 0 {
            let left_pad_size = 2usize.pow(i as u32) - 1;
            let inner_pad_size = 2usize.pow((i + 1) as u32) - 1;

            for _ in 0..left_pad_size {
                s.push(' ');
            }

            while idx < 2usize.pow(4 - i as u32) {
                match &self.inner[idx] {
                    Some(num) => s.push_str(&num.to_string()),
                    None => s.push('x'),
                }

                for _ in 0..inner_pad_size {
                    s.push(' ');
                }

                idx += 1;
            }

            s.push('\n');

            i -= 1;
        }

        write!(f, "{}", s)
    }
}

impl<T, const N: usize> Tree<T, N> {
    fn is_right_child(index: usize) -> bool {
        index % 2 == 0
    }

    fn is_left_child(index: usize) -> bool {
        index % 2 == 1
    }

    fn rightmost_child(tree: [Option<u8>; 32], index: usize) -> Option<u8> {
        todo!()
    }

    fn left(&self, index: usize) -> Option<usize> {
        let mut parent_idx = parent_index(index)?;

        loop {
            match self.inner[parent_idx] {
                Some(_) => return None, // huh?
                None => {
                    if Self::is_right_child(parent_idx) {
                        return Some(2); // ayyy
                    }
                }
            }

            parent_idx = parent_index(parent_idx)?;
        }
    }
}

pub const fn children_indices(index: usize) -> (usize, usize) {
    (index * 2, index * 2 + 1)
}

fn parent_index(index: usize) -> Option<usize> {
    if index <= 1 {
        return None;
    }

    let previous_power_of_2 = previous_power_of_2(index);
    let diff = index - previous_power_of_2;
    let half_diff = diff / 2;

    let previous_layer = previous_power_of_2 / 2;
    Some(previous_layer + half_diff)
}

// General math functions
fn previous_power_of_2(x: usize) -> usize {
    2usize.pow(log2(x) - 1)
}

fn log2(x: usize) -> u32 {
    usize::BITS - x.leading_zeros()
}
