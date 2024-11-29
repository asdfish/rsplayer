use std::{
    io::BufReader,
    fs::File
};

use rodio::{
    Decoder,
    OutputStream,
    OutputStreamHandle,
    Sink,
};

pub struct AudioHandler {
    stream: OutputStream,
    stream_handle: OutputStreamHandle,
    sink: Sink,
}

impl AudioHandler {
    pub fn new() -> AudioHandler {
        let (stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        return AudioHandler {
            stream: stream,
            stream_handle: stream_handle,
            sink: sink,
        };
    }

    pub fn play(&self, path: String) {
        let file: BufReader<File> = BufReader::new(File::open(path).unwrap());
        let source = Decoder::new(file).unwrap();

        if !self.sink.empty() {
            self.sink.clear();
        }

        self.sink.append(source);
        self.sink.play();
    }

    pub fn is_playing(&self) -> bool {
        return !self.sink.empty() || self.sink.is_paused();
    }
}
