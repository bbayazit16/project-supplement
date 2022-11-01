use rand::{thread_rng, Rng};
use std::{
    collections::HashMap,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
};

fn info(message: String) {
    println!(
        "\x1b[32m\x1b[1mINFO \x1b[0m[{}] \x1b[1m{}\x1b[0m",
        base_time(),
        message
    );
}

fn warn(message: String) {
    println!(
        "\x1b[33m\x1b[1mWARN \x1b[0m[{}] \x1b[1m{}\x1b[0m",
        base_time(),
        message
    );
}

fn base_time() -> String {
    chrono::Local::now()
        .format("%d/%m/%y %H:%M:%S.%3f")
        .to_string()
}

#[derive(Debug)]
struct Guess {
    count: u8,
    num: u32,
}

impl Guess {
    fn new() -> Self {
        Guess {
            count: 0,
            num: thread_rng().gen_range(0..=1_000_000_000),
        }
    }
}

impl std::fmt::Display for Guess {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "count={}, num={}", self.count, self.num)
    }
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:3000").unwrap();

    let mut map: HashMap<String, Guess> = HashMap::new();
    map.insert("fd581b2f6501b75a".to_string(), Guess::new());
    map.insert("21b57f556b027a49".to_string(), Guess::new());
    let map_mpsc = Arc::new(Mutex::new(map));

    for stream in listener.incoming() {
        std::thread::spawn({
            let map = Arc::clone(&map_mpsc);
            move || {
                handle_connection(stream.unwrap(), map);
            }
        });
    }
}

fn handle_connection(mut stream: TcpStream, map: Arc<Mutex<HashMap<String, Guess>>>) {
    let mut buf = [0u8; 1024];
    stream.read(&mut buf).unwrap();

    let (status, contents) = parse_request(buf, map);

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status,
        contents.len(),
        contents
    );

    let ok = stream.write(response.as_bytes());

    if ok.is_err() {
        warn("Err writing to stream".to_owned());
    }
}

fn parse_request(buf: [u8; 1024], map: Arc<Mutex<HashMap<String, Guess>>>) -> (String, String) {
    if buf.starts_with(b"GET /guess") {
        let key = String::from_utf8_lossy(&buf);
        let key = key
            .split("\n")
            .nth(0)
            .unwrap()
            .split_whitespace()
            .nth(1)
            .unwrap();
        let params = extract_params(key.to_owned());

        if params.is_err() {
            return (
                "HTTP/1.1 400 BAD REQUEST".to_owned(),
                "{\"message\": \"😡 bad request\"}".to_owned(),
            );
        }
        let params = params.unwrap();
        let api_key = params.get("key");
        let num = params.get("num");

        if api_key.is_none() || num.is_none() {
            return (
                "HTTP/1.1 400 BAD REQUEST".to_owned(),
                "{\"message\": \"😡 bad request\"}".to_owned(),
            );
        }

        let api_key = api_key.unwrap();
        if !map.lock().unwrap().contains_key(api_key) {
            return (
                "HTTP/1.1 403 FORBIDDEN".to_owned(),
                "{\"message\": \"Yanlis api key 😡\"}".to_owned(),
            );
        }

        let num = num.unwrap().parse::<u32>();
        if num.is_err() {
            return (
                "HTTP/1.1 400 BAD REQUEST".to_owned(),
                "{\"message\": \"😡 num sayi degil ya da cok buyuk\"}".to_owned(),
            );
        }

        let num = num.unwrap();
        if num > 1_000_000_000 {
            return (
                "HTTP/1.1 400 BAD REQUEST".to_owned(),
                "{\"message\": \"😡 sayi makimum 1,000,000,000\"}".to_owned(),
            );
        }
        return handle_guess(api_key, num, &map);
    }

    if buf.starts_with(b"GET /reset") {
        let key = String::from_utf8_lossy(&buf);
        let key = key
            .split("\n")
            .nth(0)
            .unwrap()
            .split_whitespace()
            .nth(1)
            .unwrap();
        let params = extract_params(key.to_owned());
        if params.is_err() {
            return (
                "HTTP/1.1 400 BAD REQUEST".to_owned(),
                "{\"message\": \"😡 bad request\"}".to_owned(),
            );
        }
        let params = params.unwrap();
        let api_key = params.get("key");

        if api_key.is_none() {
            return (
                "HTTP/1.1 400 BAD REQUEST".to_owned(),
                "{\"message\": \"😡 bad request\"}".to_owned(),
            );
        }

        let api_key = api_key.unwrap();

        if !map.lock().unwrap().contains_key(api_key) {
            return (
                "HTTP/1.1 403 FORBIDDEN".to_owned(),
                "{\"message\": \"Yanlis api key 😡\"}".to_owned(),
            );
        }

        return handle_reset(api_key, &map);
    }
    return (
        "HTTP/1.1 404 NOT FOUND".to_owned(),
        "{\"message\": \"bu methodu bilmiyorum 😡 guess/?key=apikey ya da reset/?key=apikey kullan\"}"
            .to_owned(),
    );
}

fn extract_params(clean_params_line: String) -> Result<HashMap<String, String>, ()> {
    let mut params = HashMap::new();
    if !clean_params_line.contains("?") {
        return Err(());
    }
    for param in clean_params_line
        .split("?")
        .nth(1)
        .unwrap()
        .split("&")
        .collect::<Vec<&str>>()
    {
        let parsed = param.split("=").collect::<Vec<&str>>();
        if parsed.len() != 2 {
            return Err(());
        }
        params.insert(parsed[0].to_owned(), parsed[1].to_owned());
    }
    Ok(params)
}

fn handle_reset(key: &String, map: &Arc<Mutex<HashMap<String, Guess>>>) -> (String, String) {
    *map.lock().unwrap().get_mut(key).unwrap() = Guess::new();

    match key.as_str() {
        "fd581b2f6501b75a" => info(format!(
            "Ozan reset={}",
            map.lock().unwrap().get(key).unwrap()
        )),
        "21b57f556b027a49" => info(format!(
            "Bora reset={} ",
            map.lock().unwrap().get(key).unwrap()
        )),
        _ => warn(format!("??? {}", key)),
    };

    (
        "HTTP/1.1 200 OK".to_owned(),
        "{\"message\": \"Yeni tahmin atandi, bol sans 😀\"}".to_owned(),
    )
}

fn handle_guess(
    key: &String,
    num: u32,
    map: &Arc<Mutex<HashMap<String, Guess>>>,
) -> (String, String) {
    (*map.lock().unwrap().get_mut(key).unwrap()).count += 1;

    match key.as_str() {
        "fd581b2f6501b75a" => info(format!(
            "Ozan req={} {}",
            num,
            map.lock().unwrap().get(key).unwrap()
        )),
        "21b57f556b027a49" => info(format!(
            "Bora req={} {}",
            num,
            map.lock().unwrap().get(key).unwrap()
        )),
        _ => warn(format!("??? {}", key)),
    }

    if map.lock().unwrap().get(key).unwrap().count == 31 {
        *map.lock().unwrap().get_mut(key).unwrap() = Guess::new();
    }

    let guess = map.lock().unwrap().get(key).unwrap().num;

    if guess == num {
        *map.lock().unwrap().get_mut(key).unwrap() = Guess::new();
        match key.as_str() {
            "fd581b2f6501b75a" => info(format!(
                "Ozan success {}",
                map.lock().unwrap().get(key).unwrap()
            )),
            "21b57f556b027a49" => info(format!(
                "Bora success {}",
                map.lock().unwrap().get(key).unwrap()
            )),
            _ => warn(format!("??? {}", key)),
        }
        ("HTTP/1.1 200 OK".to_owned(), "{\"message\": 0}".to_owned())
    } else if guess < num {
        ("HTTP/1.1 200 OK".to_owned(), "{\"message\": -1}".to_owned())
    } else {
        ("HTTP/1.1 200 OK".to_owned(), "{\"message\": 1}".to_owned())
    }
}
