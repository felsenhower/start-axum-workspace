#[cfg(test)]
mod tests {

    use anyhow::Result;
    use headless_chrome::{Browser, LaunchOptions, Tab};
    use rstest::*;
    use std::sync::Arc;
    use leptos::prelude::*;

    fn init_browser() -> Result<Browser> {
        let options = LaunchOptions::default_builder()
            .headless(true)
            .sandbox(false)
            .build()?;
        let browser = Browser::new(options)?;
        Ok(browser)
    }
    
    fn init_tab(browser: &Browser, site_url: &str) -> Result<Arc<Tab>> {
        let tab = browser.new_tab()?;
        tab.set_default_timeout(std::time::Duration::from_secs(5));
        tab.navigate_to(site_url)?;
        tab.wait_until_navigated()?;
        Ok(tab)
    }

    #[fixture]
    #[once]
    fn browser() -> Browser {
        init_browser().expect("Unable to create browser")
    }

    #[fixture]
    #[once]
    fn site_url() -> String {
        let conf = get_configuration(Some("../Cargo.toml")).unwrap();
        let addr = conf.leptos_options.site_addr;
        format!("http://{}", addr)
    }

    #[fixture]
    fn tab(browser: &Browser, site_url: &str) -> Arc<Tab> {
        init_tab(browser, site_url).expect("Unable to create tab")
    }

    #[rstest]
    fn test_page_title_is_correct(tab: Arc<Tab>) -> Result<()> {
        assert_eq!(tab.get_title()?, "Welcome to Leptos");
        assert_eq!(
            tab.find_element("h1")?.get_inner_text()?,
            "Welcome to Leptos!"
        );
        Ok(())
    }

    #[rstest]
    fn test_button_click_updates_text(tab: Arc<Tab>) -> Result<()> {
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
