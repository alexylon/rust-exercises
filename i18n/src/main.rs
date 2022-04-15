mod i18n_test;

use warp::Filter;

#[allow(unused_variables)]
#[tokio::main]
async fn main() {
    // GET http://127.0.0.1:3000/{locale}/hello =>
    // 200 OK with body in the corresponding language
    let hello_path = warp::path!(String / "hello")
        .map(|locale| i18n_test::i18n_test(locale, "Marcus"));

    // GET http://127.0.0.1:3000 with header locale: {locale} =>
    // 200 OK with body in the corresponding language
    let hello_header = warp::header::<String>("accept-language")
        .map(|locales: String| { i18n_test::i18n_test(locales, "Marcus") });

    warp::serve(hello_header)
        .run(([127, 0, 0, 1], 3000))
        .await;
}
