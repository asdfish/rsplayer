use {
    std::{
        io::BufReader,
        fs::File,
        time::Duration,
    },
    rodio::{
        Decoder,
        OutputStream,
        Sink,
        Source,
    },
};

pub struct AudioHandler {
    _stream: OutputStream, // must be kept alive
    sink: Sink,

    pub current_source_duration: Option<Duration>, // may be useful for config functions
}

impl AudioHandler {
    pub fn new() -> AudioHandler {
        let (stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        AudioHandler {
            _stream: stream,
            sink,

            current_source_duration: None,
        }
    }

    pub fn play(&mut self, path: String) {
        let file: BufReader<File> = BufReader::new(File::open(path).unwrap());
        let source = Decoder::new(file).unwrap();

        if self.is_playing() {
            self.sink.clear();
        }

        self.current_source_duration = source.total_duration();

        self.sink.append(source);
        self.sink.play();
    }
    pub fn play_duration(&self) -> Duration {
        self.sink.get_pos()
    }

    pub fn is_playing(&self) -> bool {
        !self.sink.empty() || self.sink.is_paused()
    }
}
