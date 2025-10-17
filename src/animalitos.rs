use std::{collections::HashMap, iter::Map};
#[derive(Clone)]

pub struct Animalitos{
    pub animalitos:HashMap<&'static str,&'static str>
}


impl  Animalitos {
    
   pub async fn new()->Self{

        let mut animalitos=HashMap::new();

        animalitos.insert("0","/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[4]/div[7]/div/div[1]/div[2]" );
        animalitos.insert("75","/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[4]/div[7]/div/div[1]/div[3]" );
        animalitos.insert("00","/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[4]/div[7]/div/div[1]/div[4]" );

        animalitos.insert("1","/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[4]/div[7]/div/div[2]/div[1]" );
        animalitos.insert("2","/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[4]/div[7]/div/div[2]/div[2]" );
        animalitos.insert("3","/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[4]/div[7]/div/div[2]/div[3]" );
        animalitos.insert("4","/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[4]/div[7]/div/div[2]/div[4]" );
        animalitos.insert("5","/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[4]/div[7]/div/div[2]/div[5]" );

        animalitos.insert("6","/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[4]/div[7]/div/div[3]/div[1]" );
        animalitos.insert("7","/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[4]/div[7]/div/div[3]/div[2]" );
        animalitos.insert("8","/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[4]/div[7]/div/div[3]/div[3]" );
        animalitos.insert("9","/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[4]/div[7]/div/div[3]/div[4]" );
        animalitos.insert("10","/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[4]/div[7]/div/div[3]/div[5]" );

        animalitos.insert("11","/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[4]/div[7]/div/div[4]/div[1]" );
        animalitos.insert("12","/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[4]/div[7]/div/div[4]/div[2]" );
        animalitos.insert("13","/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[4]/div[7]/div/div[4]/div[3]" );
        animalitos.insert("14","/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[4]/div[7]/div/div[4]/div[4]" );
        animalitos.insert("15","/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[4]/div[7]/div/div[4]/div[5]" );

        animalitos.insert("16","/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[4]/div[7]/div/div[5]/div[1]" );
        animalitos.insert("17","/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[4]/div[7]/div/div[5]/div[2]" );
        animalitos.insert("18","/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[4]/div[7]/div/div[5]/div[3]" );
        animalitos.insert("19","/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[4]/div[7]/div/div[5]/div[4]" );
        animalitos.insert("20","/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[4]/div[7]/div/div[5]/div[5]" );

        animalitos.insert("21","/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[4]/div[7]/div/div[6]/div[1]" );
        animalitos.insert("22","/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[4]/div[7]/div/div[6]/div[2]" );
        animalitos.insert("23","/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[4]/div[7]/div/div[6]/div[3]" );
        animalitos.insert("24","/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[4]/div[7]/div/div[6]/div[4]" );
        animalitos.insert("25","/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[4]/div[7]/div/div[6]/div[5]" );

        animalitos.insert("26","/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[4]/div[7]/div/div[7]/div[1]" );
        animalitos.insert("27","/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[4]/div[7]/div/div[7]/div[2]" );
        animalitos.insert("28","/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[4]/div[7]/div/div[7]/div[3]" );
        animalitos.insert("29","/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[4]/div[7]/div/div[7]/div[4]" );
        animalitos.insert("30","/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[4]/div[7]/div/div[7]/div[5]" );

        animalitos.insert("31","/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[4]/div[7]/div/div[8]/div[1]" );
        animalitos.insert("32","/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[4]/div[7]/div/div[8]/div[2]" );
        animalitos.insert("33","/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[4]/div[7]/div/div[8]/div[3]" );
        animalitos.insert("34","/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[4]/div[7]/div/div[8]/div[4]" );
        animalitos.insert("35","/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[4]/div[7]/div/div[8]/div[5]" );

        animalitos.insert("36","/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[4]/div[7]/div/div[9]/div[1]" );
        animalitos.insert("37","/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[4]/div[7]/div/div[9]/div[2]" );
        animalitos.insert("38","/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[4]/div[7]/div/div[9]/div[3]" );
        animalitos.insert("39","/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[4]/div[7]/div/div[9]/div[4]" );
        animalitos.insert("40","/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[4]/div[7]/div/div[9]/div[5]" );


        animalitos.insert("41","/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[4]/div[7]/div/div[10]/div[1]" );
        animalitos.insert("42","/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[4]/div[7]/div/div[10]/div[2]" );
        animalitos.insert("43","/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[4]/div[7]/div/div[10]/div[3]" );
        animalitos.insert("44","/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[4]/div[7]/div/div[10]/div[4]" );
        animalitos.insert("45","/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[4]/div[7]/div/div[10]/div[5]" );

        animalitos.insert("46","/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[4]/div[7]/div/div[11]/div[1]" );
        animalitos.insert("47","/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[4]/div[7]/div/div[11]/div[2]" );
        animalitos.insert("48","/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[4]/div[7]/div/div[11]/div[3]" );
        animalitos.insert("49","/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[4]/div[7]/div/div[11]/div[4]" );
        animalitos.insert("50","/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[4]/div[7]/div/div[11]/div[5]" );

        animalitos.insert("51","/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[4]/div[7]/div/div[12]/div[1]" );
        animalitos.insert("52","/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[4]/div[7]/div/div[12]/div[2]" );
        animalitos.insert("53","/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[4]/div[7]/div/div[12]/div[3]" );
        animalitos.insert("54","/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[4]/div[7]/div/div[12]/div[4]" );
        animalitos.insert("55","/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[4]/div[7]/div/div[12]/div[5]" );
        
        animalitos.insert("56","/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[4]/div[7]/div/div[13]/div[1]" );
        animalitos.insert("57","/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[4]/div[7]/div/div[13]/div[2]" );
        animalitos.insert("58","/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[4]/div[7]/div/div[13]/div[3]" );
        animalitos.insert("59","/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[4]/div[7]/div/div[13]/div[4]" );
        animalitos.insert("60","/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[4]/div[7]/div/div[13]/div[5]" );

        animalitos.insert("61","/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[4]/div[7]/div/div[14]/div[1]" );
        animalitos.insert("62","/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[4]/div[7]/div/div[14]/div[2]" );
        animalitos.insert("63","/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[4]/div[7]/div/div[14]/div[3]" );
        animalitos.insert("64","/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[4]/div[7]/div/div[14]/div[4]" );
        animalitos.insert("65","/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[4]/div[7]/div/div[14]/div[5]" );

        animalitos.insert("66","/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[4]/div[7]/div/div[15]/div[1]" );
        animalitos.insert("67","/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[4]/div[7]/div/div[15]/div[2]" );
        animalitos.insert("68","/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[4]/div[7]/div/div[15]/div[3]" );
        animalitos.insert("69","/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[4]/div[7]/div/div[15]/div[4]" );
        animalitos.insert("70","/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[4]/div[7]/div/div[15]/div[5]" );

        animalitos.insert("71","/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[4]/div[7]/div/div[16]/div[1]" );
        animalitos.insert("72","/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[4]/div[7]/div/div[16]/div[2]" );
        animalitos.insert("73","/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[4]/div[7]/div/div[16]/div[3]" );
        animalitos.insert("74","/html/body/div[3]/div/div/div[2]/div/div/div[1]/div/div[4]/div[7]/div/div[16]/div[4]" );

       


       



        Animalitos{
            animalitos:animalitos

        }
    }
}