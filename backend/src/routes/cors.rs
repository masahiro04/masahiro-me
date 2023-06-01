use worker::*;

pub fn cors() -> Cors {
    let origins = vec![
        String::from("https://masahiro.me"),
        String::from("http://localhost:3000"),
    ];
    Cors::new()
        .with_origins(vec!["*".to_string()])
        .with_allowed_headers(vec!["*".to_string()])
        .with_max_age(86_400)
        .with_methods(vec![Method::Get, Method::Options, Method::Head])
}
