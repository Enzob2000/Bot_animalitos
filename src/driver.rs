use serde_json::json;
use thirtyfour::{BrowserCapabilitiesHelper, CapabilitiesHelper, ChromiumLikeCapabilities, DesiredCapabilities, WebDriver};
use tokio::fs::read_to_string;

use crate::perfil::{self, Perfil};


pub struct PerfilJugadas{

pub usuario:String,
pub contrasena:String,
pub driver: WebDriver


}

pub struct Driver;



impl  Driver {


pub async  fn new(&self)->WebDriver{

  
 let mut caps = DesiredCapabilities::chrome();

    // 2. Agrega los flags
   //  caps.add_arg("--disable-blink-features=AutomationControlled").unwrap();
   //  caps.add_arg("--disable-infobars").unwrap();
   //  caps.add_arg("--start-maximized").unwrap();

   //  // 3. Excluye el switch que inyecta el banner "Chrome is being controlled by automated test software"
   //  caps.add_experimental_option("excludeSwitches", json!(["enable-automation"])).unwrap();
   //  caps.add_experimental_option("useAutomationExtension", json!(false)).unwrap();
 
 caps.set_headless()
 .unwrap();

    // 4. Inicializa el driver
    let driver = WebDriver::new("http://localhost:9515", caps).await.unwrap();

    driver

   }

 

pub async fn factory(&self,perfiles:Vec<Perfil>)->Vec<PerfilJugadas>{


let mut perfilesju=Vec::new();


for perfil in perfiles{

 let driver=self.new().await;


 let usuario=PerfilJugadas{

    usuario:perfil.usuario,
    contrasena:perfil.contrasena,
    driver:driver
 };

perfilesju.push(usuario);



}

return perfilesju;





}
    
}