use regex::Regex;



pub struct Expresion;


impl  Expresion {

    pub fn evaluar(expre:&str)->Result<Vec<&str>,&str>{
    
   let re = Regex::new(r"^(?:0?\d|[1-6]\d|7[0-5])(?:,(?:0?\d|[1-6]\d|7[0-5]))*$").unwrap();

      if let Some(_) = re.captures(expre) {

      let numeros:Vec<&str>=expre.split(",").collect();

      return Ok(numeros);




    
 }

 Err("El mensaje no es valido")
    
}
}
