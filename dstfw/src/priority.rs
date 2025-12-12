use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Task {
    pub task_name: String,
    pub priority: usize,
}

// Implement the trait so that the queue becomes a min-heap instead of max-heap
impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering {
        // Flip the order on priority so that the lowest number goes first
        other.priority.cmp(&self.priority)
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DetailedTask {
    pub task_name: String,
    pub priority: usize,
    pub only_during_day: bool,
}

// Implement the trait so that the queue becomes a min-heap instead of max-heap
impl Ord for DetailedTask {
    fn cmp(&self, other: &Self) -> Ordering {
        // Flip the order on priority so that the lowest number goes first
        // The tiebreaker is if the current task can only be done durijng
        // the day, then it should go first
        other
            .priority
            .cmp(&self.priority)
            .then_with(|| self.only_during_day.cmp(&other.only_during_day))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for DetailedTask {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_heap_with_detailed_tasks() {
        let mut heap: BinaryHeap<DetailedTask> = BinaryHeap::new();
        assert_eq!(heap.peek(), None);

        // Add some values that come in
        heap.push(DetailedTask {
            task_name: "Start laundry".to_string(),
            priority: 3,
            only_during_day: false,
        });
        heap.push(DetailedTask {
            task_name: "Sweep stoop".to_string(),
            priority: 5,
            only_during_day: true,
        });
        heap.push(DetailedTask {
            task_name: "Clean bathroom".to_string(),
            priority: 5,
            only_during_day: false,
        });

        // Check that the first item has the LOWEST priority number
        assert_eq!(heap.peek().unwrap().task_name, "Start laundry");
        assert_eq!(heap.len(), 3);

        // Now check that, even though they came in with a random order,
        // that the task with the lowest priority always comes out first
        assert_eq!(heap.pop().unwrap().task_name, "Start laundry");
        // Stoop should be swept before cleaning bathroom assumning
        // it is daytime since it can only really be done during the day
        assert_eq!(heap.pop().unwrap().task_name, "Sweep stoop");
        assert_eq!(heap.pop().unwrap().task_name, "Clean bathroom");
        assert_eq!(heap.pop(), None);
    }

    #[test]
    fn test_binary_heap_with_tasks() {
        let mut heap: BinaryHeap<Task> = BinaryHeap::new();
        assert_eq!(heap.peek(), None);

        // Add some values that come in
        heap.push(Task {
            task_name: "Organize spices".to_string(),
            priority: 33,
        });
        heap.push(Task {
            task_name: "Start laundry".to_string(),
            priority: 3,
        });
        heap.push(Task {
            task_name: "Clean bathroom".to_string(),
            priority: 5,
        });
        // heap.push(5);
        // heap.push(2);

        // Check that the first item has the LOWEST priority number
        assert_eq!(heap.peek().unwrap().task_name, "Start laundry");
        assert_eq!(heap.len(), 3);

        // Now check that, even though they came in with a random order,
        // that the task with the lowest priority always comes out first
        assert_eq!(heap.pop().unwrap().task_name, "Start laundry");
        assert_eq!(heap.pop().unwrap().task_name, "Clean bathroom");
        assert_eq!(heap.pop().unwrap().task_name, "Organize spices");
        // // assert_eq!(heap.pop(), Some(2));
        // // assert_eq!(heap.pop(), Some(1));
        assert_eq!(heap.pop(), None);
    }

    #[test]
    fn test_binary_heap_with_integers() {
        let mut heap: BinaryHeap<usize> = BinaryHeap::new();
        assert_eq!(heap.peek(), None);

        // Add some values that come in
        heap.push(1);
        heap.push(5);
        heap.push(2);

        // Check that the first item is the greatest and that peeking
        // does not remove any items
        assert_eq!(heap.peek(), Some(&5));
        assert_eq!(heap.len(), 3);

        // Now check that, even though they came in with a random order,
        // that the greatest one always comes out first
        assert_eq!(heap.pop(), Some(5));
        assert_eq!(heap.pop(), Some(2));
        assert_eq!(heap.pop(), Some(1));
        assert_eq!(heap.pop(), None);
    }
}
