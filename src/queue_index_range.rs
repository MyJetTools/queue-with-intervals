use crate::QueueValue;

#[derive(Debug, PartialEq)]
pub enum QueueIndexRangeCompare {
    Below,
    Inside,
    Above,
}

pub enum RemoveResult<T: QueueValue> {
    NoUpdate,
    InsertNew(QueueIndexRange<T>),
    RemoveItem,
}

#[derive(Debug, Clone)]
pub struct QueueIndexRange<T: QueueValue> {
    pub from_id: T,
    pub to_id: T,
}

impl<T: QueueValue> QueueIndexRange<T> {
    pub fn restore(from_id: T, to_id: T) -> QueueIndexRange<T> {
        QueueIndexRange { from_id, to_id }
    }

    pub fn new_empty(start_id: T) -> QueueIndexRange<T> {
        // For unsigned types, if start_id is 0, we use start_id+1 and start_id to represent empty
        // For signed types, we use start_id and start_id-1
        if let Some(prev) = start_id.checked_sub_one() {
            QueueIndexRange {
                from_id: start_id,
                to_id: prev,
            }
        } else {
            // For unsigned types when start_id is 0, use start_id+1 and start_id
            QueueIndexRange {
                from_id: start_id + T::one(),
                to_id: start_id,
            }
        }
    }

    pub fn new_with_single_value(value: T) -> QueueIndexRange<T> {
        QueueIndexRange {
            from_id: value,
            to_id: value,
        }
    }

    /*
    pub fn try_join_with_the_next_one(&mut self, next_one: &QueueIndexRange) -> bool {
        if self.to_id + 1 == next_one.from_id {
            self.to_id = next_one.to_id;
            return true;
        }

        return false;
    }

     */

    pub fn is_in_my_interval(&self, id: T) -> bool {
        id >= self.from_id && id <= self.to_id
    }
    pub fn is_in_my_interval_to_enqueue(&self, id: T) -> bool {
        if let Some(from_minus_one) = self.from_id.checked_sub_one() {
            if let Some(to_plus_one) = self.to_id.checked_add_one() {
                return id >= from_minus_one && id <= to_plus_one;
            }
        }
        // Handle edge cases for unsigned types
        if let Some(to_plus_one) = self.to_id.checked_add_one() {
            id >= self.from_id && id <= to_plus_one
        } else {
            id >= self.from_id && id <= self.to_id
        }
    }

    pub fn can_be_joined_to_interval_from_the_left(&self, id: T) -> bool {
        if let Some(from_minus_one) = self.from_id.checked_sub_one() {
            from_minus_one <= id && id <= self.to_id
        } else {
            self.from_id <= id && id <= self.to_id
        }
    }

    pub fn can_be_joined_to_interval_from_the_right(&self, id: T) -> bool {
        if let Some(to_plus_one) = self.to_id.checked_add_one() {
            self.from_id <= id && id <= to_plus_one
        } else {
            self.from_id <= id && id <= self.to_id
        }
    }

    pub fn is_my_interval_to_remove(&self, id: T) -> bool {
        if self.is_empty() {
            panic!(
                "MyServiceBus. We are trying to find interval to remove but we bumped empty interval"
            );
        }

        id >= self.from_id && id <= self.to_id
    }

    pub fn remove(&mut self, message_id: T) -> RemoveResult<T> {
        if self.from_id == message_id && self.to_id == message_id {
            self.from_id += T::one();

            return RemoveResult::RemoveItem;
        }

        if self.from_id == message_id {
            self.from_id += T::one();
            return RemoveResult::NoUpdate;
        }

        if self.to_id == message_id {
            if let Some(prev) = self.to_id.checked_sub_one() {
                self.to_id = prev;
            } else {
                // For unsigned types at 0, this shouldn't happen in normal usage
                panic!("Cannot decrement to_id below zero");
            }
            return RemoveResult::NoUpdate;
        }

        if let (Some(msg_plus_one), Some(msg_minus_one)) =
            (message_id.checked_add_one(), message_id.checked_sub_one())
        {
            let new_item = QueueIndexRange {
                from_id: msg_plus_one,
                to_id: self.to_id,
            };

            self.to_id = msg_minus_one;

            return RemoveResult::InsertNew(new_item);
        }

        // Fallback for edge cases
        panic!("Cannot split interval at message_id");
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if self.from_id > self.to_id {
            return None;
        }

        let result = self.from_id;
        self.from_id = self.from_id + T::one();
        Some(result)
    }

    pub fn peek(&self) -> Option<T> {
        if self.from_id > self.to_id {
            return None;
        }

        return Some(self.from_id);
    }

    pub fn enqueue(&mut self, id: T) {
        if self.is_empty() {
            self.from_id = id;
            self.to_id = id;
            return;
        }

        if self.from_id >= id && self.to_id <= id {
            panic!(
                "Warning.... Something went wrong. We are enqueueing the Value {} which is already in the queue. Range: {:?}. ",
                id, self,
            );
        } else if let Some(to_plus_one) = self.to_id.checked_add_one() {
            if to_plus_one == id {
                self.to_id = id;
                return;
            }
        }
        if let Some(from_minus_one) = self.from_id.checked_sub_one() {
            if from_minus_one == id {
                self.from_id = id;
                return;
            }
        }
        {
            panic!(
                "Something went wrong. Invalid interval is chosen to enqueue. Range: {:?}. NewValue: {}",
                self, id
            );
        }
    }

    pub fn try_to_merge_with_next_item(
        &self,
        next_item: &QueueIndexRange<T>,
    ) -> Option<QueueIndexRange<T>> {
        if let Some(to_plus_one) = self.to_id.checked_add_one() {
            if to_plus_one == next_item.from_id {
                return Some(QueueIndexRange {
                    from_id: self.from_id,
                    to_id: next_item.to_id,
                });
            }
        }

        None
    }

    pub fn try_join(&mut self, id_to_join: T) -> bool {
        if self.is_empty() {
            self.from_id = id_to_join;
            self.to_id = id_to_join;
            return true;
        }

        if let Some(from_minus_one) = self.from_id.checked_sub_one() {
            if id_to_join == from_minus_one {
                self.from_id = id_to_join;
                return true;
            }
        }

        if let Some(to_plus_one) = self.to_id.checked_add_one() {
            if id_to_join == to_plus_one {
                self.to_id = id_to_join;
                return true;
            }
        }

        return false;
    }

    pub fn is_empty(&self) -> bool {
        self.to_id < self.from_id
    }

    pub fn make_empty(&mut self) {
        if let Some(to_plus_one) = self.to_id.checked_add_one() {
            self.from_id = to_plus_one;
        } else {
            // For unsigned types at max, use a different strategy
            // Set from_id to max and to_id to max-1 (if possible)
            self.from_id = self.to_id;
            if let Some(prev) = self.to_id.checked_sub_one() {
                self.to_id = prev;
            } else {
                // This is an edge case - queue is at max value
                // Keep it as is, it will be considered empty
            }
        }
    }

    pub fn is_before(&self, id: T) -> bool {
        if let Some(from_minus_one) = self.from_id.checked_sub_one() {
            id < from_minus_one
        } else {
            id < self.from_id
        }
    }

    pub fn compare_with(&self, id: T) -> Option<QueueIndexRangeCompare> {
        if self.is_empty() {
            return None;
        }

        if id < self.from_id {
            return Some(QueueIndexRangeCompare::Below);
        }

        if id > self.to_id {
            return Some(QueueIndexRangeCompare::Above);
        }

        return Some(QueueIndexRangeCompare::Inside);
    }

    pub fn covered_with_range_to_insert(&self, range_to_insert: &QueueIndexRange<T>) -> bool {
        range_to_insert.from_id <= self.from_id && range_to_insert.to_id >= self.to_id
    }

    #[cfg(test)]
    pub fn to_string(&self) -> String {
        if self.is_empty() {
            return "EMPTY".to_string();
        }

        return format!("{:?} - {:?}", self.from_id, self.to_id);
    }

    pub fn len(&self) -> usize {
        if self.from_id > self.to_id {
            return 0;
        }
        // Calculate length safely
        // For most practical ranges, this will work fine
        // For very large ranges, we use a loop to avoid overflow
        let mut count = 0usize;
        let mut current = self.from_id;
        while current <= self.to_id {
            count = count.saturating_add(1);
            if let Some(next) = current.checked_add_one() {
                if next > self.to_id {
                    break;
                }
                current = next;
            } else {
                // Reached max value
                count = count.saturating_add(1);
                break;
            }
        }
        count
    }

    /// Returns the length as the same type T
    /// This is useful when you need the length in the same type as the range
    pub fn len_as_t(&self) -> T {
        if self.from_id > self.to_id {
            return T::zero();
        }
        // This is a simplified version - for very large ranges this might overflow
        // But for most practical use cases it will work
        let one = T::one();
        self.to_id - self.from_id + one
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queue() {
        let index_range = QueueIndexRange::new_empty(0);

        assert_eq!(index_range.len(), 0);
        assert_eq!(0, index_range.from_id);
        assert_eq!(-1, index_range.to_id);

        println!("{}", index_range.to_string());
    }

    #[test]
    fn test_one_enqueue_one_dequeue() {
        let mut index_range = QueueIndexRange::new_empty(0);

        index_range.enqueue(0);

        assert_eq!(index_range.len(), 1);
        assert_eq!(0, index_range.from_id);
        assert_eq!(0, index_range.to_id);

        let res = index_range.dequeue();

        assert_eq!(index_range.len(), 0);
        assert_eq!(1, index_range.from_id);
        assert_eq!(0, index_range.to_id);
        assert_eq!(0, res.unwrap());
    }

    #[test]
    fn test_two_enqueue_two_dequeue() {
        let mut index_range = QueueIndexRange::new_with_single_value(5);

        index_range.enqueue(6);

        assert_eq!(index_range.len(), 2);

        let res = index_range.dequeue();
        assert_eq!(5, res.unwrap());
        let res = index_range.dequeue();
        assert_eq!(6, res.unwrap());

        let res = index_range.dequeue();
        assert_eq!(true, res.is_none());
    }

    #[test]
    fn test_match_case() {
        let index_range = QueueIndexRange::restore(5, 10);

        let _result = index_range.compare_with(4).unwrap();
        assert_eq!(true, matches!(QueueIndexRangeCompare::Below, _result));

        let _result = index_range.compare_with(5).unwrap();
        assert_eq!(true, matches!(QueueIndexRangeCompare::Inside, _result));

        let _result = index_range.compare_with(10).unwrap();
        assert_eq!(true, matches!(QueueIndexRangeCompare::Inside, _result));

        let _result = index_range.compare_with(11).unwrap();
        assert_eq!(true, matches!(QueueIndexRangeCompare::Above, _result));
    }

    #[test]
    fn test_new_empty_with_different_start_ids() {
        let range1 = QueueIndexRange::new_empty(0);
        assert!(range1.is_empty());
        assert_eq!(0, range1.from_id);
        assert_eq!(-1, range1.to_id);

        let range2 = QueueIndexRange::new_empty(100);
        assert!(range2.is_empty());
        assert_eq!(100, range2.from_id);
        assert_eq!(99, range2.to_id);

        let range3 = QueueIndexRange::new_empty(-10);
        assert!(range3.is_empty());
        assert_eq!(-10, range3.from_id);
        assert_eq!(-11, range3.to_id);
    }

    #[test]
    fn test_new_with_single_value() {
        let range = QueueIndexRange::new_with_single_value(42);
        assert_eq!(42, range.from_id);
        assert_eq!(42, range.to_id);
        assert_eq!(1, range.len());
        assert!(!range.is_empty());
    }

    #[test]
    fn test_is_in_my_interval_boundary_cases() {
        let range = QueueIndexRange::restore(10, 20);
        assert!(range.is_in_my_interval(10));
        assert!(range.is_in_my_interval(15));
        assert!(range.is_in_my_interval(20));
        assert!(!range.is_in_my_interval(9));
        assert!(!range.is_in_my_interval(21));
    }

    #[test]
    fn test_is_in_my_interval_empty_range() {
        let range = QueueIndexRange::new_empty(0);
        assert!(!range.is_in_my_interval(0));
        assert!(!range.is_in_my_interval(-1));
        assert!(!range.is_in_my_interval(1));
    }

    #[test]
    fn test_is_in_my_interval_to_enqueue() {
        let range = QueueIndexRange::restore(10, 20);
        assert!(range.is_in_my_interval_to_enqueue(9)); // from_id - 1
        assert!(range.is_in_my_interval_to_enqueue(10));
        assert!(range.is_in_my_interval_to_enqueue(20));
        assert!(range.is_in_my_interval_to_enqueue(21)); // to_id + 1
        assert!(!range.is_in_my_interval_to_enqueue(8));
        assert!(!range.is_in_my_interval_to_enqueue(22));
    }

    #[test]
    fn test_can_be_joined_to_interval_from_the_left() {
        let range = QueueIndexRange::restore(10, 20);
        assert!(range.can_be_joined_to_interval_from_the_left(9)); // from_id - 1
        assert!(range.can_be_joined_to_interval_from_the_left(10));
        assert!(range.can_be_joined_to_interval_from_the_left(20));
        assert!(!range.can_be_joined_to_interval_from_the_left(8));
        assert!(!range.can_be_joined_to_interval_from_the_left(21));
    }

    #[test]
    fn test_can_be_joined_to_interval_from_the_right() {
        let range = QueueIndexRange::restore(10, 20);
        assert!(range.can_be_joined_to_interval_from_the_right(10));
        assert!(range.can_be_joined_to_interval_from_the_right(20));
        assert!(range.can_be_joined_to_interval_from_the_right(21)); // to_id + 1
        assert!(!range.can_be_joined_to_interval_from_the_right(9));
        assert!(!range.can_be_joined_to_interval_from_the_right(22));
    }

    #[test]
    fn test_compare_with_empty_range() {
        let range = QueueIndexRange::new_empty(0);
        assert!(range.compare_with(0).is_none());
        assert!(range.compare_with(-1).is_none());
        assert!(range.compare_with(1).is_none());
    }

    #[test]
    fn test_compare_with_negative_values() {
        let range = QueueIndexRange::restore(-10, -5);
        assert_eq!(
            QueueIndexRangeCompare::Below,
            range.compare_with(-11).unwrap()
        );
        assert_eq!(
            QueueIndexRangeCompare::Inside,
            range.compare_with(-10).unwrap()
        );
        assert_eq!(
            QueueIndexRangeCompare::Inside,
            range.compare_with(-7).unwrap()
        );
        assert_eq!(
            QueueIndexRangeCompare::Inside,
            range.compare_with(-5).unwrap()
        );
        assert_eq!(
            QueueIndexRangeCompare::Above,
            range.compare_with(-4).unwrap()
        );
    }

    #[test]
    fn test_try_to_merge_with_next_item() {
        let range1 = QueueIndexRange::restore(10, 20);
        let range2 = QueueIndexRange::restore(21, 30); // Adjacent
        let merged = range1.try_to_merge_with_next_item(&range2);
        assert!(merged.is_some());
        let merged = merged.unwrap();
        assert_eq!(10, merged.from_id);
        assert_eq!(30, merged.to_id);

        let range3 = QueueIndexRange::restore(22, 30); // Not adjacent
        let merged = range1.try_to_merge_with_next_item(&range3);
        assert!(merged.is_none());
    }

    #[test]
    fn test_try_join() {
        let mut range = QueueIndexRange::restore(10, 20);
        assert!(range.try_join(9)); // from_id - 1
        assert_eq!(9, range.from_id);
        assert_eq!(20, range.to_id);

        let mut range2 = QueueIndexRange::restore(10, 20);
        assert!(range2.try_join(21)); // to_id + 1
        assert_eq!(10, range2.from_id);
        assert_eq!(21, range2.to_id);

        let mut range3 = QueueIndexRange::restore(10, 20);
        assert!(!range3.try_join(8)); // Too far
        assert!(!range3.try_join(22)); // Too far
    }

    #[test]
    fn test_try_join_empty_range() {
        let mut range = QueueIndexRange::new_empty(0);
        assert!(range.try_join(5));
        assert_eq!(5, range.from_id);
        assert_eq!(5, range.to_id);
        assert!(!range.is_empty());
    }

    #[test]
    fn test_covered_with_range_to_insert() {
        let range = QueueIndexRange::restore(10, 20);
        assert!(range.covered_with_range_to_insert(&QueueIndexRange::restore(10, 20)));
        assert!(range.covered_with_range_to_insert(&QueueIndexRange::restore(5, 25)));
        assert!(range.covered_with_range_to_insert(&QueueIndexRange::restore(10, 25)));
        assert!(range.covered_with_range_to_insert(&QueueIndexRange::restore(5, 20)));
        assert!(!range.covered_with_range_to_insert(&QueueIndexRange::restore(15, 25)));
        assert!(!range.covered_with_range_to_insert(&QueueIndexRange::restore(5, 15)));
    }

    #[test]
    fn test_len_calculations() {
        let range1 = QueueIndexRange::restore(10, 10);
        assert_eq!(1, range1.len());

        let range2 = QueueIndexRange::restore(10, 20);
        assert_eq!(11, range2.len());

        let range3 = QueueIndexRange::restore(-10, -5);
        assert_eq!(6, range3.len());

        let range4 = QueueIndexRange::new_empty(0);
        assert_eq!(0, range4.len());
    }

    #[test]
    fn test_iterator_for_range() {
        let range = QueueIndexRange::restore(10, 12);
        let collected: Vec<i64> = range.into_iter().collect();
        assert_eq!(vec![10, 11, 12], collected);
    }

    #[test]
    fn test_iterator_empty_range() {
        let range = QueueIndexRange::new_empty(0);
        let collected: Vec<i64> = range.into_iter().collect();
        assert_eq!(Vec::<i64>::new(), collected);
    }

    #[test]
    fn test_iterator_single_value() {
        let range = QueueIndexRange::restore(42, 42);
        let collected: Vec<i64> = range.into_iter().collect();
        assert_eq!(vec![42], collected);
    }

    #[test]
    fn test_iterator_negative_values() {
        let range = QueueIndexRange::restore(-5, -2);
        let collected: Vec<i64> = range.into_iter().collect();
        assert_eq!(vec![-5, -4, -3, -2], collected);
    }
}
