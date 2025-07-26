use std::time::Duration;
use thirtyfour::{common::command::BySelector, prelude::*};
use tokio::time::sleep;
mod animalitos;
mod telegram;
mod expresion;
mod driver;
mod perfil;
mod jugadas;
mod webdriver;

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    webdriver::Webdriver.carga().await;
    
    // 1. Arranca el driver apuntando al WebDriver local
   telegram::Telegram.escucha().await;
    Ok(())
}
