#[approck::http(GET /example/boxit/ajax/random-xy; return JSON;)]
pub mod random_xy {
    use rand::Rng;
    use serde_json::json;
    // create a array of 50 colors
    const COLORS: [&str; 50] = [
        "#FF6633", "#FFB399", "#FF33FF", "#FFFF99", "#00B3E6", "#E6B333", "#3366E6", "#999966",
        "#99FF99", "#B34D4D", "#80B300", "#809900", "#E6B3B3", "#6680B3", "#66991A", "#FF99E6",
        "#CCFF1A", "#FF1A66", "#E6331A", "#33FFCC", "#66994D", "#B366CC", "#4D8000", "#B33300",
        "#CC80CC", "#66664D", "#991AFF", "#E666FF", "#4DB3FF", "#1AB399", "#E666B3", "#33991A",
        "#CC9999", "#B3B31A", "#00E680", "#4D8066", "#809980", "#E6FF80", "#1AFF33", "#999933",
        "#FF3380", "#CCCC00", "#66E64D", "#4D80CC", "#9900B3", "#E64D66", "#4DB380", "#FF4D4D",
        "#99E6E6", "#6666FF",
    ];

    static WIDTH: f64 = 2000.0;
    static HEIGHT: f64 = 800.0;

    pub async fn request() -> Response {
        let mut rng = rand::thread_rng();
        let width: f64 = rng.gen_range(10.0..100.0);
        let height: f64 = rng.gen_range(10.0..100.0);
        let x: f64 = rng.gen_range(0.0..(WIDTH - width));
        let y: f64 = rng.gen_range(45.0..(HEIGHT - height));
        let color: &str = COLORS[rng.gen_range(0..50)];
        Response::JSON(
            json!({
                "x": x,
                "y": y,
                "width": width,
                "height": height,
                "color": color,
            })
            .into(),
        )
    }
}
