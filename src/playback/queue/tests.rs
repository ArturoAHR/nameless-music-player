use pretty_assertions::assert_eq;

use crate::assert_matches;

use super::*;

fn generate_queue(
    track_pool: Vec<TrackId>,
    currently_playing_track_id: Option<TrackId>,
    repeat_mode: PlaybackRepeatMode,
    order: PlaybackQueueOrder,
) -> PlaybackQueue {
    let mut queue = PlaybackQueue::new();

    queue.start(track_pool, currently_playing_track_id);

    match currently_playing_track_id {
        Some(track_id) => {
            assert_eq!(queue.current(), Some(track_id));
        }
        None => {
            assert_eq!(queue.current(), None);
        }
    }

    match repeat_mode {
        PlaybackRepeatMode::NoRepeat => {
            assert_matches!(queue.repeat_mode, PlaybackRepeatMode::NoRepeat);
        }
        PlaybackRepeatMode::Repeat => {
            queue.cycle_repeat_mode();
            assert_matches!(queue.repeat_mode, PlaybackRepeatMode::Repeat);
        }
        PlaybackRepeatMode::RepeatOne => {
            queue.cycle_repeat_mode();
            queue.cycle_repeat_mode();

            assert_matches!(queue.repeat_mode, PlaybackRepeatMode::RepeatOne);
        }
    }

    match order {
        PlaybackQueueOrder::Sequential => {
            assert_matches!(queue.order, PlaybackQueueOrder::Sequential);
        }
        PlaybackQueueOrder::Shuffle => {
            queue.cycle_queue_order();

            assert_matches!(queue.order, PlaybackQueueOrder::Shuffle);
        }
    }

    for (index, track_id) in queue.track_pool.iter().enumerate() {
        assert_eq!(
            index, *track_id as usize,
            "The queue track pool is missing the expected value {index}"
        );
    }

    queue
}

#[test]
fn should_start_queue() {
    generate_queue(
        (0..10).collect(),
        Some(0),
        PlaybackRepeatMode::NoRepeat,
        PlaybackQueueOrder::Sequential,
    );
}

#[test]
fn should_start_queue_without_a_starting_track() {
    generate_queue(
        (0..10).collect(),
        None,
        PlaybackRepeatMode::NoRepeat,
        PlaybackQueueOrder::Sequential,
    );
}

#[test]
fn should_play_the_track_pool_sequentially() {
    let mut queue = generate_queue(
        (0..10).collect(),
        Some(0),
        PlaybackRepeatMode::NoRepeat,
        PlaybackQueueOrder::Sequential,
    );

    for index in 1..10 {
        let next_track_id = queue
            .next()
            .unwrap_or_else(|| panic!("Queue unexpectedly ended when retrieving track id {index}"));

        assert_eq!(next_track_id, index, "Unexpected track queued");
    }

    let value_outside_queue = queue.next();

    assert!(
        value_outside_queue.is_none(),
        "Returned another track after the non-repeating queue ended: {value_outside_queue:?}"
    );
}

#[test]
fn should_play_the_track_pool_sequentially_back_and_forth() {
    let mut queue = generate_queue(
        (0..10).collect(),
        Some(0),
        PlaybackRepeatMode::NoRepeat,
        PlaybackQueueOrder::Sequential,
    );

    for index in 1..10 {
        let next_track_id = queue.next().unwrap_or_else(|| {
            panic!("Queue unexpectedly ended when retrieving next track id {index}")
        });

        assert_eq!(next_track_id, index, "Unexpected track queued");
    }

    let next_track = queue.next();

    assert!(
        next_track.is_none(),
        "Returned another track after the non-repeating queue ended: {next_track:?}"
    );

    for index in (0..9).rev() {
        let previous = queue.previous().unwrap_or_else(|| {
            panic!("Queue unexpectedly ended when retrieving previous track id {index}")
        });

        assert_eq!(previous, index, "Unexpected track queued");
    }

    let previous_track = queue.previous();

    assert!(
        previous_track.is_none(),
        "Returned another track after reaching the non-repeating queue start: {previous_track:?}"
    );
}

#[test]
fn should_play_the_track_pool_sequentially_repeating() {
    let mut queue = generate_queue(
        (0..10).collect(),
        Some(0),
        PlaybackRepeatMode::Repeat,
        PlaybackQueueOrder::Sequential,
    );

    for expected_track_id in (1..10).chain(0..10) {
        let next_track_id = queue.next().unwrap_or_else(|| {
            panic!("Queue unexpectedly ended when retrieving track id {expected_track_id}")
        });

        assert_eq!(
            next_track_id, expected_track_id,
            "Unexpected track queued at queue position: {}",
            queue.cursor
        );
    }
}

#[test]
fn should_play_the_track_pool_sequentially_repeating_backwards() {
    let mut queue = generate_queue(
        (0..10).collect(),
        Some(0),
        PlaybackRepeatMode::Repeat,
        PlaybackQueueOrder::Sequential,
    );

    for expected_track_id in (0..10).chain(0..10).rev() {
        let previous_track_id = queue.previous().unwrap_or_else(|| {
            panic!("Queue unexpectedly ended when retrieving track id {expected_track_id}")
        });

        assert_eq!(
            previous_track_id, expected_track_id,
            "Unexpected track queued at queue position: {}",
            queue.cursor
        );
    }
}

#[test]
fn should_play_the_track_pool_sequentially_in_repeat_one_mode() {
    let mut queue = generate_queue(
        (0..10).collect(),
        Some(0),
        PlaybackRepeatMode::RepeatOne,
        PlaybackQueueOrder::Sequential,
    );

    for expected_track_id in (1..10).chain(0..10) {
        let next_track_id = queue.next().unwrap_or_else(|| {
            panic!("Queue unexpectedly ended when retrieving track id {expected_track_id}")
        });

        assert_eq!(
            next_track_id, expected_track_id,
            "Unexpected track queued at queue position: {}",
            queue.cursor
        );
    }
}

#[test]
fn should_play_the_track_pool_sequentially_in_repeat_one_mode_backwards() {
    let mut queue = generate_queue(
        (0..10).collect(),
        Some(0),
        PlaybackRepeatMode::RepeatOne,
        PlaybackQueueOrder::Sequential,
    );

    for expected_track_id in (0..10).chain(0..10).rev() {
        let previous_track_id = queue.previous().unwrap_or_else(|| {
            panic!("Queue unexpectedly ended when retrieving track id {expected_track_id}")
        });

        assert_eq!(
            previous_track_id, expected_track_id,
            "Unexpected track queued at queue position: {}",
            queue.cursor
        );
    }
}

#[test]
fn should_peek_the_next_track_in_the_queue_sequentially() {
    let mut queue = generate_queue(
        (0..10).collect(),
        Some(0),
        PlaybackRepeatMode::NoRepeat,
        PlaybackQueueOrder::Sequential,
    );

    let next_track_id = queue.peek_next();

    assert_matches!(next_track_id, Some(1));
}

#[test]
fn should_peek_the_previous_track_in_the_queue_sequentially() {
    let mut queue = generate_queue(
        (0..10).collect(),
        Some(0),
        PlaybackRepeatMode::NoRepeat,
        PlaybackQueueOrder::Sequential,
    );

    let previous_track_id = queue.peek_previous();

    assert_matches!(previous_track_id, None);
}

#[test]
fn should_peek_the_next_track_in_the_queue_repeating_sequentially() {
    let mut queue = generate_queue(
        (0..10).collect(),
        Some(0),
        PlaybackRepeatMode::Repeat,
        PlaybackQueueOrder::Sequential,
    );

    let next_track_id = queue.peek_next();

    assert_matches!(next_track_id, Some(1));
}

#[test]
fn should_peek_the_previous_track_in_the_queue_repeating_sequentially() {
    let mut queue = generate_queue(
        (0..10).collect(),
        Some(0),
        PlaybackRepeatMode::Repeat,
        PlaybackQueueOrder::Sequential,
    );

    let previous_track_id = queue.peek_previous();

    assert_matches!(previous_track_id, Some(9));
}

#[test]
fn should_peek_the_next_track_in_the_queue_repeating_one_sequentially() {
    let mut queue = generate_queue(
        (0..10).collect(),
        Some(0),
        PlaybackRepeatMode::RepeatOne,
        PlaybackQueueOrder::Sequential,
    );

    let next_track_id = queue.peek_next();

    assert_matches!(next_track_id, Some(1));
}

#[test]
fn should_peek_the_previous_track_in_the_queue_repeating_one_sequentially() {
    let mut queue = generate_queue(
        (0..10).collect(),
        Some(0),
        PlaybackRepeatMode::RepeatOne,
        PlaybackQueueOrder::Sequential,
    );

    let previous_track_id = queue.peek_previous();

    assert_matches!(previous_track_id, Some(9));
}

#[test]
fn should_play_the_track_pool_sequentially_without_a_starting_track() {
    let mut queue = generate_queue(
        (0..10).collect(),
        None,
        PlaybackRepeatMode::NoRepeat,
        PlaybackQueueOrder::Sequential,
    );

    for index in 1..10 {
        let next_track_id = queue
            .next()
            .unwrap_or_else(|| panic!("Queue unexpectedly ended when retrieving track id {index}"));

        assert_eq!(next_track_id, index, "Unexpected track queued");
    }

    let value_outside_queue = queue.next();

    assert!(
        value_outside_queue.is_none(),
        "Returned another track after the non-repeating queue ended: {value_outside_queue:?}"
    );
}

#[test]
fn should_play_the_track_pool_sequentially_back_and_forth_without_a_starting_track() {
    let mut queue = generate_queue(
        (0..10).collect(),
        None,
        PlaybackRepeatMode::NoRepeat,
        PlaybackQueueOrder::Sequential,
    );

    for index in 1..10 {
        let next_track_id = queue.next().unwrap_or_else(|| {
            panic!("Queue unexpectedly ended when retrieving next track id {index}")
        });

        assert_eq!(next_track_id, index, "Unexpected track queued");
    }

    let next_track = queue.next();

    assert!(
        next_track.is_none(),
        "Returned another track after the non-repeating queue ended: {next_track:?}"
    );

    for index in (0..9).rev() {
        let previous = queue.previous().unwrap_or_else(|| {
            panic!("Queue unexpectedly ended when retrieving previous track id {index}")
        });

        assert_eq!(previous, index, "Unexpected track queued");
    }

    let previous_track = queue.previous();

    assert!(
        previous_track.is_none(),
        "Returned another track after reaching the non-repeating queue start: {previous_track:?}"
    );
}

#[test]
fn should_play_the_track_pool_sequentially_repeating_without_a_starting_track() {
    let mut queue = generate_queue(
        (0..10).collect(),
        None,
        PlaybackRepeatMode::Repeat,
        PlaybackQueueOrder::Sequential,
    );

    for expected_track_id in (1..10).chain(0..10) {
        let next_track_id = queue.next().unwrap_or_else(|| {
            panic!("Queue unexpectedly ended when retrieving track id {expected_track_id}")
        });

        assert_eq!(
            next_track_id, expected_track_id,
            "Unexpected track queued at queue position: {}",
            queue.cursor
        );
    }
}

#[test]
fn should_play_the_track_pool_sequentially_repeating_backwards_without_a_starting_track() {
    let mut queue = generate_queue(
        (0..10).collect(),
        None,
        PlaybackRepeatMode::Repeat,
        PlaybackQueueOrder::Sequential,
    );

    for expected_track_id in (0..10).chain(0..10).rev() {
        let previous_track_id = queue.previous().unwrap_or_else(|| {
            panic!("Queue unexpectedly ended when retrieving track id {expected_track_id}")
        });

        assert_eq!(
            previous_track_id, expected_track_id,
            "Unexpected track queued at queue position: {}",
            queue.cursor
        );
    }
}

#[test]
fn should_play_the_track_pool_sequentially_in_repeat_one_mode_without_a_starting_track() {
    let mut queue = generate_queue(
        (0..10).collect(),
        None,
        PlaybackRepeatMode::RepeatOne,
        PlaybackQueueOrder::Sequential,
    );

    for expected_track_id in (1..10).chain(0..10) {
        let next_track_id = queue.next().unwrap_or_else(|| {
            panic!("Queue unexpectedly ended when retrieving track id {expected_track_id}")
        });

        assert_eq!(
            next_track_id, expected_track_id,
            "Unexpected track queued at queue position: {}",
            queue.cursor
        );
    }
}

#[test]
fn should_play_the_track_pool_sequentially_in_repeat_one_mode_backwards_without_a_starting_track() {
    let mut queue = generate_queue(
        (0..10).collect(),
        None,
        PlaybackRepeatMode::RepeatOne,
        PlaybackQueueOrder::Sequential,
    );

    for expected_track_id in (0..10).chain(0..10).rev() {
        let previous_track_id = queue.previous().unwrap_or_else(|| {
            panic!("Queue unexpectedly ended when retrieving track id {expected_track_id}")
        });

        assert_eq!(
            previous_track_id, expected_track_id,
            "Unexpected track queued at queue position: {}",
            queue.cursor
        );
    }
}

#[test]
fn should_peek_the_next_track_in_the_queue_sequentially_without_a_starting_track() {
    let mut queue = generate_queue(
        (0..10).collect(),
        None,
        PlaybackRepeatMode::NoRepeat,
        PlaybackQueueOrder::Sequential,
    );

    let next_track_id = queue.peek_next();

    assert_matches!(next_track_id, Some(0));
}

#[test]
fn should_peek_the_previous_track_in_the_queue_sequentially_without_a_starting_track() {
    let mut queue = generate_queue(
        (0..10).collect(),
        Some(0),
        PlaybackRepeatMode::NoRepeat,
        PlaybackQueueOrder::Sequential,
    );

    let previous_track_id = queue.peek_previous();

    assert_matches!(previous_track_id, None);
}

#[test]
fn should_peek_the_next_track_in_the_queue_repeating_sequentially_without_a_starting_track() {
    let mut queue = generate_queue(
        (0..10).collect(),
        None,
        PlaybackRepeatMode::Repeat,
        PlaybackQueueOrder::Sequential,
    );

    let next_track_id = queue.peek_next();

    assert_matches!(next_track_id, Some(1));
}

#[test]
fn should_peek_the_previous_track_in_the_queue_repeating_sequentially_without_a_starting_track() {
    let mut queue = generate_queue(
        (0..10).collect(),
        None,
        PlaybackRepeatMode::Repeat,
        PlaybackQueueOrder::Sequential,
    );

    let previous_track_id = queue.peek_previous();

    assert_matches!(previous_track_id, Some(9));
}

#[test]
fn should_peek_the_next_track_in_the_queue_repeating_one_sequentially_without_a_starting_track() {
    let mut queue = generate_queue(
        (0..10).collect(),
        None,
        PlaybackRepeatMode::RepeatOne,
        PlaybackQueueOrder::Sequential,
    );

    let next_track_id = queue.peek_next();

    assert_matches!(next_track_id, Some(1));
}

#[test]
fn should_peek_the_previous_track_in_the_queue_repeating_one_sequentially_without_a_starting_track()
{
    let mut queue = generate_queue(
        (0..10).collect(),
        None,
        PlaybackRepeatMode::RepeatOne,
        PlaybackQueueOrder::Sequential,
    );

    let previous_track_id = queue.peek_previous();

    assert_matches!(previous_track_id, Some(9));
}

#[test]
fn should_queue_track_next() {
    let mut queue = generate_queue(
        (0..10).collect(),
        None,
        PlaybackRepeatMode::Repeat,
        PlaybackQueueOrder::Sequential,
    );

    queue.insert_next(10);

    assert_eq!(queue.next(), Some(10));
}

#[test]
fn should_queue_several_tracks_next() {
    let mut queue = generate_queue(
        (0..10).collect(),
        None,
        PlaybackRepeatMode::Repeat,
        PlaybackQueueOrder::Sequential,
    );

    queue.insert_next(10);
    queue.insert_next(11);
    queue.insert_next(12);

    assert_eq!(queue.next(), Some(10));
    assert_eq!(queue.next(), Some(11));
    assert_eq!(queue.next(), Some(12));
}

#[test]
fn should_queue_a_track_5_tracks_later() {
    let mut queue = generate_queue(
        (0..10).collect(),
        None,
        PlaybackRepeatMode::Repeat,
        PlaybackQueueOrder::Sequential,
    );

    queue.insert(queue.cursor + 5, 10);

    for _ in 0..4 {
        queue.next();
    }

    assert_eq!(queue.next(), Some(10));
}

#[test]
fn should_queue_insertion_should_happen_at_the_end_if_insert_index_exceeds_queue_entries_length() {
    let mut queue = generate_queue(
        (0..10).collect(),
        None,
        PlaybackRepeatMode::Repeat,
        PlaybackQueueOrder::Sequential,
    );

    let original_queue_entries_length = queue.entries.len();

    queue.insert(queue.entries.len() + 10, 10);

    for _ in 0..original_queue_entries_length - 1 {
        queue.next();
    }

    assert_eq!(queue.next(), Some(10));
}

#[test]
fn should_queue_several_tracks_next_after_inserting_one_at_the_end() {
    let mut queue = generate_queue(
        (0..10).collect(),
        None,
        PlaybackRepeatMode::Repeat,
        PlaybackQueueOrder::Sequential,
    );

    queue.insert(queue.entries.len(), 15);

    queue.insert_next(10);
    queue.insert_next(11);
    queue.insert_next(12);

    assert_eq!(queue.next(), Some(10));
    assert_eq!(queue.next(), Some(11));
    assert_eq!(queue.next(), Some(12));
}

#[test]
fn should_remove_the_next_track() {
    let mut queue = generate_queue(
        (0..10).collect(),
        None,
        PlaybackRepeatMode::Repeat,
        PlaybackQueueOrder::Sequential,
    );

    queue.remove(queue.cursor + 1).unwrap();

    assert_eq!(queue.next(), Some(2));
}

#[test]
fn should_remove_the_current_track() {
    let mut queue = generate_queue(
        (0..10).collect(),
        None,
        PlaybackRepeatMode::Repeat,
        PlaybackQueueOrder::Sequential,
    );

    queue.remove(queue.cursor).unwrap();

    assert_eq!(queue.current(), Some(1));
}

#[test]
fn should_remove_the_only_track_in_the_queue() {
    let mut queue = generate_queue(
        (0..10).collect(),
        Some(0),
        PlaybackRepeatMode::NoRepeat,
        PlaybackQueueOrder::Sequential,
    );

    queue.remove(queue.cursor).unwrap();

    assert_eq!(queue.current(), None);
}

#[test]
fn should_fail_to_remove_when_index_is_out_of_bounds() {
    let mut queue = generate_queue(
        (0..10).collect(),
        None,
        PlaybackRepeatMode::Repeat,
        PlaybackQueueOrder::Sequential,
    );

    let result = queue.remove(queue.entries.len());

    assert_matches!(result, Err(PlaybackQueueError::InvalidQueueRemovePosition));
}

#[test]
fn should_truncate() {
    let mut queue = generate_queue(
        (0..10).collect(),
        None,
        PlaybackRepeatMode::Repeat,
        PlaybackQueueOrder::Sequential,
    );

    queue.truncate();

    assert_eq!(queue.current(), Some(0));
    assert_eq!(queue.entries.len(), 1);
}

#[test]
fn should_truncate_without_removing_user_queued_entries() {
    let mut queue = generate_queue(
        (0..10).collect(),
        None,
        PlaybackRepeatMode::Repeat,
        PlaybackQueueOrder::Sequential,
    );

    queue.insert(queue.entries.len(), 15);

    queue.insert_next(10);
    queue.insert_next(11);
    queue.insert_next(12);

    queue.truncate();

    assert_eq!(queue.next(), Some(10));
    assert_eq!(queue.next(), Some(11));
    assert_eq!(queue.next(), Some(12));
    assert_eq!(queue.next(), Some(15));
}
