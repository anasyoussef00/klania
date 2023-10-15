use tauri::{api::dialog::FileDialogBuilder, AppHandle};

use crate::{entities::song::Song, services::song_service::SongService};

pub fn open_file(app_handle: AppHandle) {
    FileDialogBuilder::new()
        .set_title("Pick a song")
        .add_filter("Audio", &["mp3", "m4a", "ogg", "wav", "flac"])
        .pick_file(move |f| match f {
            Some(file_path) => {
                tokio::spawn(async move {
                    let song = Song::new(
                        None,
                        file_path.display().to_string(),
                        None,
                        None,
                        None,
                    );
                    let song_service = SongService::new(&app_handle, song).await;

                    song_service.insert(&app_handle).await;
                });
            }
            None => {}
        })
}
