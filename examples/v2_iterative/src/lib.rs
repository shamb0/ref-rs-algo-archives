#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Block {
    pub balance: u64,
}

pub fn find_balance_changes(blocks: &[Block], start_index: usize, end_index: usize) -> Option<usize> {
    if start_index >= end_index || blocks.is_empty() {
        return None;
    }

    let start_balance = blocks[start_index].balance;
    let mut left = start_index;
    let mut right = end_index;

    while left < right {
        let mid = left + (right - left) / 2;
        if blocks[mid].balance == start_balance {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    if left == end_index && blocks[left].balance == start_balance {
        None
    } else {
        Some(left)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_blocks() {
        let blocks = vec![];
        assert_eq!(find_balance_changes(&blocks, 0, 0), None);
    }

    #[test]
    fn test_single_block() {
        let blocks = vec![Block { balance: 100 }];
        assert_eq!(find_balance_changes(&blocks, 0, 0), None);
    }

    #[test]
    fn test_no_change() {
        let blocks = vec![
            Block { balance: 100 },
            Block { balance: 100 },
            Block { balance: 100 },
        ];
        assert_eq!(find_balance_changes(&blocks, 0, 2), None);
    }

    #[test]
    fn test_change_in_middle() {
        let blocks = vec![
            Block { balance: 100 },
            Block { balance: 100 },
            Block { balance: 200 },
            Block { balance: 200 },
        ];
        assert_eq!(find_balance_changes(&blocks, 0, 3), Some(2));
    }

    #[test]
    fn test_change_at_end() {
        let blocks = vec![
            Block { balance: 100 },
            Block { balance: 100 },
            Block { balance: 200 },
        ];
        assert_eq!(find_balance_changes(&blocks, 0, 2), Some(2));
    }
}
