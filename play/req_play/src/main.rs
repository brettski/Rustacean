use std::collections::HashMap;

fn main() {
    let r = get_ip();

    println!("r {:?}", r);
}

fn get_ip()  -> Result<String, String> {
    //-> Result<String, String> {
    // let url = "https://httpstat.us/200";
    let url = "https://us-central1-brettskicom.cloudfunctions.net/HeaderDump";
    let res_result = reqwest::blocking::get(url);
    // println!("request:: {:#?}", res_result);
    let res = match res_result {
        Ok(val) => val,
        Err(err) => return Err(String::from(err.to_string())),
    };

    let status_code = res.status();
    println!("{:?}", status_code == 200);

    if status_code != 200 {
        return Err(String::from(format!("{} ({})", "None 200 result from endpoint", res.status().to_string())))
    }

    let json = match res.json::<HashMap<String, String>>() {
        Ok(it) => it,
        // Err(err) => return Err(format!("{} {}","unable to parse json: ", err.to_string())),
        Err(err) => return Err("unable to parse json: ".to_string() + &err.to_string()),
    };

    println!("jsonResult:: {:#?}", json);

    // unwrap() for example only
    let curip = json.get("x-forwarded-for").unwrap();

    println!("curip:: {:?}", curip);


    Ok(curip.to_string())
}