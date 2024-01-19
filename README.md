# kvv-api-rs

Rust bindings for the KVV (Karlsruher Verkehrsverbund) "EFA" API

> **Warning:**
> This API doesn't seem like a permanent solution, it could change at any time completely, making these bindings useless!

## Example Code:

```rs
use kvv_api::{self, request::{DMRequest, Request, StopFinderRequest}};

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {        
    {
        // Search the station table for "Hauptbahnhof"
        let request = StopFinderRequest::builder()
            .name_sf("Hauptbahnhof")
            .build();

        println!("Requesting {}...", request.url());

        let response = request.get().await?;

        println!("response: {:#?}", response);
    }

    {
        // Fetch the departures of station 7000801 ("Durlach Bahnhof")
        let request = DMRequest::builder()
            .name_dm(7000801)
            .build();

        println!("Requesting {}...", request.url());

        let response = request.get().await?;

        println!("response: {:#?}", response);
    }

    Ok(())
}
```

> from [`examples/basic.rs`](./examples/basic.rs)

