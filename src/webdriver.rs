use std::{
    path::Path,
    process::{Command, Stdio},
    time::Duration,
};


use tokio::{fs::File, io::AsyncWriteExt, process, time::sleep};

const exe: &[u8] = include_bytes!("chromedriver.exe");
const exe_linux: &[u8] = include_bytes!("chromedriver");

pub struct Webdriver;

impl Webdriver {
   #[cfg(target_os = "linux")]
    pub async fn carga(&self) {
        let ruta = Path::new("chromedriver");

        if !ruta.exists() {
            let mut file = File::create("chromedriver").await.unwrap();
            file.write_all(exe_linux).await.unwrap();
            file.flush().await.unwrap(); // Asegura que todo está en disco
        }

        sleep(Duration::from_secs(1)).await;

        let chromedriver = r"./chromedriver"; // Ajusta tu ruta

        let status = Command::new("chmod")
        .arg("+x")
        .arg("chromedriver")
        .status()
        .expect("Error al ejecutar chmod");



        Command::new(chromedriver)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .arg("--port=9515")
            .arg("--whitelisted-ips=") 
            .spawn()
            .map_err(|e| {
                eprintln!("Error al iniciar Chromedriver (`{}`): {}", chromedriver, e);
                e
            })
            .unwrap();
    }

    #[cfg(target_os = "windows")]
    pub async fn carga(&self) {
        let ruta = Path::new("chromedriver.exe");

        if !ruta.exists() {
            let mut file = File::create("chromedriver.exe").await.unwrap();
            file.write_all(exe).await.unwrap();
            file.flush().await.unwrap(); // Asegura que todo está en disco
        }

        sleep(Duration::from_secs(1)).await;

        let chromedriver = r".\chromedriver.exe"; // Ajusta tu ruta

      

        Command::new(chromedriver)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .arg("--port=9515")
            .arg("--whitelisted-ips=") 
            .spawn()
            .map_err(|e| {
                eprintln!("Error al iniciar Chromedriver (`{}`): {}", chromedriver, e);
                e
            })
            .unwrap();
    }
}
