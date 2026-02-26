use std::{collections::HashMap, time::Duration};

use chrono::{DateTime, Local, Timelike};
use serde_json::Value;
use thirtyfour::{
    By, WebDriver,
    action_chain::ActionChain,
    error::WebDriverResult,
    prelude::{ElementQueryable, ElementWaitable},
};
use tokio::{fs::read_to_string, time::sleep};

use crate::{animalitos::Animalitos, perfil::{Cantidad, Loteri}};
#[derive(Clone)]
pub struct Jugadas {
    animales: Animalitos,
    driver: WebDriver,
    animalitosjugados: HashMap<String, Vec<String>>,
}

impl Jugadas {
    pub async fn new(driver: WebDriver) -> Jugadas {
        let animalitos = Animalitos::new().await;

        Jugadas {
            animales: animalitos,
            driver,
            animalitosjugados: HashMap::new(),
        }
    }
    pub async fn click(&self, select: &str) {
        let mut intentos = 0;

        while intentos != 3 {
            match self.driver.find(By::XPath(select)).await {
                Ok(e) => {
                    match self.driver.execute("arguments[0].click();", vec![e.to_json().unwrap()]).await {
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
        self.driver.goto("https://www.apuestasroyal.com/index.php#").await?;
           sleep(Duration::from_millis(200)).await;
       
        // 3. Busca el elemento con tu selector CSS

        self.click("/html/body/header/section/div[1]/button").await;

        sleep(Duration::from_secs(5)).await;

        // 4. Interactúa: por ejemplo, haz clic

        let form = self
            .driver
            .find(By::XPath("/html/body/section/div/div/form/input[2]"))
            .await?;

        form.send_keys(nombre.clone()).await?;

        let form1 = self
            .driver
            .find(By::XPath("/html/body/section/div/div/form/input[3]"))
            .await?;

        form1.send_keys(contra).await?;

        self.click("/html/body/section/div/div/form/button").await;


        sleep(Duration::from_secs(3)).await;

        self.click("/html/body/div[8]/div/button[2]").await;

        Ok(())
    }

    pub async fn jugada(&mut self, numero: &str) {
        let now = Local::now().hour();

        let monto= match read_to_string("cantidad.json").await {
            Ok(c) => {
                let cantidad: Cantidad = serde_json::from_str(&c).unwrap();
                cantidad.monto
            }
            Err(_) => "0".to_string(),
        };

         if monto == "0" {
            println!("No se ha establecido una cantidad válida para apostar.");
            return;
        }

        let loterias= match read_to_string("loteria.json").await {
            Ok(l) => {
                let loteri: Loteri= serde_json::from_str(&l).unwrap();
                loteri.loto
            }
            Err(_) => "0".to_string(),
        };

         if loterias == "0" {
            println!("No se ha establecido una lotería válida para apostar.");
            return;
        }
        let animalito=format!("{}{}", numero, loterias);

        if let Some(animalito) = self.animalitosjugados.get_mut(&animalito) {
            if animalito.contains(&numero.to_string()) {
                return;
            } else {
                animalito.push(numero.to_string());
            }
        } else {
            self.animalitosjugados.insert(animalito.clone(), vec![numero.to_string()]);
        }

        if let Some(animalito) = self.animales.animalitos.get(&animalito) {

            let ani = match self.driver.find(By::XPath(animalito.to_string())).await {
                Ok(ani) => ani,
                Err(_) => {
                    println!("no funciona el animalito");
                    return;
                }
            };
            match ani.click().await {
                Ok(_) => {},
                Err(_) => {},
            };


             let form = match self
                         .driver
                         .find(By::XPath("/html/body/div[6]/div/div[6]/b/input"))
                         .await {
                 Ok(e) => {e},
                 Err(_) => {
                        println!("no funciona el form");
                        return;
                 },
             };

              let Cantidad= match read_to_string("cantidad.json").await {
            Ok(c) => {
                let cantidad: Cantidad = serde_json::from_str(&c).unwrap();
                cantidad.monto
            }
            Err(_) => "0".to_string(),
        };
           sleep(Duration::from_millis(200)).await;

           match form.clear().await {
               Ok(_) => {},
               Err(e) => {
                    println!("no funciona el form");
                    println!("error {}", e);
                    return;
               },
           }
           sleep(Duration::from_millis(200)).await;
             
             match form.send_keys(Cantidad).await {
                 Ok(_) => {},
                 Err(e) => {
                    println!("no funciona el form");
                    println!("error {}", e);
                    return;
                 },
             };

           sleep(Duration::from_millis(200)).await;
             
             self.click("/html/body/div[6]/div/button[1]").await;

       

           


            
    }

    println!("el animalito {} ya fue jugado", numero);
}

    pub async fn ficha(&self) -> Result<(), ()> {

        let elemento= match self.driver.find(By::XPath("/html/body/main/section[2]/div[4]/div[1]/div[1]/a/img")).await {
            Ok(e) => e,
            Err(_) => {
                println!("no funciona la ficha");
                return Err(());
            }
        };

        match self.driver.execute("arguments[0].click();", vec![elemento.to_json().unwrap()]).await {
            Ok(e) => {
                println!("funciona la ficha");
            },
            Err(e) => {
                println!("no funciona la ficha");
                println!("error {}", e);
            },
        };
           sleep(Duration::from_millis(200)).await;

  
        self.click("/html/body/main/div/div[1]/div[1]/a/img")
            .await;
        
        let Loteri= match read_to_string("loteria.json").await {
            Ok(c) => {
                let cantidad: Loteri = serde_json::from_str(&c).unwrap();
                cantidad.loto
            }
            Err(_) => "0".to_string(),
        };
           sleep(Duration::from_millis(200)).await;
       
       match Loteri.as_str() {
            "lo" => {
                self.click("/html/body/main/div/div[10]/div[1]/a[1]/img").await;
            },
            "gu" => {
                self.click("/html/body/main/div/div[7]/div[1]/a[1]/img").await;
            },
            "gr" => {
                self.click("/html/body/main/div/div[6]/div[1]/a[1]/img").await;
            }
            _ => {
                self.click("/html/body/main/div/div[10]/div[1]/a[1]/img").await;
               
            }
            
        }
           sleep(Duration::from_millis(200)).await;
       
        self.click("/html/body/main/div/div[2]/div[1]/div/div[2]/div/div/div/label[1]")
       .await;

    let pagina=  match self.driver.current_url().await {
        Ok(e) => e.to_string(),
        Err(_) => return Err(()),
    };
    println!("pagina actual: {}", pagina);

        if pagina.to_string()
            == "https://www.apuestasroyal.com/animalitos.php?sorteo=LOTTO%20ACTIVO"
        {
            Ok(())
        } else {
            Err(())
        }
    }

    pub async fn finalizar(&self) -> Result<(), ()> {
           sleep(Duration::from_millis(200)).await;
        
           self.click("/html/body/main/div/div[1]/div[3]/div/button[1]").await;

           sleep(Duration::from_millis(200)).await;

           self.click("/html/body/div[6]/div/button[1]").await;

           sleep(Duration::from_millis(1000)).await;

           self.click("/html/body/div[6]/div/button[1]").await;

          


           Ok(())
        // self.click("#btn_loto_purchase").await;
        // self.click("#kt_body > div.swal2-container.swal2-center.swal2-backdrop-show > div > div.swal2-actions > button.swal2-confirm.swal2-styled").await;
    }

     pub async fn cambio_loteria(&self,mensaje:Vec<String>) -> Result<(), ()> {

        match mensaje[1].as_str() {
            "lo" => {
                self.click("/html/body/main/div/div[3]/a[5]").await;
            },
            "gu" => {
                self.click("/html/body/main/div/div[3]/a[2]").await;
            },
            "gr" => {
                self.click("/html/body/main/div/div[3]/a[7]").await;
            }
            _ => {
                println!("Lotería no reconocida: {}", mensaje[1]);
                return Err(());
            }
            
        }
        
           

        self.click("/html/body/main/div/div[2]/div[1]/div/div[2]/div/div/div/label[1]").await;
     
           Ok(())
        // self.click("#btn_loto_purchase").await;
        // self.click("#kt_body > div.swal2-container.swal2-center.swal2-backdrop-show > div > div.swal2-actions > button.swal2-confirm.swal2-styled").await;

}

pub async fn refrescar_hora(&self) -> Result<(), ()> {
        

          
        self.click("/html/body/main/div/div[2]/div[1]/div/div[2]/div/div/div/label[1]").await;
     

           Ok(())
        // self.click("#btn_loto_purchase").await;
        // self.click("#kt_body > div.swal2-container.swal2-center.swal2-backdrop-show > div > div.swal2-actions > button.swal2-confirm.swal2-styled").await;
    }
}