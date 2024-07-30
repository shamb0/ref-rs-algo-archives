use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Block {
    pub balance: u64,
}

pub fn find_balance_changes(blocks: &[Block], start_index: usize, end_index: usize) -> Option<usize> {
    if start_index >= end_index || blocks.is_empty() {
        return None;
    }

    if start_index + 1 == end_index {
        if blocks[start_index].balance != blocks[end_index].balance {
            return Some(end_index);
        } else {
            return None;
        }
    }

    let mid_index = start_index + (end_index - start_index) / 2;

    if blocks[start_index].balance != blocks[mid_index].balance {
        find_balance_changes(blocks, start_index, mid_index)
    } else {
        find_balance_changes(blocks, mid_index, end_index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_blocks() {
        let mut blocks = vec![];
        assert_eq!(find_balance_changes(&blocks, 0, 0), None);
    }

    #[test]
    fn test_single_block() {
        let mut blocks = vec![Block { balance: 100 }];
        assert_eq!(find_balance_changes(&blocks, 0, 0), None);
    }

    #[test]
    fn test_no_change() {
        let mut blocks = vec![
            Block { balance: 100 },
            Block { balance: 100 },
            Block { balance: 100 },
        ];
        assert_eq!(find_balance_changes(&blocks, 0, 2), None);
    }

    #[test]
    fn test_change_in_middle() {
        let mut blocks = vec![
            Block { balance: 100 },
            Block { balance: 100 },
            Block { balance: 200 },
            Block { balance: 200 },
        ];
        assert_eq!(find_balance_changes(&blocks, 0, 3), Some(2));
    }

    #[test]
    fn test_change_at_end() {
        let mut blocks = vec![
            Block { balance: 100 },
            Block { balance: 100 },
            Block { balance: 200 },
        ];
        assert_eq!(find_balance_changes(&blocks, 0, 2), Some(2));
    }

    #[test]
    fn test_unsorted_blocks() {
        let mut blocks = vec![
            Block { balance: 200 },
            Block { balance: 100 },
            Block { balance: 300 },
            Block { balance: 100 },
        ];
        assert_eq!(find_balance_changes(&blocks, 0, 3), Some(1));
    }
}
