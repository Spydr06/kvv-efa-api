use kvv_api::{self, request::{DMRequest, Request, StopFinderRequest}};

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let request = DMRequest::builder()
        .name_dm(7001530)
        .build();

    println!("Requesting {}...", request.url());

    let response = request.get().await?;

    println!("response: {:#?}", response);
    
    let request = StopFinderRequest::builder()
        .name_sf("Hauptbahnhof")
        .build();

    println!("Requesting {}...", request.url());

    let response = request.get().await?;

    println!("response: {:#?}", response);
    
    Ok(())
}
