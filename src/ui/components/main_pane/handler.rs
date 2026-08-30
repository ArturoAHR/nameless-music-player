use iced::{Element, Renderer, Task};

use crate::{
    app::{self, App, TrackList},
    event::Event,
    tag::models::TagId,
    ui::{
        self,
        components::main_pane::{Message, Outcome},
        theme::Theme,
    },
};

impl App {
    pub fn view_main_pane(&self) -> Element<'_, app::Message, Theme, Renderer> {
        self.main_pane
            .view(
                &self.theme,
                &self.tracks,
                &self.tags,
                &self.displayed_track_ids,
                self.playback_bar.playing_track_id.as_ref(),
            )
            .map(ui::Message::MainPane)
            .map(app::Message::Ui)
    }

    pub fn handle_main_pane(&mut self, message: Message) -> Task<app::Message> {
        let (task, outcomes) = self.main_pane.update(message, &self.displayed_track_ids);
        let component_task = task.map(ui::Message::MainPane).map(app::Message::Ui);

        if outcomes.is_empty() {
            return component_task;
        }

        let mut tasks = vec![component_task];

        for outcome in outcomes {
            let outcome = match outcome {
                Outcome::Playback(playback_outcome) => app::Outcome::Playback(playback_outcome),
                Outcome::Modal(modal_outcome) => app::Outcome::Modal(modal_outcome),
            };

            let outcome_task = self.handle_outcome(outcome);

            tasks.push(outcome_task);
        }

        Task::batch(tasks)
    }

    pub fn notify_main_pane(&mut self, event: &Event) -> Task<app::Message> {
        self.main_pane
            .on_event(event)
            .map(ui::Message::MainPane)
            .map(app::Message::Ui)
    }

    pub fn display_main_library_tracks(&mut self) {
        self.track_list = TrackList::MainLibrary;

        self.displayed_track_ids = self.tracks.keys().copied().collect();

        self.sort_displayed_tracks();
    }

    pub fn display_tag_tracks(&mut self, tag_id: TagId) {
        if let Some(tag_track_ids) = self.track_tag_index.get_tag_tracks(tag_id) {
            self.track_list = TrackList::Tag(tag_id);

            self.displayed_track_ids = tag_track_ids.iter().copied().collect();

            self.sort_displayed_tracks();
        }
    }

    pub fn sort_displayed_tracks(&mut self) {
        self.displayed_track_ids.sort_unstable_by_key(|id| {
            let Some(track) = self.tracks.get(id) else {
                return ("unknown".to_owned(), "untitled".to_owned());
            };

            (
                track.artist.as_deref().unwrap_or("unknown").to_lowercase(),
                track.title.as_deref().unwrap_or("untitled").to_lowercase(),
            )
        });
    }
}
