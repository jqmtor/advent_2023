use roaring::RoaringBitmap;

// Condition records
// Vec<RecordLine>

struct RecordLine {
    conditions: Vec<Condition>,
    damaged_groups: Vec<u32>,
}
enum Condition {
    Operational,
    Damaged,
    Unknown,
}

/**
 * Notes:
 * - The numbers at the end of the line allow us to infer what
 * possibilities are valid or not by evaluating the number of clusters
 * and the total number of damaged springs. This is not enough to filter
 * all the possibilities, but can serve as a quick elimination technique.
 */
use Condition::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_possibilities() {
        // ???.###
        let line = RecordLine {
            conditions: vec![
                Unknown,
                Unknown,
                Unknown,
                Operational,
                Damaged,
                Damaged,
                Damaged,
            ],
            damaged_groups: vec![1, 1, 3],
        };
        let poss_1 = 0b0010111;
        let poss_2 = 0b0100111;
        let poss_3 = 0b0110111;
        let poss_4 = 0b1000111;
        let poss_5 = 0b1010111;
        let poss_6 = 0b1100111;
        let poss_7 = 0b1110111;
    }

    #[test]
    fn test_bitmap() {
        let mut rb = RoaringBitmap::new();

        assert_eq!(rb.len(), 0);
        rb.insert(0x6);
        println!("{}", rb);
        assert_eq!(rb.len(), 2);

        let mut rb_2 = RoaringBitmap::new();
        rb_2.insert(0x1);
        assert_eq!(rb, rb_2);
    }
}
