use crate::driver::Driver;
use crate::jugadas;
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

pub struct Telegram;

impl Telegram {
    pub async fn escucha(&self) {
        let (tx, mut rx) = broadcast::channel::<Vec<String>>(16);

        let tx: Ar = Arc::new(Mutex::new(tx));

        // let rx = Arc::new(Mutex::new(rx));

        let bot = Bot::new("8369993762:AAEFLWZ2fMiZuMIGVJhyDNso3gG4mDYuE9I");

        let perfiles = perfil::Perfil::new().await;

        let pefiles = Driver.factory(perfiles).await;

        for i in pefiles {
            let mut rx = rx.resubscribe();
            let mut jugadas = jugadas::Jugadas::new(i.driver.clone()).await;
            let botaux = bot.clone();

            tokio::spawn(async move {
                let keepalive_task = {
                    let driver = i.driver;
                    let jugadasx = jugadas.clone();
                    tokio::spawn(async move {
                        let mut ticker = interval(Duration::from_secs(30));
                        loop {
                            let now = Local::now().minute();

                            if now == 55 {
                                match driver.refresh().await {
                                    Ok(_) => {}
                                    Err(_) => {
                                        sleep(Duration::from_secs(5)).await;
                                        driver.refresh().await.unwrap()
                                    }
                                };

                                jugadasx.ficha().await.unwrap();
                            };
                            ticker.tick().await;

                            // Comando liviano para mantener viva la sesión
                            let _ = driver.current_url().await;
                        }
                    })
                };

                let grupo = "-4940706854".to_string();
                jugadas
                    .desbloquear(i.usuario.clone(), i.contrasena)
                    .await
                    .unwrap();

                let mensaje = match jugadas.ficha().await {
                    Ok(_) => {
                        format!("{}\n\u{1F7E2} Disponible para jugar", i.usuario)
                    }
                    Err(_) => {
                        format!("{}\n\u{1F534} No Disponible para jugar", i.usuario)
                    }
                };
                botaux
                    .send_message(grupo.clone(), mensaje)
                    .await
                    .unwrap();

                while let Ok(mensaje) = rx.recv().await {
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

                        botaux
                            .send_message(grupo.clone(), mensaje)
                            .await
                            .unwrap();
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
                        bot.send_message(msg.chat.id, e).await?;
                    }
                },
            }
        }

        Ok(())
    }
}
