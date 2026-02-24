use std::fs::read_to_string;


use serde::{Deserialize, Serialize};
use thirtyfour::WebDriver;


#[derive(Debug, Deserialize)]
pub struct Perfil{
pub usuario:String,
pub contrasena:String,



}

#[derive(Debug, Deserialize,Serialize)]
pub struct Cantidad{
pub monto:String,




}
#[derive(Debug, Deserialize,Serialize)]

pub struct Loteri{
    pub loto:String
}


impl  Perfil{

pub async fn new()->Vec<Perfil>{



let perfiles=match read_to_string("perfiles.json") {
    Ok(e) => e,
    Err(e) => {

       println!("{}",e);

        return Vec::new()
    },
};



let perfiles:Vec<Perfil>=serde_json::from_str(&perfiles).unwrap();

perfiles


 
}


    
}