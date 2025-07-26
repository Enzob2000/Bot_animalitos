use std::{clone, time::Duration};

use thirtyfour::{By, WebDriver, error::WebDriverResult};
use tokio::time::sleep;

use crate::animalitos::Animalitos;
#[derive(Clone)]
pub struct Jugadas {
    animales: Animalitos,
}

impl Jugadas {
    pub async fn new() -> Jugadas {
        let animalitos = Animalitos::new().await;

        Jugadas {
            animales: animalitos,
        }
    }

    pub async fn desbloquear(
        &self,
        driver:& WebDriver,
        nombre: String,
        contra: String,
    ) -> WebDriverResult<()> {
        driver.goto("https://secure.loteriadehoy.com/").await?;

        sleep(Duration::from_secs(10)).await;

        // 3. Busca el elemento con tu selector CSS
        let elem = driver
        .find(By::Css("#kt_body > div.swal2-container.swal2-center.swal2-backdrop-show > div > div.swal2-actions > button.swal2-cancel.swal2-styled"))
        .await?;

        // 4. Interactúa: por ejemplo, haz clic
        elem.click().await?;

        let form = driver
            .find(By::Css("#login_form > div:nth-child(1) > input"))
            .await?;

        form.send_keys(nombre).await?;

        let form1 = driver
            .find(By::Css("#login_form > div:nth-child(2) > input"))
            .await?;

        form1.send_keys(contra).await?;

        driver
            .find(By::Css("#kt_login_signin_submit"))
            .await?
            .click()
            .await?;

        sleep(Duration::from_secs(5)).await;

        driver.find(By::Css("#kt_content > div > div > section > div.container > div > div.col-lg-6.col-md-12 > div > div.contentCircle > div.CirItem.title-box.CirItem1.active > button")).await?.click().await?;

        Ok(())
    }

    pub async fn jugada(&self, driver: &WebDriver, numero: String) {
        if let Some(animalito) = self.animales.animalitos.get(numero.as_str()) {
            driver
                .find(By::Css(*animalito))
                .await
                .unwrap()
                .click()
                .await
                .unwrap();
            driver
                .find(By::Css(*animalito))
                .await
                .unwrap()
                .click()
                .await
                .unwrap();
            driver
                .find(By::Css(*animalito))
                .await
                .unwrap()
                .click()
                .await
                .unwrap();
        }
    }

    pub async fn ficha(&self, driver:&WebDriver) {
        driver
            .find(By::Css(
                "#play_vtab > li.nav-item2.how-work-item.col-lg-2.col-md-3.col-sm-3.col-3.p_22",
            ))
            .await
            .unwrap()
            .click()
            .await
            .unwrap();

        driver
            .find(By::Css(
                r"#p_1_tab > div:nth-child(6) > div > div:nth-child(5)",
            ))
            .await
            .unwrap()
            .click()
            .await
            .unwrap();
    }

    pub async fn finalizar(&self, driver:&WebDriver) {
        driver
            .find(By::Css(
                r"#p_1_tab > div.col-12.d-flex.jc-all.row > button.btn.btn-proccess.ml-5.mr-3.loto-play-single-bet2",
            ))
            .await
            .unwrap()
            .click()
            .await
            .unwrap();
        driver.find(By::Css("#play_single_bet > div > div > div.modal-body > div:nth-child(1) > div.div_wallet.col-12 > div > select")).await.unwrap().click().await.unwrap();
        driver.find(By::Css("#play_single_bet > div > div > div.modal-body > div:nth-child(1) > div.div_wallet.col-12 > div > select > option:nth-child(3)")).await.unwrap().click().await.unwrap();
        driver.find(By::Css("#btn_loto_purchase")).await.unwrap().click().await.unwrap();

    }
}
