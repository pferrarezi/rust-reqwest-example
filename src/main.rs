extern crate reqwest;

fn main() {
    let url = "http://httpbin.org/anything";
    let req = reqwest::blocking::get(url).expect("Erro na requisição GET");
    let req_text;
    if req.status().is_success() {
        req_text = req.text().expect("Erro obtendo response text");
        println!("body = {}", req_text);
    } else {
        // req_text = String::from("");
        print!("Status: {}", req.status())
    }
}
