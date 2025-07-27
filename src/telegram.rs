use crate::driver::Driver;
use crate::jugadas;
use crate::{
    driver, expresion,
    perfil::{self, Perfil},
};
use std::{
    clone, collections::HashMap, env, fs, hash::Hash, path::Path, sync::Arc, thread::sleep,
    time::Duration, vec,
};
use teloxide::update_listeners::Polling;
use teloxide::{
    dispatching::dialogue::GetChatId,
    prelude::*,
    types::{InputFile, InputMediaPhoto, Recipient},
};
use thirtyfour::WebDriver;
use tokio::sync::{Mutex, broadcast::Sender, mpsc::Receiver};
use tokio::time::interval;
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

        // let rx_clone=Arc::clone(&rx);

        for i in pefiles {
            let mut rx = rx.resubscribe();
            let mut jugadas = jugadas::Jugadas::new(i.driver.clone()).await;

            tokio::spawn(async move {
                let keepalive_task = {
                    let driver = i.driver;
                    tokio::spawn(async move {
                        let mut ticker = interval(Duration::from_secs(30));
                        loop {
                            ticker.tick().await;

                            // Comando liviano para mantener viva la sesión
                            let _ = driver.current_url().await;
                        }
                    })
                };

                jugadas.desbloquear(i.usuario, i.contrasena).await.unwrap();

                jugadas.ficha().await;

                while let Ok(mensaje) = rx.recv().await {
                    for i in mensaje {

                          jugadas.jugada(i.as_str()).await
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
        let start = format!(r"numeros a juagar separados por ','");

        if let Some(mensage) = msg.text() {
            match mensage {
                "/start" => {
                    bot.send_message(msg.chat.id, start.clone()).await?;
                }
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
