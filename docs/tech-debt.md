## Outcome System

- Move Queue related Outcomes from `PlaybackOutcome` enum to its own `QueueOutcome` enum.

## UI Theme

- Add shadow sizing and palette values.
- Add sizing for 6.0 pixels.

## Playback Queue

- `generate_previous_entries_sequentially` function could potentially be optimized to not require collection when getting the previous tracks identifiers reversed.

## Track Tagging Modal

- `handle_keyboard` function  could be broken down further to resolve `clippy::too_many_lines`.
