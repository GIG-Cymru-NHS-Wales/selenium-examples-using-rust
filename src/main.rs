//! Demo of Selenium browser automation with Rust.
//! Please see the file README.md for more information.
//!
//! Requires chromedriver running on port 9515:
//!
//!     chromedriver --port=9515
//!
//!     cargo run --example chrome_options

//! ## Tracking
//!
//!   * Package: selenium-examples-using-rust
//!   * Version: 1.4.0
//!   * Created: 2019-11-02T00:00:00Z
//!   * Updated: 2025-04-24T13:58:02Z
//!   * License: GPL-2.0-or-greater or for custom license contact us
//!   * Contact: Joel Parker Henderson (joel.henderson@wales.nhs.uk)

use thirtyfour::prelude::*;
use thirtyfour::components::SelectElement;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {

    // The use of color_eyre gives much nicer error reports, including making
    // it much easier to locate where the error occurred.
    color_eyre::install()?;

    //TODO
    //options.addArguments('--verbose'); // Enable verbose logging.
    //options.addArguments('--disable-notifications'); // Disable notifications such as popups.
    //options.setUserPreferences({ "profile.default_content_setting_values.cookies": 2 }); // Reject cookies.

    let mut caps = DesiredCapabilities::chrome();
    caps.insert_browser_option(
        "prefs",
        serde_json::json!({
            "profile.default_content_settings": {
                "images": 2
            },
            "profile.managed_default_content_settings": {
                "images": 2
            }
        }),
    )?;

    let driver = WebDriver::new("http://localhost:9515", caps).await?;

    driver.goto("https://testingexamples.github.io").await?;

    ////
    // Find elements in various ways.
    ////
    
    // Find an element by id.
    //
    // This demonstrates `By::Id`.
    //
    // Example HTML:
    //
    //      <p id="id-example-1">Lorem Ipsum</p>
    //
    let element_by_id = driver.find(By::Id("id-example-1")).await?;
    println!("{}", element_by_id.outer_html().await?);

    // Find an element by name .
    //
    // This demonstrates `By::Name`.
    //
    // Example HTML:
    //
    //     <p name="name-example-1">Lorem Ipsum</p>
    //
    let element_by_name = driver.find(By::Name("name-example-1")).await?;
    println!("{}", element_by_name.outer_html().await?);

    // File an element by class name.
    //
    // This demonstrates `By::ClassName`.
    //
    // Example HTML:
    //
    //     <p name="name-example-1">Lorem Ipsum</p>
    //
    let element_by_class_name = driver.find(By::ClassName("class-example-1")).await?;
    println!("{}", element_by_class_name.outer_html().await?);

    // Find an element that is a link by its text.
    //
    // This demonstrates `By::LinkText`.
    //
    // Example HTML:
    //
    //     <a href="https://example.com">Link Example 1</a>
    //
    let element_by_link_text = driver.find(By::LinkText("Link Example 1")).await?;
    println!("{}", element_by_link_text.outer_html().await?);

    // Find an element by XPath query.
    //
    // This demonstrates `By::XPath`.
    //
    // Example HTML:
    //
    //     <input type=submit>
    //
    let element_by_xpath = driver.find(By::XPath("//input[@type='submit']")).await?;
    println!("{}", element_by_xpath.outer_html().await?);

    ////
    // Interact with form inputs in various ways.
    ////

    // Type in a text input.
    //
    // Example HTML:
    //
    //     <input type="text" id="text-example-1">
    //
    let text = driver.find(By::Id("text-example-1-id")).await?;
    println!("{}", text.outer_html().await?);
    text.send_keys("hello").await?;

    // Click a checkbox input.
    //
    // Example HTML:
    //
    //     <input type="checkbox" id="checkbox-example-1-id">
    //
    let checkbox = driver.find(By::Id("checkbox-example-1-id")).await?;
    println!("{}", checkbox.outer_html().await?);
    checkbox.click().await?;

    // Click a radio input.
    //
    // Example HTML:
    //
    //     <input type="radio" id="radio-example-1-id-option-1-id">
    //
    let radio = driver.find(By::Id("radio-example-1-option-1-id")).await?;
    println!("{}", radio.outer_html().await?);
    radio.click().await?;

    // Choose a select input option.
    //
    // Example HTML:
    //
    //     <select id="select-example-1-id">
    //       <option>alfa</option>
    //       <option>bravo</option>
    //       <option>charlie</option>
    //     </select>
    //
    let select_element = driver.find(By::Id("select-example-1-id")).await?;
    println!("{}", select_element.outer_html().await?);
    let select = SelectElement::new(&select_element).await?;
    select.select_by_index(0).await?;
    let option = select.first_selected_option().await?;
    println!("{}", option.outer_html().await?);

    // Always explicitly close the browser. This prevents the executor from being blocked
    driver.quit().await?;

    Ok(())

}
