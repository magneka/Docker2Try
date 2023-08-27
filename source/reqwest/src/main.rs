use reqwest::Error;

fn age_group(age: i32) -> Result<String, String> {
    let valid_age = match age {
        _ if age < 0 => return Err("not born yet".to_string()),
        _ if age > 150 => return Err("seriously?!!".to_string()),
        validated => validated,
    };

    let result = match valid_age {
        _ if age < 10 => "child".to_string(),
        _ if age >= 18 => "adult".to_string(),
        a => format!("teenager of {} years old", a),
    };

    Ok(result)
}

async fn get_body_request() -> Result<String, Error> {
    let response = reqwest::get("https://uc.no").await?;
    //println!("Status: {}", response.status());

    let body = response.text().await?;
    //println!("Body:\n{}", body);

    Ok(body)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    //let body = get_request().await?;    

    let age: i32 = -1;
    match age_group(age) {
        Ok(description) => println!("{}", description),
        Err(err) => println!("Error: {}", err),
    }

    
    let age: i32 = 15;
    match age_group(age) {
        Ok(description) => println!("{}", description),
        Err(err) => println!("Error: {}", err),
    }

    match get_body_request().await {
        Ok(description) => 
            println!("{}", description
        ),
        Err(err) => 
            println!("Error: {}", err
        ),
    }

    //let bdy = get_body_request().await?;
    //println!("Body:\n{}", bdy);

    

    Ok(())
}