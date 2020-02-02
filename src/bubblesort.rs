use std::fmt::{Display, Debug};
use crate::list::{List, ListError};

pub fn bubblesort<L, T>(list: &mut L, cmp: fn(a: &T, b: &T) -> bool) ->
    Result<(), ListError> where L: List<T>, T: Eq + Clone + Display + Debug {
    if list.length()? <= 1 {
        return Ok(());
    }

    let n: usize = list.length()?;

    for i in 0..n {
        for j in (i+1..n).rev() {
            if cmp(list.get(j)?, list.get(i)?) {
                list.swap(i, j)?;
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::arraylist::ArrayList;

    fn cmp_leq<T: Eq + Ord>(a: &T, b: &T) -> bool {
        a <= b
    }

    #[test]
    pub fn test_bubblesort_normal1() -> Result<(), ListError> {
        let mut actual_list: ArrayList<u64> = ArrayList::new();

        actual_list.append(33)?;
        actual_list.append(12)?;
        actual_list.append(0)?;
        actual_list.append(1)?;
        actual_list.append(4)?;

        let mut expected_list: ArrayList<u64> = ArrayList::new();

        expected_list.append(0)?;
        expected_list.append(1)?;
        expected_list.append(4)?;
        expected_list.append(12)?;
        expected_list.append(33)?;

        let actual_res: Result<(), ListError> = bubblesort(&mut actual_list,
                                                           cmp_leq);
        let expected_res: Result<(), ListError> = Ok(());

        assert_eq!(actual_list, expected_list);
        assert_eq!(actual_res, expected_res);

        Ok(())
    }

    #[test]
    pub fn test_bubblesort_normal_empty() -> Result<(), ListError> {
        let mut actual_list: ArrayList<u64> = ArrayList::new();
        let expected_list: ArrayList<u64> = ArrayList::new();

        let actual_res: Result<(), ListError> = bubblesort(&mut actual_list,
                                                           cmp_leq);
        let expected_res: Result<(), ListError> = Ok(());

        assert_eq!(actual_list, expected_list);
        assert_eq!(actual_res, expected_res);

        Ok(())
    }

    #[test]
    pub fn test_bubblesort_normal_single() -> Result<(), ListError> {
        let mut actual_list: ArrayList<u64> = ArrayList::new();
        actual_list.append(1)?;

        let mut expected_list: ArrayList<u64> = ArrayList::new();
        expected_list.append(1)?;

        let actual_res: Result<(), ListError> = bubblesort(&mut actual_list,
                                                           cmp_leq);
        let expected_res: Result<(), ListError> = Ok(());

        assert_eq!(actual_list, expected_list);
        assert_eq!(actual_res, expected_res);

        Ok(())
    }

    #[test]
    pub fn test_bubblesort_normal_sorted_two_elems() -> Result<(), ListError> {
        let mut actual_list: ArrayList<u64> = ArrayList::new();
        actual_list.append(1)?;
        actual_list.append(12)?;

        let mut expected_list: ArrayList<u64> = ArrayList::new();
        expected_list.append(1)?;
        expected_list.append(12)?;

        let actual_res: Result<(), ListError> = bubblesort(&mut actual_list,
                                                           cmp_leq);
        let expected_res: Result<(), ListError> = Ok(());

        assert_eq!(actual_list, expected_list);
        assert_eq!(actual_res, expected_res);

        Ok(())
    }

}

