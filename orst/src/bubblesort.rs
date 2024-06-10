use super::Sorter;

pub struct BubbleSort;

impl<T> Sorter<T> for BubbleSort {
    fn sort(&self, slice: &mut [T])
    where T: Ord,
    {
        let mut swapped = true;
        while swapped {
            swapped = false;
            for i in 1..slice.len() {
                if slice[i - 1] > slice[i] {
                    slice.swap(i - 1, i);
                    swapped = true;  
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bubblesort_works() {
        let mut things = vec![3, 2, 4, 1];
        BubbleSort.sort(&mut things);
        assert_eq!(things, [1, 2, 3, 4]);
    }
}
