#[cfg(test)]
mod tests {

    use anyhow::Result;
    use headless_chrome::{Browser, LaunchOptions, Tab};
    use std::sync::Arc;

    fn init_browser() -> Result<(Browser, Arc<Tab>)> {
        let options = LaunchOptions::default_builder()
            .build()
            .map_err(|e| anyhow::anyhow!("Couldn't find appropriate Chrome binary: {}", e))?;
        let browser = Browser::new(options)?;
        let tab = browser.new_tab()?;
        tab.set_default_timeout(std::time::Duration::from_secs(5));
        tab.navigate_to("http://127.0.0.1:3000")?;
        tab.wait_until_navigated()?;
        Ok((browser, tab))
    }

    #[test]
    fn test_page_title_is_correct() -> Result<()> {
        let (_browser, tab) = init_browser()?;
        assert_eq!(tab.get_title()?, "Welcome to Leptos");
        assert_eq!(
            tab.find_element("h1")?.get_inner_text()?,
            "Welcome to Leptos!"
        );
        Ok(())
    }

    #[test]
    fn test_button_click_updates_text() -> Result<()> {
        let (_browser, tab) = init_browser()?;
        let button = tab.find_element("button")?;
        assert_eq!(button.get_inner_text()?, "Click Me: 0");
        button.click()?;
        assert_eq!(button.get_inner_text()?, "Click Me: 1");
        Ok(())
    }
}

fn main() {
    println!("To run the end-to-end tests, run \"cargo test\" in this directory while the dev server is running or run \"cargo leptos end-to-end\" in the project root.");
}
