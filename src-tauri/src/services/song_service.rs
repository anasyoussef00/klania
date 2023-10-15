use std::borrow::Cow;

use sqlx::error::ErrorKind;
use tauri::{
    api::dialog::{MessageDialogBuilder, MessageDialogButtons, MessageDialogKind},
    AppHandle, Manager,
};

use crate::{entities::song::Song, managers::song_manager::SongManager};

pub struct SongService(SongManager);

impl SongService {
    pub async fn new(app_handle: &AppHandle, song: Song) -> Self {
        let song_manager = SongManager::new(app_handle, song).await;

        Self(song_manager)
    }

    pub async fn insert(&self, app_handle: &AppHandle) -> Option<Song> {
        match self.0.insert().await {
            Ok(id) => {
                if let Ok(song) = self.0.get(id).await {
                    app_handle
                        .get_focused_window()
                        .expect("Could not get focused window!")
                        .emit("SONG-ADDED", &song)
                        .expect("Could not emit event!");

                    MessageDialogBuilder::new("Nice!", "Song added successfully!")
                        .kind(MessageDialogKind::Info)
                        .buttons(MessageDialogButtons::Ok)
                        .show(|_| {});

                    Some(song)
                } else {
                    MessageDialogBuilder::new(
                        "Oops..!",
                        "Could not retrieve last added song song!",
                    )
                    .kind(MessageDialogKind::Error)
                    .buttons(MessageDialogButtons::Ok)
                    .show(|_| {});

                    None
                }
            }
            Err(err) => {
                if let Some(err) = err.as_database_error() {
                    match err.kind() {
                        ErrorKind::UniqueViolation => {
                            let err_code = err.code().unwrap_or(Cow::from("Oops..!"));

                            MessageDialogBuilder::new(err_code, "This song already exists!")
                                .kind(MessageDialogKind::Error)
                                .buttons(MessageDialogButtons::Ok)
                                .show(|_| {});

                            None
                        }
                        _ => {
                            let err_code = err.code().unwrap_or(Cow::from("Oops..!"));
                            static ERR_MESSAGE: &str = "An unexpected database error has occurred please check your logs! [If you believe that this is a bug please report it.]";

                            MessageDialogBuilder::new(err_code, ERR_MESSAGE)
                                .kind(MessageDialogKind::Error)
                                .buttons(MessageDialogButtons::Ok)
                                .show(|_| {});

                            None
                        }
                    }
                } else {
                    static ERR_MESSAGE: &str = "An unexpected internal error has occurred please check your logs! [If you believe that this is a bug please report it.]";

                    MessageDialogBuilder::new("Oops..!", ERR_MESSAGE)
                        .kind(MessageDialogKind::Error)
                        .buttons(MessageDialogButtons::Ok)
                        .show(|_| {});

                    None
                }
            }
        }
    }
}
