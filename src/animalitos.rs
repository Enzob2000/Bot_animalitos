use std::{collections::HashMap, iter::Map};
#[derive(Clone)]

pub struct Animalitos{
    pub animalitos:HashMap<&'static str,&'static str>
}


impl  Animalitos {
    
   pub async fn new()->Self{

        let mut animalitos=HashMap::new();

        animalitos.insert("00","/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[9]/div[7]/div/div[2]/div[3]" );

        
        animalitos.insert("0","/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[9]/div[7]/div/div[2]/div[2]" );

        

        animalitos.insert("01",r"/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[9]/div[7]/div/div[3]/div[1]" );
        animalitos.insert("1",r"/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[9]/div[7]/div/div[3]/div[1]" );

        
        animalitos.insert("2",r"#/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[9]/div[7]/div/div[3]/div[2]" );
        animalitos.insert("02",r"/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[9]/div[7]/div/div[3]/div[2]" );

        animalitos.insert("03",r"/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[9]/div[7]/div/div[3]/div[3]" );
        animalitos.insert("3",r"/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[9]/div[7]/div/div[3]/div[3]" );
        
        animalitos.insert("04",r"/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[9]/div[7]/div/div[4]/div[1]" );
        animalitos.insert("4",r"/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[9]/div[7]/div/div[4]/div[1]" );

        animalitos.insert("05",r"/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[9]/div[7]/div/div[4]/div[2]" );
        animalitos.insert("5",r"/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[9]/div[7]/div/div[4]/div[2]" );

        animalitos.insert("06",r"/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[9]/div[7]/div/div[4]/div[3]" );
        animalitos.insert("6",r"/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[9]/div[7]/div/div[4]/div[3]" );

        animalitos.insert("07",r"/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[9]/div[7]/div/div[5]/div[1]" );
        animalitos.insert("7",r"/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[9]/div[7]/div/div[5]/div[1]" );

        animalitos.insert("08",r"/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[9]/div[7]/div/div[5]/div[2]" );
        animalitos.insert("8",r"/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[9]/div[7]/div/div[5]/div[2]" );

        animalitos.insert("09",r"/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[9]/div[7]/div/div[5]/div[3]" );
        animalitos.insert("9",r"/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[9]/div[7]/div/div[5]/div[3]" );

        animalitos.insert("10",r"/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[9]/div[7]/div/div[6]/div[1]" );
        animalitos.insert("11",r"/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[9]/div[7]/div/div[6]/div[2]" );
        animalitos.insert("12",r"/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[9]/div[7]/div/div[6]/div[3]" );

        animalitos.insert("13",r"/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[9]/div[7]/div/div[7]/div[1]" );
        animalitos.insert("14",r"/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[9]/div[7]/div/div[7]/div[2]" );
        animalitos.insert("15",r"/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[9]/div[7]/div/div[7]/div[3]" );

        animalitos.insert("16",r"/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[9]/div[7]/div/div[8]/div[1]" );
        animalitos.insert("17",r"/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[9]/div[7]/div/div[8]/div[2]" );
        animalitos.insert("18",r"/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[9]/div[7]/div/div[8]/div[3]" );

        animalitos.insert("19",r"/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[9]/div[7]/div/div[9]/div[1]" );
        animalitos.insert("20",r"/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[9]/div[7]/div/div[9]/div[2]" );
        animalitos.insert("21",r"/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[9]/div[7]/div/div[9]/div[3]" );

        animalitos.insert("22",r"/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[9]/div[7]/div/div[10]/div[1]" );
        animalitos.insert("23",r"/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[9]/div[7]/div/div[10]/div[2]" );
        animalitos.insert("24",r"/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[9]/div[7]/div/div[10]/div[3]" );

        animalitos.insert("25",r"/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[9]/div[7]/div/div[11]/div[1]" );
        animalitos.insert("26",r"/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[9]/div[7]/div/div[11]/div[2]" );
        animalitos.insert("27",r"/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[9]/div[7]/div/div[11]/div[3]" );

        animalitos.insert("28",r"/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[9]/div[7]/div/div[12]/div[1]" );
        animalitos.insert("29",r"/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[9]/div[7]/div/div[12]/div[2]" );
        animalitos.insert("30",r"/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[9]/div[7]/div/div[12]/div[3]" );

        animalitos.insert("31",r"/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[9]/div[7]/div/div[13]/div[1]" );
        animalitos.insert("32",r"/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[9]/div[7]/div/div[13]/div[2]" );
        animalitos.insert("33",r"/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[9]/div[7]/div/div[13]/div[3]" );

        animalitos.insert("34",r"/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[9]/div[7]/div/div[14]/div[1]" );
        animalitos.insert("35",r"/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[9]/div[7]/div/div[14]/div[2]" );
        animalitos.insert("36",r"/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[9]/div[7]/div/div[14]/div[3]" );
       



        Animalitos{
            animalitos:animalitos

        }
    }
}