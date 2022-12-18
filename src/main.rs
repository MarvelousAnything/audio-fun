use rodio::{Decoder, OutputStream, Source};
use std::fs::File;
use std::io::BufReader;

fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let file = BufReader::new(File::open("data/hello.ogg").unwrap());
    let file2 = BufReader::new(File::open("data/hello.ogg").unwrap());
    let source = Decoder::new(file).unwrap();
    let source2 = Decoder::new(file2).unwrap();
    let mixed = source.mix(source2);
    stream_handle.play_raw(mixed.convert_samples()).unwrap();
    std::thread::sleep(std::time::Duration::from_secs(5));
}
