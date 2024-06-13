use serde::{Deserialize, Serialize};
use spin_sdk::http::{IntoResponse, Request, Response};
use spin_sdk::{http_component, key_value};

#[derive(Serialize, Deserialize, Debug)]
struct Counter {
    count: usize
}

/// A simple Spin HTTP component.
#[http_component]
fn handle_ndc_spin(req: Request) -> anyhow::Result<impl IntoResponse> {
    println!("Handling request to {:?}", req.header("spin-full-url"));

    let store = key_value::Store::open("redis")?;

    let count: Counter = match store.get_json::<Counter>("counter").unwrap() {
        Some(c) => {
            Counter {
                count: c.count + 1
            }
        },
        None => {
            Counter {
                count: 1
            }
        },
    };

    store.set_json::<Counter>("counter", &count).unwrap();

    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body(serde_json::to_string_pretty::<Counter>(&count).unwrap())
        .build())
}