use crate::driver::Driver;
use crate::jugadas;
use crate::perfil::Cantidad;
use crate::{
    driver, expresion,
    perfil::{self, Perfil},
};
use chrono::{Local, Timelike};
use std::{
    clone, collections::HashMap, env, fs, hash::Hash, path::Path, sync::Arc, time::Duration, vec,
};
use teloxide::update_listeners::Polling;
use teloxide::{
    dispatching::dialogue::GetChatId,
    prelude::*,
    types::{InputFile, InputMediaPhoto, Recipient},
};
use thirtyfour::WebDriver;
use tokio::sync::{Mutex, broadcast::Sender, mpsc::Receiver};
use tokio::time::{interval, sleep};
use tokio::{spawn, sync::broadcast};
type HHandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;
type Ar = Arc<Mutex<Sender<Vec<String>>>>;


const GRUPO: &str = "-4940706854";
pub struct Telegram;
impl Telegram {
    pub async fn escucha(&self) {
        let (tx, mut rx) = broadcast::channel::<Vec<String>>(50);

        let tx: Ar = Arc::new(Mutex::new(tx));

        // let rx = Arc::new(Mutex::new(rx));
        let grupo = GRUPO.to_string();

        let bot = Bot::new("8369993762:AAEFLWZ2fMiZuMIGVJhyDNso3gG4mDYuE9I");

        let perfiles = perfil::Perfil::new().await;

        let pefiles = Driver.factory(perfiles).await;

        match bot.send_message(grupo.clone(), "Cargando Bot...")
                    .await {
            Ok(_) => {},
            Err(e) => {
                println!("Error al enviar mensaje de inicio: {}", e);
                return;
            },
        };

        for i in pefiles {
            let mut rx = tx.lock().await.subscribe();
            let mut jugadas = jugadas::Jugadas::new(i.driver.clone()).await;
            let botaux = bot.clone();
            let grupo = grupo.clone();

            tokio::spawn(async move {
                let keepalive_task = {
                    let driver = i.driver;
                    let jugadasx = jugadas.clone();
                    let usuario = i.usuario.clone();
                    let botaux2 = botaux.clone();
                    let grupo2 = grupo.clone();
                    tokio::spawn(async move {
                        let mut ticker = interval(Duration::from_secs(30));
                        let mut hora = true;
                        loop {
                            let now = Local::now().minute();

                            if now == 57 && hora {
                                driver.refresh().await.unwrap();

                               let mensaje=match jugadasx.refrescar_hora().await {
                                   Ok(_) => {
                                        println!("Hora de jugada refrescada para {}", usuario);

                                        format!("{}\n\u{1F7E2} Hora de jugada refrescada", usuario)
                                   },
                                   Err(_) => {
                                        println!("Error al refrescar hora de jugada para {}", usuario);
                                        format!("{}\n\u{1F534} Error al refrescar hora de jugada", usuario)
                                   },
                               };
                                botaux2.send_message(grupo2.clone(), mensaje).await.unwrap();
                                hora = false;
                            } else {
                                hora = true
                            };
                            ticker.tick().await;

                            // Comando liviano para mantener viva la sesión
                            let _ = driver.current_url().await;
                        }
                    })
                };

                match jugadas
                                    .desbloquear(i.usuario.clone(), i.contrasena)
                                    .await {
                    Ok(_) => {},
                    Err(e) => {
                        println!("Error al desbloquear cuenta {}: {}", i.usuario, e);
                    },
                };

                let mensaje = match jugadas.ficha().await {
                    Ok(_) => {
                        format!("{}\n\u{1F7E2} Disponible para jugar", i.usuario)
                    }
                    Err(_) => {
                        format!("{}\n\u{1F534} No Disponible para jugar", i.usuario)
                    }
                };
                botaux.send_message(grupo.clone(), mensaje).await.unwrap();

                while let Ok(mensaje) = rx.recv().await {

                     if mensaje[0].to_lowercase() == "loteria" {
                        
                        match jugadas.cambio_loteria(mensaje).await {
                            Ok(_) => {
                                println!("Loteria cambiada");
                            },
                            Err(_) => {
                                println!("Error al cambiar loteria");
                            },
                        };
                        continue;
                    
                    
                    }
                    

                    for j in mensaje {
                        jugadas.jugada(j.as_str()).await;

                        let mensaje = match jugadas.finalizar().await {
                            Ok(_) => {
                                format!("{}\n\u{1F7E2} numero {} jugado con exito", i.usuario, j)
                            }
                            Err(_) => {
                                format!("{}\n\u{1F534} numero {} agotado", i.usuario, j)
                            }
                        };

                        botaux.send_message(grupo.clone(), mensaje).await.unwrap();
                    }
                }
                keepalive_task.abort();
            });
        }

        let handler = dptree::entry().branch(Update::filter_message().endpoint(Handler::recep));

        let es = Dispatcher::builder(bot, handler)
            .enable_ctrlc_handler()
            .dependencies(dptree::deps![tx])
            .build()
            .dispatch()
            .await;
    }
}

struct Handler;

impl Handler {
    async fn recep(bot: Bot, msg: Message, tx: Ar) -> HHandlerResult {
        if let Some(mensage) = msg.text() {
            match mensage {
                "/start" => {}
                _ => match expresion::Expresion::evaluar(mensage) {
                    Ok(numeros) => {
                        {

                    let now = Local::now();

                    if now.minute() >= 50 {
                        let error = format!(
                            "Hora no disponible para hacer jugadas,espere a las {}",
                            now.hour12().1 + 1
                        );

                        bot.send_message(GRUPO.to_string(), error).await.unwrap();

                        
                    };



                            let cola = Arc::clone(&tx);
                            let tx = cola.lock().await;
                            tx.send(
                                numeros
                                    .iter()
                                    .map(|e| e.to_string())
                                    .collect::<Vec<String>>(),
                            )?
                        };
                    }
                    Err(e) => {

                        println!("mensaje recibido: {}", mensage);

                        if mensage.to_lowercase().contains("info"){
                         
                         let cantidad= match fs::read_to_string("cantidad.json") {
                            Ok(c) => {
                                let cantidad: Cantidad = serde_json::from_str(&c).unwrap();
                                cantidad.monto
                            }
                            Err(_) => "0".to_string(),};

                            let loteria= match fs::read_to_string("loteria.json") {
                                Ok(c) => {
                                    let loteria: perfil::Loteri = serde_json::from_str(&c).unwrap();
                                    loteria.loto
                                }
                                Err(_) => "lo".to_string(),
                            };

                            

                            let info = format!("Cantidad actual: {}\nLoteria actual: {}", cantidad, loteria).to_string();
                            match bot.send_message(msg.chat.id, info.clone()).await {
                                Ok(e) => {
                                    println!("Información enviada a Telegram: {}", info);
                                },
                                Err(e) => {
                                    println!("Error al enviar información a Telegram: {}", e);
                                },
                            };
                            return Ok(());

                        };
                        

                        if mensage.to_lowercase().contains("set"){



                         
                         let cantidad=mensage.split(" ").collect::<Vec<&str>>()[1].to_string();

                         println!("Cantidad a establecer: {}", cantidad);

                         let cantidad=Cantidad{
                            monto:cantidad.to_string().trim().to_string()
                         };
                            fs::write("cantidad.json", serde_json::to_string(&cantidad).unwrap()).unwrap();
                            bot.send_message(msg.chat.id, format!("Cantidad establecida a {}", cantidad.monto)).await?;

                            return Ok(());
                        }


                        if mensage.to_lowercase().contains("loteria"){
                         
                         let loteria=mensage.split(" ").collect::<Vec<&str>>()[1].to_string();

                         let loteria=perfil::Loteri{
                            loto:loteria.to_string().trim().to_string()
                         };
                            fs::write("loteria.json", serde_json::to_string(&loteria).unwrap()).unwrap();

                            bot.send_message(msg.chat.id, format!("Loteria establecida a {}", loteria.loto)).await?;

                            let cola = Arc::clone(&tx);
                            let tx = cola.lock().await;
                            tx.send(
                                mensage
                                    .split(" ")
                                    .into_iter().map(|e| e.to_string())
                                    .collect::<Vec<String>>()
                                    
                            )?;

                            return Ok(());
                        }

                        bot.send_message(msg.chat.id, e).await?;
                    }
                },
            }
        }

        Ok(())
    }
}
