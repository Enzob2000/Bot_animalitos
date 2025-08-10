use std::{collections::HashMap, time::Duration};

use chrono::{DateTime, Local, Timelike};
use serde_json::Value;
use thirtyfour::{
    action_chain::ActionChain, error::WebDriverResult, prelude::{ElementQueryable, ElementWaitable}, By, WebDriver
};
use tokio::time::sleep;

use crate::animalitos::Animalitos;
#[derive(Clone)]
pub struct Jugadas {
    animales: Animalitos,
    driver: WebDriver,
    animalitosjugados:HashMap<u32,Vec<String>>
}

impl Jugadas {
    pub async fn new(driver: WebDriver) -> Jugadas {
        let animalitos = Animalitos::new().await;

        Jugadas {
            animales: animalitos,
            driver,
            animalitosjugados:HashMap::new()
        }
    }
    pub async fn click(&self, select: &str) {
        let mut intentos = 0;

        while intentos != 3 {
            match self.driver.find(By::Css(select)).await {
                Ok(e) => {
                   




                    match e.click().await {
                        Ok(_) => {
                            intentos = 3;
                        }
                        Err(r) => {
                            println!("click error {}", select);
                            println!(" error {}", r);

                            sleep(Duration::from_secs(2)).await;

                            intentos += 1
                        }
                    };
                }
                Err(_) => {
                    println!("error {}", select);
                    sleep(Duration::from_secs(2)).await;
                    intentos += 1;
                }
            };
        }
    }

    pub async fn desbloquear(&mut self, nombre: String, contra: String) -> WebDriverResult<()> {
        self.driver.goto("https://secure.loteriadehoy.com/").await?;

        // 3. Busca el elemento con tu selector CSS

        self.click("#kt_body > div.swal2-container.swal2-center.swal2-backdrop-show > div > div.swal2-actions > button.swal2-cancel.swal2-styled").await;

        // 4. Interactúa: por ejemplo, haz clic

        let form = self
            .driver
            .find(By::Css("#login_form > div:nth-child(1) > input"))
            .await?;

        form.send_keys(nombre).await?;

        let form1 = self
            .driver
            .find(By::Css("#login_form > div:nth-child(2) > input"))
            .await?;

        form1.send_keys(contra).await?;

        self.click("#kt_login_signin_submit").await;

        self.click("#kt_content > div > div > section > div.container > div > div.col-lg-6.col-md-12 > div > div.contentCircle > div.CirItem.title-box.CirItem1.active > button").await;

        Ok(())
    }

    
    pub async fn jugada(&mut self, numero: &str) {

         let now=Local::now().hour();

        if let Some(animalito)= self.animalitosjugados.get_mut(&now){


             if animalito.contains(&numero.to_string()){

                return;
             }else {
                 animalito.push(numero.to_string());
             }
        }else {
            self.animalitosjugados.insert(now, vec![numero.to_string()]);
        }
      
        if let Some(animalito) = self.animales.animalitos.get(numero) {

            let ani=match self.driver.find(By::XPath(animalito.to_string())).await {
                Ok(ani) => ani,
                Err(_) => {

                    println!("no funciona el animalito");
                    return ;
                } 
            };

            let mut intent=0;

            while intent!=3 {
                
            match self.driver
                    .execute(
                        "arguments[0].click();",
                        vec![ani.to_json().unwrap()],        // convertimos WebElement a JSON
                    )
                    .await {
                Ok(_) => {
                    println!("funciona el animalito");
                    intent+=1;
                },
                Err(_) => {

                    println!("no funciona el animalito");
                   return ;
                },
            }
        
        };


        }
    }

    pub async fn ficha(&self) {

        
        self.click("#play_vtab > li.nav-item2.how-work-item.col-lg-2.col-md-3.col-sm-3.col-3.p_22")
            .await;
      
       
        self.click(r"#p_22_tab > div:nth-child(6) > div > div:nth-child(4)")
            .await;

       self.click("#p_22_tab > div:nth-child(3) > div > div.col-12.row.kt-checkbox-inline.jc-all.mb-5 > div:nth-child(1) > label")
       .await;
    }
    
   

    pub async fn finalizar(&self) {
        
        self.click(r"#p_22_tab > div.col-12.d-flex.jc-all.row > button.btn.btn-proccess.ml-5.mr-3.loto-play-single-bet2").await;
        self.click("#play_single_bet > div > div > div.modal-body > div:nth-child(1) > div.div_wallet.col-12 > div > select").await;
        self.click("#play_single_bet > div > div > div.modal-body > div:nth-child(1) > div.div_wallet.col-12 > div > select > option:nth-child(3)").await; 
        self.click("#btn_loto_purchase").await;
        self.click("#kt_body > div.swal2-container.swal2-center.swal2-backdrop-show > div > div.swal2-actions > button.swal2-confirm.swal2-styled").await;
        
    }
}
