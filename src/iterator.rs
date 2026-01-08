use super::{QueueIndexRange, QueueWithIntervals};
use crate::QueueValue;

pub struct QueueWithIntervalsIterator<T: QueueValue> {
    intervals: QueueWithIntervals<T>,
}

impl<T: QueueValue> QueueWithIntervalsIterator<T> {
    pub fn new(intervals: QueueWithIntervals<T>) -> Self {
        Self { intervals }
    }
}

impl<T: QueueValue> Iterator for QueueWithIntervalsIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        return self.intervals.dequeue();
    }
}

pub struct QueueIndexRangeIterator<T: QueueValue> {
    from_id: T,
    to_id: T,
}

impl<T: QueueValue> QueueIndexRangeIterator<T> {
    pub fn new(range: &QueueIndexRange<T>) -> Self {
        Self {
            from_id: range.from_id,
            to_id: range.to_id,
        }
    }
}
impl<T: QueueValue> Iterator for QueueIndexRangeIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.from_id <= self.to_id {
            let result = self.from_id;
            if let Some(next) = self.from_id.checked_add_one() {
                self.from_id = next;
            } else {
                // Reached max value
                self.from_id = self.to_id;
                if let Some(_) = self.to_id.checked_add_one() {
                    // This shouldn't happen, but handle it
                    return Some(result);
                }
            }
            return Some(result);
        }

        return None;
    }
}

impl<T: QueueValue> IntoIterator for QueueIndexRange<T> {
    type Item = T;
    type IntoIter = QueueIndexRangeIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        QueueIndexRangeIterator::new(&self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iter() {
        let mut queue = QueueWithIntervals::new();

        queue.enqueue(5);
        queue.enqueue(6);

        let mut result: Vec<i64> = Vec::new();
        result.extend(queue);

        assert_eq!(2, result.len());
        assert_eq!(5, result[0]);
        assert_eq!(6, result[1]);
    }

    #[test]
    fn test_iterator_empty_queue() {
        let queue = QueueWithIntervals::new();
        let collected: Vec<i64> = queue.iter().collect();
        assert_eq!(Vec::<i64>::new(), collected);
    }

    #[test]
    fn test_iterator_exhausts() {
        let mut queue = QueueWithIntervals::new();
        queue.enqueue(1);
        queue.enqueue(2);

        let mut iter = queue.iter();
        assert_eq!(Some(1), iter.next());
        assert_eq!(Some(2), iter.next());
        assert_eq!(None, iter.next());
        assert_eq!(None, iter.next()); // Multiple None calls
    }

    #[test]
    fn test_iterator_multiple_intervals() {
        let mut queue = QueueWithIntervals::new();
        queue.enqueue_range(QueueIndexRange::restore(1, 3));
        queue.enqueue_range(QueueIndexRange::restore(10, 12));

        let collected: Vec<i64> = queue.iter().collect();
        assert_eq!(vec![1, 2, 3, 10, 11, 12], collected);
    }

    #[test]
    fn test_queue_index_range_iterator() {
        let range = QueueIndexRange::restore(5, 7);
        let mut iter = QueueIndexRangeIterator::new(&range);
        assert_eq!(Some(5), iter.next());
        assert_eq!(Some(6), iter.next());
        assert_eq!(Some(7), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn test_queue_index_range_iterator_empty() {
        let range = QueueIndexRange::new_empty(0);
        let mut iter = QueueIndexRangeIterator::new(&range);
        assert_eq!(None, iter.next());
    }
}
