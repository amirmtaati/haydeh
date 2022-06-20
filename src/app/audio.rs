use mp3_metadata;
use rodio::Sink;
use std::{
    env,
    fs::File,
    io::{self, BufReader},
    path::*,
};
use walkdir::{DirEntry, WalkDir};

#[derive(Clone)]
pub struct Audio {
    pub title: String,
    pub artist: String,
    pub album: String,
    pub path: PathBuf,
}

impl Audio {
    pub fn load_dir() -> PathBuf {
        let args: Vec<String> = env::args().collect();
        let path: PathBuf = args[1].clone().into();

        path
    }

    pub fn get_audios(path: &PathBuf) -> Result<Vec<Audio>, io::Error> {
        let mut audios: Vec<Audio> = Vec::new();

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

                audios.push(Audio {
                    title,
                    artist,
                    album,
                    path,
                });
            }
        }

        Ok(audios)
    }

    pub fn get_artist_songs(artist: String, path: &PathBuf) -> Result<Vec<Audio>, io::Error> {
        let mut songs: Vec<Audio> = Vec::new();

        for entry in WalkDir::new(path) {
            let entry = entry?;

            if Audio::is_mp3(&entry) {
                let path: PathBuf = entry.path().into();
                let meta = mp3_metadata::read_from_file(&path).expect("ERROR");
                let mut title = String::new();
                let mut song_artist = String::new();
                let mut album = String::new();

                if let Some(tag) = meta.tag {
                    title = tag.title;
                    song_artist = tag.artist;
                    album = tag.album;
                }

                if song_artist == artist {
                    songs.push(Audio {
                        title,
                        artist: song_artist,
                        album,
                        path,
                    });
                }
            }
        }

        Ok(songs)
    }

    pub fn play(&self) {
        let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
        let sink = rodio::Sink::try_new(&stream_handle).unwrap();
        let file = File::open(&self.path).unwrap();
        let source = rodio::Decoder::new(BufReader::new(file)).unwrap();

        sink.append(source);
        sink.sleep_until_end();
    }

    pub fn is_mp3(entry: &DirEntry) -> bool {
        let c = entry.file_name().to_str().unwrap().ends_with("mp3");
        c
    }
}
