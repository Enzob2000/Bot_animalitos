use std::{path::Path, process::{Command, Stdio}, time::Duration};

use tokio::{fs::File, io::AsyncWriteExt, process, time::sleep};



const exe: &[u8]=include_bytes!("chromedriver.exe");

pub struct Webdriver;



impl Webdriver {

    pub async fn carga(&self){

        let ruta = Path::new("chromedriver.exe");


    if !ruta.exists(){
        let mut file = File::create("chromedriver.exe").await.unwrap();
        file.write_all(exe).await.unwrap();
        file.flush().await.unwrap();         // Asegura que todo está en disco
    } 

sleep(Duration::from_secs(1)).await;

     let chromedriver = r".\chromedriver.exe"; // Ajusta tu ruta

    Command::new(chromedriver)
    .stdin(Stdio::null())
    .stdout(Stdio::null())
    .stderr(Stdio::null())

        .arg("--port=9515")
        .spawn()
        .map_err(|e| {
            eprintln!("Error al iniciar Chromedriver (`{}`): {}", chromedriver, e);
            e
        }).unwrap();

    }
}


        


    
    
