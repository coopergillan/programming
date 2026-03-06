use std::cmp::Ordering;
use std::collections::BinaryHeap;

// Create an enum for four business applications
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Application {
    Compliance,
    Ledger,
    TradeFeed,
    TradingPlatform,
}

// Lay out different tasks that could arise during the workday,
// some of which are specific to one of the above applications
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TaskCategory {
    DeployApproval(Application),
    DeployAuthorization(Application),
    Email,
    ProjectStatusUpdate,
    CodeReview(Application),
    CodeReviewTopPriority(Application),
    DirectMessage,
    MessagingMention,
    MessagingTeamChannel,
    Messages,
}

impl TaskCategory {
    // fn application_priority(app: &Application) -> usize {
    //     match app {
    //         Application::TradingPlatform => 1,
    //         Application::TradeFeed => 2,
    //         Application::Ledger => 3,
    //         Application::Compliance => 4,
    //     }
    // }

    fn priority(&self) -> usize {
        match self {
            // TradingPlatform deploy authorization: most important
            TaskCategory::DeployAuthorization(Application::TradingPlatform) => 1,
            // TradingPlatform deploy approval: next
            TaskCategory::DeployApproval(Application::TradingPlatform) => 2,
            // TradingPlatform high priority code review: next
            TaskCategory::CodeReviewTopPriority(Application::TradingPlatform) => 3,

            // Non-TradingPlatform deploy authorizations
            // TradeFeed deploy authorization: after all TradingPlatform
            TaskCategory::DeployAuthorization(Application::TradeFeed) => 4,
            // Ledger deploy authorization: after TradeFeed
            TaskCategory::DeployAuthorization(Application::Ledger) => 5,
            // Compliance deploy authorization: after Ledger
            TaskCategory::DeployAuthorization(Application::Compliance) => 6,

            // Deploy approvals
            // TradeFeed deploy approval: after all other deploy auth
            TaskCategory::DeployApproval(Application::TradeFeed) => 7,
            // Ledger deploy approval: after Ledger deploy auth
            TaskCategory::DeployApproval(Application::Ledger) => 8,
            // Compliance deploy approval: after Compliance deploy auth
            TaskCategory::DeployApproval(Application::Compliance) => 9,

            // Direct messages come after all deploy-related tasks
            TaskCategory::DirectMessage => 10,
            // Mention in mnessaging application: after direct messages
            TaskCategory::MessagingMention => 11,

            // TradeFeed high priority code review comes after mentions in messaging app
            TaskCategory::CodeReviewTopPriority(Application::TradeFeed) => 12,
            // Ledger high priority code review: after TradeFeed high priority code review
            TaskCategory::CodeReviewTopPriority(Application::Ledger) => 13,
            // Compliance high priority code review: after Ledger high priority code review
            TaskCategory::CodeReviewTopPriority(Application::Compliance) => 14,

            // Check messaging team channel(s): after all top priority code reviews
            TaskCategory::MessagingTeamChannel => 15,

            // TradingPlatform regular code review: after all high priority code reviews and checking team channel(s) in messaging app
            TaskCategory::CodeReview(Application::TradingPlatform) => 16,
            // TradeFeed regular code review: after TradingPlatform regular code review
            TaskCategory::CodeReview(Application::TradeFeed) => 17,
            // Ledger regular code review: after TradeFeed regular code review
            TaskCategory::CodeReview(Application::Ledger) => 18,
            // Compliance regular code review: after Ledger regular code review
            TaskCategory::CodeReview(Application::Compliance) => 19,

            // All other messages in messaging app comes after all code reviews
            TaskCategory::Messages => 20,
            // ProjectStatusUpdate: after catching up on messages
            TaskCategory::ProjectStatusUpdate => 21,
            // Email: lowest priority - after updating ProjectStatusUpdate boards
            TaskCategory::Email => 22,
        }
    }
}

// Implement the trait so that the queue becomes a min-heap instead of max-heap
// by flipping the order on priority so that the lowest number goes first
impl Ord for TaskCategory {
    fn cmp(&self, other: &Self) -> Ordering {
        other.priority().cmp(&self.priority())
    }
}

impl PartialOrd for TaskCategory {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TaskWithDuration {
    pub task_name: String,
    pub duration_in_seconds: usize,
    // Say whether or not there are downstream tasks
    pub has_dependents: bool,
}

// Implement the trait so that tasks with the longest duration are prioritized first
// Use whether or not there are dependents as a tiebreaker - that is, if two tasks
// have the same duration, the one with downstream dependents should becompleted first
impl Ord for TaskWithDuration {
    fn cmp(&self, other: &Self) -> Ordering {
        // Set the ordering by the duration in seconds
        self.duration_in_seconds
            .cmp(&other.duration_in_seconds)
            .then_with(|| self.has_dependents.cmp(&other.has_dependents))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for TaskWithDuration {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heap_with_priority_by_duration() {
        let mut heap: BinaryHeap<TaskWithDuration> = BinaryHeap::new();
        assert_eq!(heap.peek(), None);

        // Add some tasks with duration
        heap.push(TaskWithDuration {
            task_name: "Wash dishes".to_string(),
            duration_in_seconds: 300,
            has_dependents: false,
        });
        heap.push(TaskWithDuration {
            task_name: "Dry clothes".to_string(),
            duration_in_seconds: 3000,
            has_dependents: false, // Actually, they need to be folded - maybe a DAG later
        });
        heap.push(TaskWithDuration {
            task_name: "Clean bathroom".to_string(),
            duration_in_seconds: 600,
            has_dependents: false,
        });
        heap.push(TaskWithDuration {
            task_name: "Wash clothes".to_string(),
            duration_in_seconds: 3000,
            has_dependents: true, // Clothes must be dried after
        });

        // Assert that the longest task should be completed first
        assert_eq!(heap.pop().unwrap().task_name, "Wash clothes".to_string());
        assert_eq!(heap.pop().unwrap().task_name, "Dry clothes".to_string());
        assert_eq!(heap.pop().unwrap().task_name, "Clean bathroom".to_string());
        assert_eq!(heap.pop().unwrap().task_name, "Wash dishes".to_string());
        assert_eq!(heap.pop(), None);
    }

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

    #[test]
    fn test_task_category_min_heap() {
        let mut heap: BinaryHeap<TaskCategory> = BinaryHeap::new();
        assert_eq!(heap.peek(), None);

        // Add some values that come in
        // Email, ProjectStatusUpdate, and messages catch-up are nearly always available
        heap.push(TaskCategory::Email);
        heap.push(TaskCategory::Messages);
        heap.push(TaskCategory::ProjectStatusUpdate);

        // Started checking messages before anything else happened
        assert_eq!(heap.pop().unwrap(), TaskCategory::Messages);

        // Received a DM while going through messages
        heap.push(TaskCategory::DirectMessage);
        // Confirm that rises to the top (for now)
        assert_eq!(heap.peek(), Some(&TaskCategory::DirectMessage));

        // Then a whole bunch of stuff happens while finishing catching up on messages
        // Get asked to authorize a TradingPlatform deploy before checking the DM that came through
        heap.push(TaskCategory::DeployAuthorization(Application::TradingPlatform));

        // Get pinged for an Compliance deploy approval
        heap.push(TaskCategory::DeployApproval(Application::Compliance));
        // Get pinged for an TradeFeed deploy approval
        heap.push(TaskCategory::DeployApproval(Application::TradeFeed));

        // Finally catch breath and check to see that the TradingPlatform deploy auth is next
        assert_eq!(heap.pop().unwrap(), TaskCategory::DeployAuthorization(Application::TradingPlatform));

        // Next up should be approving the TradeFeed deploy
        assert_eq!(heap.pop().unwrap(), TaskCategory::DeployApproval(Application::TradeFeed));

        // There are then still four tasks waiting, with Compliance's deploy approval up next
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.peek(), Some(&TaskCategory::DeployApproval(Application::Compliance)));

        // Before even able to start that approval some other things happen
        // Someone posts in the team channel
        heap.push(TaskCategory::MessagingTeamChannel);
        // Compliance deploy approval still up next
        assert_eq!(heap.peek(), Some(&TaskCategory::DeployApproval(Application::Compliance)));

        // Some regular ol' TradeFeed code is ready to be reviewed
        heap.push(TaskCategory::CodeReview(Application::TradeFeed));
        // Some other messages are received
        heap.push(TaskCategory::Messages);
        // A Ledger deploy authorization is requested
        heap.push(TaskCategory::DeployAuthorization(Application::Ledger));
        // Some high-priority TradingPlatform code needs to be reviewed
        heap.push(TaskCategory::CodeReviewTopPriority(Application::TradingPlatform));
        // Some regular ol' Compliance code is ready for review
        heap.push(TaskCategory::CodeReview(Application::Compliance));
        // Someone mentions you in messaging app
        heap.push(TaskCategory::MessagingMention);

        // Wow, okay, so there are 11 tasks in the queue
        assert_eq!(heap.len(), 11);

        // Assert that the order is based on prioirty and not when they were inserted
        // TradingPlatform high priority code review is the top priority
        assert_eq!(heap.pop().unwrap(), TaskCategory::CodeReviewTopPriority(Application::TradingPlatform));
        // Ledger deploy auth is up next
        assert_eq!(heap.pop().unwrap(), TaskCategory::DeployAuthorization(Application::Ledger));
        // Approve an Compliance deploy
        assert_eq!(heap.pop().unwrap(), TaskCategory::DeployApproval(Application::Compliance));
        // Check direct messages (DMs)
        assert_eq!(heap.pop().unwrap(), TaskCategory::DirectMessage);
        // Check mentions in messaging app
        assert_eq!(heap.pop().unwrap(), TaskCategory::MessagingMention);
        // Check team channel
        assert_eq!(heap.pop().unwrap(), TaskCategory::MessagingTeamChannel);
        // Do TradeFeed code review
        assert_eq!(heap.pop().unwrap(), TaskCategory::CodeReview(Application::TradeFeed));
        // Then on to reviewing Compliance code
        assert_eq!(heap.pop().unwrap(), TaskCategory::CodeReview(Application::Compliance));
        // Catch up on all other messages in messaging app
        assert_eq!(heap.pop().unwrap(), TaskCategory::Messages);
        // Go and update the project status board
        assert_eq!(heap.pop().unwrap(), TaskCategory::ProjectStatusUpdate);
        // Finally go read those emails
        assert_eq!(heap.pop().unwrap(), TaskCategory::Email);

        // Be done for the day and get back to it tomorrow
        assert_eq!(heap.pop(), None);
        dbg!(&heap);
    }
}
