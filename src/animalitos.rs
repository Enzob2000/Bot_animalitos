use std::{collections::HashMap, iter::Map};
#[derive(Clone)]

pub struct Animalitos{
    pub animalitos:HashMap<&'static str,&'static str>
}


impl  Animalitos {
    
   pub async fn new()->Self{

        let mut animalitos=HashMap::new();

        animalitos.insert("00","#animalito > div.row.line1 > div.num0.numerito.green" );
        animalitos.insert("0","#animalito > div.row.line1 > div.num00.numerito.green" );

        animalitos.insert("01",r"#animalito > div:nth-child(3) > div.num1.numerito.\31 -12.\31 -18.\31 -34.red.odd.flipped" );
        animalitos.insert("1",r"#animalito > div:nth-child(3) > div.num1.numerito.\31 -12.\31 -18.\31 -34.red.odd.flipped" );

        animalitos.insert("02",r"#animalito > div:nth-child(3) > div.num2.numerito.\31 -12.\31 -18.\32 -35.black.even" );
        animalitos.insert("02",r"#animalito > div:nth-child(3) > div.num2.numerito.\31 -12.\31 -18.\32 -35.black.even" );

        animalitos.insert("03",r"#animalito > div:nth-child(3) > div.num3.numerito.\31 -12.\31 -18.\33 -36.red.odd.flipped" );
        animalitos.insert("3",r"#animalito > div:nth-child(3) > div.num3.numerito.\31 -12.\31 -18.\33 -36.red.odd.flipped" );
        
        animalitos.insert("04",r"#animalito > div:nth-child(4) > div.num4.numerito.\31 -12.\31 -18.\31 -34.black.even" );
        animalitos.insert("4",r"#animalito > div:nth-child(4) > div.num4.numerito.\31 -12.\31 -18.\31 -34.black.even" );

        animalitos.insert("05",r"#animalito > div:nth-child(4) > div.num5.numerito.\31 -12.\31 -18.\32 -35.red.odd" );
        animalitos.insert("5",r"#animalito > div:nth-child(4) > div.num5.numerito.\31 -12.\31 -18.\32 -35.red.odd" );

        animalitos.insert("06",r"#animalito > div:nth-child(4) > div.num6.numerito.\31 -12.\31 -18.\33 -36.black.even" );
        animalitos.insert("6",r"#animalito > div:nth-child(4) > div.num6.numerito.\31 -12.\31 -18.\33 -36.black.even" );

        animalitos.insert("07",r"#animalito > div:nth-child(5) > div.num4.numerito.\31 -12.\31 -18.\31 -34.red.odd" );
        animalitos.insert("7",r"#animalito > div:nth-child(5) > div.num4.numerito.\31 -12.\31 -18.\31 -34.red.odd" );

        animalitos.insert("08",r"#animalito > div:nth-child(5) > div.num5.numerito.\31 -12.\31 -18.\32 -35.black.even" );
        animalitos.insert("8",r"#animalito > div:nth-child(5) > div.num5.numerito.\31 -12.\31 -18.\32 -35.black.even" );

        animalitos.insert("09",r"#animalito > div:nth-child(5) > div.num6.numerito.\31 -12.\31 -18.\33 -36.red.odd" );
        animalitos.insert("9",r"#animalito > div:nth-child(5) > div.num6.numerito.\31 -12.\31 -18.\33 -36.red.odd" );

        animalitos.insert("10",r"#animalito > div:nth-child(6) > div.num4.numerito.\31 -12.\31 -18.\31 -34.black.even" );
        animalitos.insert("11",r"#animalito > div:nth-child(6) > div.num5.numerito.\31 -12.\31 -18.\32 -35.black.odd" );
        animalitos.insert("12",r"#animalito > div:nth-child(6) > div.num6.numerito.\31 -12.\31 -18.\33 -36.red.even" );

        animalitos.insert("13",r"#animalito > div:nth-child(7) > div.num4.numerito.\31 3-24.\31 -18.\31 -34.black.odd" );
        animalitos.insert("14",r"#animalito > div:nth-child(7) > div.num5.numerito.\31 3-24.\31 -18.\32 -35.red.even" );
        animalitos.insert("15",r"#animalito > div:nth-child(7) > div.num6.numerito.\31 3-24.\31 -18.\33 -36.black.odd" );

        animalitos.insert("16",r"#animalito > div:nth-child(8) > div.num4.numerito.\31 3-24.\31 -18.\31 -34.red.even" );
        animalitos.insert("17",r"#animalito > div:nth-child(8) > div.num5.numerito.\31 3-24.\31 -18.\32 -35.black.odd" );
        animalitos.insert("18",r"#animalito > div:nth-child(8) > div.num6.numerito.\31 3-24.\31 -18.\33 -36.red.even" );

        animalitos.insert("19",r"#animalito > div:nth-child(9) > div.num4.numerito.\31 3-24.\31 9-36.\31 -34.red.odd" );
        animalitos.insert("20",r"#animalito > div:nth-child(9) > div.num5.numerito.\31 3-24.\31 9-36.\32 -35.black.even" );
        animalitos.insert("21",r"#animalito > div:nth-child(9) > div.num6.numerito.\31 3-24.\31 9-36.\33 -36.red.odd" );

        animalitos.insert("22",r"#animalito > div:nth-child(10) > div.num4.numerito.\31 3-24.\31 9-36.\31 -34.black.even" );
        animalitos.insert("23",r"#animalito > div:nth-child(10) > div.num5.numerito.\31 3-24.\31 9-36.\32 -35.red.odd" );
        animalitos.insert("24",r"#animalito > div:nth-child(10) > div.num6.numerito.\31 3-24.\31 9-36.\33 -36.black.even" );

        animalitos.insert("25",r"#animalito > div:nth-child(11) > div.num4.numerito.\32 5-36.\31 9-36.\31 -34.red.odd" );
        animalitos.insert("26",r"#animalito > div:nth-child(11) > div.num5.numerito.\32 5-36.\31 9-36.\32 -35.black.even" );
        animalitos.insert("27",r"#animalito > div:nth-child(11) > div.num6.numerito.\32 5-36.\31 9-36.\33 -36.red.odd" );

        animalitos.insert("28",r"#animalito > div:nth-child(12) > div.num4.numerito.\32 5-36.\31 9-36.\31 -34.black.even" );
        animalitos.insert("29",r"#animalito > div:nth-child(12) > div.num5.numerito.\32 5-36.\31 9-36.\32 -35.black.odd" );
        animalitos.insert("30",r"#animalito > div:nth-child(12) > div.num6.numerito.\32 5-36.\31 9-36.\33 -36.red.even.flipped" );

        animalitos.insert("31",r"#animalito > div:nth-child(13) > div.num4.numerito.\32 5-36.\31 9-36.\31 -34.black.odd" );
        animalitos.insert("32",r"#animalito > div:nth-child(13) > div.num5.numerito.\32 5-36.\31 9-36.\32 -35.red.even" );
        animalitos.insert("33",r"#animalito > div:nth-child(13) > div.num6.numerito.\32 5-36.\31 9-36.\33 -36.black.odd" );

        animalitos.insert("34",r"#animalito > div:nth-child(14) > div.num4.numerito.\32 5-36.\31 9-36.\31 -34.red.even" );
        animalitos.insert("35",r"#animalito > div:nth-child(14) > div.num5.numerito.\32 5-36.\31 9-36.\32 -35.black.odd" );
        animalitos.insert("36",r"#animalito > div:nth-child(14) > div.num6.numerito.\32 5-36.\31 9-36.\33 -36.red.even" );
       



        Animalitos{
            animalitos:animalitos

        }
    }
}