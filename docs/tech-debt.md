## Outcome System

- Move Queue related Outcomes from `PlaybackOutcome` enum to its own `QueueOutcome` enum.

## Playback Queue

- `generate_previous_entries_sequentially` could potentially be optimized to not require collection when getting the previous tracks identifiers reversed.
