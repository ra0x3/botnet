use anomaly_detector::{AnomalyDetectorResult, Extractor, Input, Key, Value};
use url::Url;

struct UserAgentExtractor {
    input: Input,
}

impl UserAgentExtractor {
    fn new(input: Input) -> Self {
        Self { input }
    }
}

impl Extractor for UserAgentExtractor {
    type Input = Input;
    fn extract(&self) -> AnomalyDetectorResult<(Key, Value)> {
        let url = Url::parse(self.input.as_ref());
        Ok(("1", "1"))
    }
}
fn main() {
    let ext = UserAgentExtractor::new(Input::from("http://google.com"));
    println!("Hello world!");
}
