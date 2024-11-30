use {
    std::{
        io::BufReader,
        fs::File,
    },
    rodio::{
        Decoder,
        OutputStream,
        Sink,
    },
};

#[allow(dead_code)]
pub struct AudioHandler {
    stream: OutputStream, // must be kept alive
    sink: Sink,
}

impl AudioHandler {
    pub fn new() -> AudioHandler {
        let (stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        return AudioHandler {
            stream: stream,
            sink: sink,
        };
    }

    pub fn play(&self, path: String) {
        let file: BufReader<File> = BufReader::new(File::open(path).unwrap());
        let source = Decoder::new(file).unwrap();

        if self.is_playing() {
            self.sink.clear();
        }

        self.sink.append(source);
        self.sink.play();
    }

    pub fn is_playing(&self) -> bool {
        return !self.sink.empty() || self.sink.is_paused();
    }
}
