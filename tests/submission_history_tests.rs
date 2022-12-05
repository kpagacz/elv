use aoc_elf::aoc_domain::*;
use aoc_elf::submission_history::SubmissionHistory;

#[test]
fn can_add_submission() {
    let submission = Submission::new(RiddlePart::One, "7".to_string(), 2020, 1);
    let submission_result = SubmissionResult::new(
        submission,
        SubmissionStatus::Correct,
        concat!(
            "That's the right answer! You are one gold star closer to saving your vacation.",
            " You got rank 1 on this star's leaderboard. [Return to Day 1]"
        )
        .to_string(),
        chrono::Utc::now(),
        7,
    );
    let mut submission_history = SubmissionHistory::new(2020, 1);
    submission_history.add(submission_result);
    assert_eq!(submission_history.get_submissions().len(), 1);
}

#[test]
fn previously_added_submission_result_can_be_retrieved() {
    let submission = Submission::new(RiddlePart::One, "7".to_string(), 2020, 1);
    let submission_result = SubmissionResult::new(
        submission,
        SubmissionStatus::Correct,
        concat!(
            "That's the right answer! You are one gold star closer to saving your vacation.",
            " You got rank 1 on this star's leaderboard. [Return to Day 1]"
        )
        .to_string(),
        chrono::Utc::now(),
        7,
    );
    let mut submission_history = SubmissionHistory::new(2020, 1);
    submission_history.add(submission_result);
    let new_submission = Submission::new(RiddlePart::One, "7".to_string(), 2020, 1);
    let retrieved_submission_result = submission_history.get_result_for_submission(&new_submission);
    assert_eq!(
        retrieved_submission_result.unwrap().submission,
        new_submission
    );
}

#[test]
fn get_result_for_submission_returns_none_for_a_new_submission() {
    let submission = Submission::new(RiddlePart::One, "7".to_string(), 2020, 1);
    let submission_history = SubmissionHistory::new(2020, 1);
    assert_eq!(
        submission_history.get_result_for_submission(&submission),
        None
    );
}

#[test]
fn can_submit_returns_false_if_submitted_too_soon() {
    let submission_result = SubmissionResult::new(
        Submission::new(RiddlePart::One, "7".to_string(), 2020, 1),
        SubmissionStatus::Correct,
        concat!(
            "That's the right answer! You are one gold star closer to saving your vacation.",
            " You got rank 1 on this star's leaderboard. [Return to Day 1]"
        )
        .to_string(),
        chrono::Utc::now(),
        7,
    );
    let mut submission_history = SubmissionHistory::new(2020, 1);
    submission_history.add(submission_result);
    assert_eq!(submission_history.can_submit(), false);
}

#[test]
fn can_submit_if_there_are_no_submissions() {
    let submission_history = SubmissionHistory::new(2020, 1);
    assert_eq!(submission_history.can_submit(), true);
}
