use std::{collections::HashMap, iter::Map};
#[derive(Clone)]

pub struct Animalitos{
    pub animalitos:HashMap<String,String>
}



impl  Animalitos {
    
   pub async fn new()->Self{

        let mut animalitos: HashMap<String, String>=HashMap::new();


animalitos.insert("00lo".to_string(),"/html/body/main/div/div[2]/div[2]/div/div[2]/div/div/button[1]".to_string());
animalitos.insert("0lo".to_string(),"/html/body/main/div/div[2]/div[2]/div/div[2]/div/div/button[2]".to_string());


for i in 3..=38{
    let key = format!("{}lo", i-2);
    let value = format!("/html/body/main/div/div[2]/div[2]/div/div[2]/div/div/button[{}]", i);
    animalitos.insert(key, value.to_string());
}



animalitos.insert("00gr".to_string(),"/html/body/main/div/div[2]/div[2]/div/div[2]/div/div/button[1]".to_string());
animalitos.insert("0gr".to_string(),"/html/body/main/div/div[2]/div[2]/div/div[2]/div/div/button[2]".to_string());


for i in 3..=38{
    let key = format!("{}gr", i-2);
    let value = format!("/html/body/main/div/div[2]/div[2]/div/div[2]/div/div/button[{}]", i);
    animalitos.insert(key, value.to_string());
}


animalitos.insert("00gu".to_string(),"/html/body/main/div/div[2]/div[2]/div/div[2]/div/div/button[1]".to_string());
animalitos.insert("0gu".to_string(),"/html/body/main/div/div[2]/div[2]/div/div[2]/div/div/button[2]".to_string());


for i in 3..=77{
    let key = format!("{}gu", i-2);
    let value = format!("/html/body/main/div/div[2]/div[2]/div/div[2]/div/div/button[{}]", i);
    animalitos.insert(key, value.to_string());
}








       


       



        Animalitos{
            animalitos:animalitos

        }
    }
}