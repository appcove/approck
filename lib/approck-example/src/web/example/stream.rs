//! The text of this source file is streamed in the response. Check the terminal for the messages
//! for each chunk that was streamed.

#[approck::http(GET /example/stream; return Stream;)]
pub mod page {
    pub async fn request(req: Request) -> Response {
        let chunk_size = 65536;
        let chunk_count = 65536;
        let total_length = chunk_size * chunk_count;

        let mut response: Stream = futures::stream::iter((0..chunk_count).map(move |i| {
            println!("chunk {i}");
            vec![0u8; chunk_size]
        }))
        .map(granite::Result::Ok)
        .into();
        response
            .headers
            .append("content-type", "application/octet-stream".parse().unwrap());
        response.headers.append(
            "content-disposition",
            "attachment; filename=\"example.bin\"".parse().unwrap(),
        );
        response
            .headers
            .append("content-length", total_length.to_string().parse().unwrap());
        Response::Stream(response)
    }
}
