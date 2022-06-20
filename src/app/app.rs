use super::audio::Audio;
use pathdiff::diff_paths;
use std::{collections::HashSet, env, io, path::PathBuf};
use tui::widgets::{ListState, TableState};
use walkdir::WalkDir;

pub struct TabState {
    pub titles: Vec<String>,
    pub index: usize,
}

impl TabState {
    pub fn new(titles: Vec<String>) -> TabState {
        TabState { titles, index: 0 }
    }

    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.titles.len();
    }

    pub fn prev(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.titles.len() - 1;
        }
    }
}

pub struct AppStates {
    pub artists_state: ListState,
    pub artists_song_state: ListState,
    pub track_state: TableState,
    pub should_quit: bool,
    pub to_play: Option<Audio>,
}

impl AppStates {
    pub fn new() -> AppStates {
        AppStates {
            artists_state: ListState::default(),
            artists_song_state: ListState::default(),
            track_state: TableState::default(),
            should_quit: false,
            to_play: None,
        }
    }
}

pub struct AppData {
    pub audio_path: PathBuf,
    pub audios: Vec<Audio>,
    pub artist_songs: Vec<Audio>,
    pub artists: Vec<String>,
}

impl AppData {
    pub fn new() -> Result<AppData, io::Error> {
        let mut audio_path = env::home_dir().unwrap().to_str().unwrap().to_string();
        audio_path.push_str(&String::from("/Music"));
        let audio_path: PathBuf = audio_path.into();
        let audios = Audio::get_audios(&audio_path)?;
        let artists = App::get_artists(&audio_path)?;

        Ok(AppData {
            audio_path,
            audios,
            artists,
            artist_songs: Vec::new(),
        })
    }
}

pub struct App {
    pub name: String,
    pub index: usize,
    pub artist_index: usize,
    pub states: AppStates,
    pub data: AppData,
    pub tabs: TabState,
}

impl App {
    pub fn new() -> Result<App, io::Error> {
        let tabs = vec![
            String::from("Browse"),
            String::from("Tracks"),
            String::from("Play queue"),
            String::from("Settings"),
        ];

        Ok(App {
            name: "Haydeh Music Player".to_string(),
            index: 0,
            artist_index: 0,
            states: AppStates::new(),
            data: AppData::new()?,

            tabs: TabState::new(tabs),
        })
    }

    pub fn get_artists(path: &PathBuf) -> Result<Vec<String>, io::Error> {
        let mut artists: HashSet<String> = HashSet::new();
        let mut res: Vec<String> = Vec::new();

        for entry in WalkDir::new(path) {
            let entry = entry?;

            if Audio::is_mp3(&entry) {
                let path: PathBuf = entry.path().into();
                let meta = mp3_metadata::read_from_file(&path).expect("ERROR");
                let mut title = String::new();
                let mut artist = String::new();
                let mut album = String::new();

                if let Some(tag) = meta.tag {
                    title = tag.title;
                    artist = tag.artist;
                    album = tag.album;
                }

                artists.insert(artist);
            }
        }

        for artist in &artists {
            res.push(artist.to_string());
        }

        Ok(res)
    }

    pub fn on_play(&mut self) {
        self.states.to_play.as_ref().unwrap().play();
    }

    pub fn on_key(&mut self, c: char) {
        match c {
            'q' => {
                self.states.should_quit = true;
            }
            'j' => {
                self.on_down();
            }
            'k' => {
                self.on_up();
            }
            'l' => {
                self.on_right();
            }
            'h' => {
                self.on_left();
            }
            _ => {}
        }
    }

    pub fn on_down(&mut self) {
        if self.tabs.index == 0 {
            self.songs_down();
        } else if self.tabs.index == 1 {
            self.next();
        }
    }

    pub fn on_up(&mut self) {
        if self.tabs.index == 0 {
            self.songs_up();
        } else if self.tabs.index == 1 {
            self.prev();
        }
    }

    pub fn on_right(&mut self) {
        self.tabs.next();
    }

    pub fn on_left(&mut self) {
        self.tabs.prev();
    }

    pub fn artists_down(&mut self) {
        if self.tabs.index == 0 {
            let i = match self.states.artists_state.selected() {
                Some(i) => {
                    if i >= self.data.artists.len() - 1 {
                        0
                    } else {
                        i + 1
                    }
                }
                None => 0,
            };

            self.states.artists_state.select(Some(i));
            self.artist_index = i;
        }
    }

    pub fn artists_up(&mut self) {
        if self.tabs.index == 0 {
            let i = match self.states.artists_state.selected() {
                Some(i) => {
                    if i == 0 {
                        self.data.artists.len() - 1
                    } else {
                        i - 1
                    }
                }
                None => 0,
            };

            self.states.artists_state.select(Some(i));
            self.artist_index = i;
        }
    }

    pub fn songs_up(&mut self) {
        if self.tabs.index == 0 {
            let i = match self.states.artists_song_state.selected() {
                Some(i) => {
                    if i == 0 {
                        self.data.artist_songs.len() - 1
                    } else {
                        i - 1
                    }
                }
                None => 0,
            };

            self.states.artists_song_state.select(Some(i));
            self.index = i;
            self.states.to_play = Some(self.data.artist_songs[i].clone());
        }
    }

    pub fn songs_down(&mut self) {
        if self.tabs.index == 0 {
            let i = match self.states.artists_song_state.selected() {
                Some(i) => {
                    if i >= self.data.artist_songs.len() - 1 {
                        0
                    } else {
                        i + 1
                    }
                }
                None => 0,
            };

            self.states.artists_song_state.select(Some(i));
            self.index = i;
            self.states.to_play = Some(self.data.artist_songs[i].clone());
        }
    }

    pub fn next(&mut self) {
        let i = match self.states.track_state.selected() {
            Some(i) => {
                if i >= self.data.audios.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };

        self.states.track_state.select(Some(i));
        self.index = i;
        self.states.to_play = Some(self.data.audios[i].clone());
    }

    pub fn prev(&mut self) {
        let i = match self.states.track_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.data.audios.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };

        self.states.track_state.select(Some(i));
        self.index = i;
        self.states.to_play = Some(self.data.audios[i].clone());
    }

    fn get_audios_path() -> PathBuf {
        let absolute_path = Audio::load_dir();
        let current_path = env::current_dir().unwrap();

        diff_paths(&absolute_path, &current_path).unwrap()
    }
}
