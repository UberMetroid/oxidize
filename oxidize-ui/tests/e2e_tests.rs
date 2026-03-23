use headless_chrome::Browser;

#[tokio::test]
async fn test_page_loads_with_title() {
    let browser = Browser::default().expect("Failed to launch Chrome");
    let tab = browser.new_tab().expect("Failed to create tab");

    tab.navigate_to("http://127.0.0.1:9531")
        .expect("Failed to navigate");

    std::thread::sleep(std::time::Duration::from_secs(2));

    let title = tab
        .evaluate("document.title", true)
        .expect("Failed to evaluate");
    let value = title.value.expect("No value");
    assert_eq!(value.as_str().unwrap_or(""), "Oxidize");
}

#[tokio::test]
async fn test_page_has_oxidize_heading() {
    let browser = Browser::default().expect("Failed to launch Chrome");
    let tab = browser.new_tab().expect("Failed to create tab");

    tab.navigate_to("http://127.0.0.1:9531")
        .expect("Failed to navigate");

    std::thread::sleep(std::time::Duration::from_secs(2));

    let heading = tab
        .evaluate("document.querySelector('h1')?.textContent", true)
        .expect("Failed to evaluate");
    let value = heading.value.expect("No value");
    assert_eq!(value.as_str().unwrap_or(""), "OXIDIZE");
}

#[tokio::test]
async fn test_page_has_upgrade_buttons() {
    let browser = Browser::default().expect("Failed to launch Chrome");
    let tab = browser.new_tab().expect("Failed to create tab");

    tab.navigate_to("http://127.0.0.1:9531")
        .expect("Failed to navigate");

    std::thread::sleep(std::time::Duration::from_secs(2));

    let buttons = tab
        .evaluate("document.querySelectorAll('button').length", true)
        .expect("Failed to evaluate");
    let value = buttons.value.expect("No value");
    let count = value.as_i64().unwrap_or(0);
    assert!(count >= 5);
}

#[tokio::test]
async fn test_leaderboard_button_exists() {
    let browser = Browser::default().expect("Failed to launch Chrome");
    let tab = browser.new_tab().expect("Failed to create tab");

    tab.navigate_to("http://127.0.0.1:9531")
        .expect("Failed to navigate");

    std::thread::sleep(std::time::Duration::from_secs(2));

    let leaderboard = tab
        .evaluate("document.body.textContent.includes('LEADERBOARD')", true)
        .expect("Failed to evaluate");
    let value = leaderboard.value.expect("No value");
    assert!(value.as_bool().unwrap_or(false));
}

#[tokio::test]
async fn test_dysphere_button_exists() {
    let browser = Browser::default().expect("Failed to launch Chrome");
    let tab = browser.new_tab().expect("Failed to create tab");

    tab.navigate_to("http://127.0.0.1:9531")
        .expect("Failed to navigate");

    std::thread::sleep(std::time::Duration::from_secs(2));

    let dysphere = tab
        .evaluate("document.body.textContent.includes('DYSPHERE')", true)
        .expect("Failed to evaluate");
    let value = dysphere.value.expect("No value");
    assert!(value.as_bool().unwrap_or(false));
}
