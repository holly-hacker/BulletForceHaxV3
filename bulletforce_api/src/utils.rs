use futures_util::StreamExt as _;
use progenitor_client::{ByteStream, ResponseValue};

pub async fn response_to_string(response: ResponseValue<ByteStream>) -> reqwest::Result<String> {
    let stream = response.into_inner();
    let pin_of_stream = stream.into_inner();

    let vec_of_results = pin_of_stream.collect::<Vec<_>>().await;

    // transpose and merge
    let vec_of_bytes = vec_of_results
        .into_iter()
        .collect::<reqwest::Result<Vec<_>>>()?;

    // merge the bytes instances into Vec<u8>
    let vec = vec_of_bytes.into_iter().flatten().collect::<Vec<u8>>();

    let cow = String::from_utf8_lossy(&vec);

    Ok(cow.to_string())
}
