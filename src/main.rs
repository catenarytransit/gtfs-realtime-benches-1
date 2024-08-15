use std::{hash::Hash, time::Instant};
use ahash::AHasher;
use quick_protobuf::BytesReader;
use std::hash::Hasher;
use prost::Message;
mod transit_realtime;
use quick_protobuf::Reader;
use quick_protobuf::MessageRead;
use wyhash2::WyHash;

#[tokio::main]
async fn main() {
    println!("Downloading uk trip data");

    let sf_trip_url = "https://birch.catenarymaps.org/gtfs_rt?feed_id=f-bus~dft~gov~uk~rt&feed_type=trip";

    let request = reqwest::get(sf_trip_url).await.unwrap();

    println!("Downloaded uk trip data");

    let bytes = request.bytes().await.unwrap();

    println!("{} bytes", bytes.len());

    println!("starting hashes");

    let start_hash = Instant::now();

    for _ in 0..1000 {
        let _ = ahash_fast_hash(&bytes.as_ref());
    }

    let end_hash = Instant::now();

    println!("hash time: {:?}", (end_hash - start_hash)/1000);

    let start_decode = Instant::now();

    for _ in 0..1000 {

        let x:gtfs_realtime::FeedMessage = prost::Message::decode(bytes.as_ref()).unwrap();
    }
    let end_decode = Instant::now();
    println!("decode time prost unoptimised: {:?}", (end_decode - start_decode)/1000);

    
    let new_start_decode = Instant::now();

    for _ in 0..1000 {

    let mut reader = BytesReader::from_bytes(&bytes);

    // now using the generated module decoding is as easy as:
    let msg = transit_realtime::FeedMessage::from_reader(&mut reader, &bytes).expect("Cannot read data");

    }

    let new_end_decode = Instant::now();

    
    println!("new decode time quick-protobuf: {:?}", (new_end_decode - new_start_decode)/1000);
    
}

pub fn ahash_fast_hash<T: Hash>(t: &T) -> u64 {
    let mut hasher = AHasher::default();
    t.hash(&mut hasher);
    hasher.finish()
}
