use std::process::Command;

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


pub async  fn new(&self,puerto:u16)->WebDriver{

   let _chrome_process = Command::new(r"C:\Program Files\Google\Chrome\Application\chrome.exe")
        // Argumentos para el comando
        .arg(format!("--remote-debugging-port={}", puerto))
        .arg(r"--user-data-dir=C:\chrome_temp_profile")
        // spawn() lo ejecuta de fondo sin bloquear el código de Rust
        .spawn()
        .expect("Fallo crítico al intentar abrir Chrome. Revisa permisos.");

  
 let mut caps = DesiredCapabilities::chrome();

 match caps.add_experimental_option("debuggerAddress", json!(format!("127.0.0.1:{}", puerto))) {
     Ok(_) => {},
     Err(e) => {
         println!("Error al configurar las capacidades del WebDriver: {}", e);
         // Aquí podrías decidir si quieres continuar con capacidades por defecto o abortar
           // Reinicia las capacidades a un estado limpio
     },
 };

    // 2. Agrega los flags
   //  caps.add_arg("--disable-blink-features=AutomationControlled").unwrap();
   //  caps.add_arg("--disable-infobars").unwrap();
   //  caps.add_arg("--start-maximized").unwrap();

   //  // 3. Excluye el switch que inyecta el banner "Chrome is being controlled by automated test software"
   //  caps.add_experimental_option("excludeSwitches", json!(["enable-automation"])).unwrap();
   //  caps.add_experimental_option("useAutomationExtension", json!(false)).unwrap();
 
//  caps.set_headless()
//  .unwrap();

    // 4. Inicializa el driver
    let driver = match WebDriver::new("http://localhost:9515", caps.clone()).await {
        Ok(driver) => driver,
        Err(e) => {
         println!("Error al iniciar el WebDriver: {}", e);

         tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

          WebDriver::new("http://localhost:9515", caps).await.unwrap()
        },
    };

    driver

   }

 

pub async fn factory(&self,perfiles:Vec<Perfil>)->Vec<PerfilJugadas>{


let mut perfilesju=Vec::new();


for (i, perfil) in perfiles.into_iter().enumerate(){

 let driver=self.new(9222 + i as u16).await;


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