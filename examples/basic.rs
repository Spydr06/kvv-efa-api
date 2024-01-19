use kvv_api::{self, request::{DMRequest, Request, StopFinderRequest}};

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {        
    /*{
        // Search the station table for "Hauptbahnhof"
        let request = StopFinderRequest::builder()
            .name_sf("Hauptbahnhof")
            .build();

        println!("Requesting {}...", request.url());

        let response = request.get().await?;

        println!("response: {:#?}", response);
    }*/

    {
        // Fetch the departures of station 7000801 ("Durlach Bahnhof")
        let request = DMRequest::builder()
            .name_dm(7000090)
            .build();

        println!("Requesting {}...", request.url());

        let response = request.get().await?;

        println!("response: {:#?}", response);
    }

    Ok(())
}
