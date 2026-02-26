
use serde_json::json;
use thirtyfour::{CapabilitiesHelper, ChromiumLikeCapabilities, DesiredCapabilities, WebDriver};
use tokio::process::Command;
use crate::perfil::Perfil;

pub struct PerfilJugadas {
    pub usuario: String,
    pub contrasena: String,
    pub driver: WebDriver,
    // Opcional: Guardamos el proceso para matarlo cuando el struct se destruya
    // pub chrome_process: Child, 
}

pub struct Driver;

impl Driver {
    pub async fn new(&self, puerto: u16) -> WebDriver {
        // 1. Hacemos que la carpeta del perfil sea única por instancia
        let profile_dir = format!(r"C:\chrome_temp_profile_{}", puerto);

        let _chrome_process = Command::new(r"C:\Program Files\Google\Chrome\Application\chrome.exe")
            .arg(format!("--remote-debugging-port={}", puerto))
            .arg(format!("--user-data-dir={}", profile_dir))
            .spawn()
            .expect("Fallo crítico al intentar abrir Chrome. Revisa permisos o la ruta.");
     tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
     
        let mut caps = DesiredCapabilities::chrome();

        match caps.add_experimental_option("debuggerAddress", json!(format!("127.0.0.1:{}", puerto))) {
            Ok(_) => {},
            Err(e) => println!("Error al configurar las capacidades del WebDriver: {}", e),
        };

        // 2. Inicializa el driver (Asegúrate de que chromedriver esté corriendo en el puerto 9515)
        let driver = match WebDriver::new("http://localhost:9515", caps.clone()).await {
            Ok(driver) => driver,
            Err(e) => {
                println!("Error al iniciar el WebDriver en puerto {}: {}. Reintentando...", puerto, e);
                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                WebDriver::new("http://localhost:9515", caps).await.unwrap()
            }
        };

        driver
    }

    pub async fn factory(&self, perfiles: Vec<Perfil>) -> Vec<PerfilJugadas> {
        let mut perfilesju = Vec::new();

        for (i, perfil) in perfiles.into_iter().enumerate() {
            // Asignamos un puerto único dinámicamente
            let puerto_debug = 9222 + i as u16;
            let driver = self.new(puerto_debug).await;

            let perfil_jugada = PerfilJugadas {
                usuario: perfil.usuario,
                contrasena: perfil.contrasena,
                driver,
            };

            perfilesju.push(perfil_jugada);
        }

        perfilesju
    }
}