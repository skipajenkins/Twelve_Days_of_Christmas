fn main() {

    twelve_days_of_christmas();
}

fn twelve_days_of_christmas(){


    let _days: [&str;12] = ["first","second","third","fourth","fifth","sixth",
    "seventh","eigth","nineth","tenth","eleventh","twelveth" 
    ];

    let _gift: [&str;12] = ["A patridge in a pear tree","Two turtle doves","Three french hens","Four calling bird","Five Golden rings","Six geese a-laying",
    "Seven swans a-swimming","Eight maids a-milking","Nine ladies dancing","Ten lords a-leaping","Eleve pipers piping","Twelve drummers drumming"];

    let mut index: usize = 0;    

    while index < 12 {
        
        println!("On the {} day of Christmas, my true love gave to me", _days[index]);
        
        if index == 0{ 
        
        println!("{}.",_gift[index]);

        }

        else if index == 1{
         println!("{},",_gift[index]);
         println!("and {}.",_gift[index-1]);
        }
        
        else if index == 2{
         println!("{},",_gift[index]);
         println!("{},",_gift[index-1]);
         println!("and {}.",_gift[index-2]);
        }
        
        else if index == 3{
         println!("{},",_gift[index]);
         println!("{},",_gift[index-1]);
         println!("{},",_gift[index-2]);
         println!("and {}.",_gift[index-3]);
        }
        
        else if index == 4{
         println!("{},",_gift[index]);
         println!("{},",_gift[index-1]);
         println!("{},",_gift[index-2]);
         println!("{},",_gift[index-3]);
         println!("and {}.",_gift[index-4]);
        }       
        
        else if index == 5{
         println!("{}.",_gift[index]);
         println!("{},",_gift[index-1]);
         println!("{},",_gift[index-2]);
         println!("{},",_gift[index-3]);
         println!("and {}.",_gift[index-5]);
        }    
        
         else if index == 6{
         println!("{}.",_gift[index]);
         println!("{},",_gift[index-1]);
         println!("{},",_gift[index-2]);
         println!("{},",_gift[index-4]);
         println!("{},",_gift[index-5]);
         println!("and {}.",_gift[index-6]);
        }
        
         else if index == 7{
         println!("{}.",_gift[index]);
         println!("{}.",_gift[index-1]);
         println!("{},",_gift[index-2]);
         println!("{},",_gift[index-3]);
         println!("{},",_gift[index-4]);
         println!("{},",_gift[index-5]);
         println!("and {}.",_gift[index-6]);
         println!("and {}.",_gift[index-7]);
        }

        else if index == 8{
         println!("{}.",_gift[index]);
         println!("{}.",_gift[index-1]);
         println!("{},",_gift[index-2]);
         println!("{},",_gift[index-3]);
         println!("{},",_gift[index-4]);
         println!("{},",_gift[index-5]);
         println!("{},",_gift[index-6]);
         println!("{},",_gift[index-7]);
         println!("and {}.",_gift[index-8]);
        }

        else if index == 9{
         println!("{}.",_gift[index]);
         println!("{}.",_gift[index-1]);
         println!("{},",_gift[index-2]);
         println!("{},",_gift[index-3]);
         println!("{},",_gift[index-4]);
         println!("{},",_gift[index-5]);
         println!("{},",_gift[index-6]);
         println!("{},",_gift[index-7]);
         println!("{},",_gift[index-8]);
         println!("and {}.",_gift[index-9]);
        }

        else if index == 10{
         println!("{}.",_gift[index]);
         println!("{}.",_gift[index-1]);
         println!("{},",_gift[index-2]);
         println!("{},",_gift[index-3]);
         println!("{},",_gift[index-4]);
         println!("{},",_gift[index-5]);
         println!("{},",_gift[index-6]);
         println!("{},",_gift[index-7]);
         println!("{},",_gift[index-8]);
         println!("{}.",_gift[index-9]);
         println!("and {}.",_gift[index-10]);
        }

        else if index == 11{
         println!("{}.",_gift[index]);
         println!("{}.",_gift[index-1]);
         println!("{},",_gift[index-2]);
         println!("{},",_gift[index-3]);
         println!("{},",_gift[index-4]);
         println!("{},",_gift[index-5]);
         println!("{},",_gift[index-6]);
         println!("{},",_gift[index-7]);
         println!("{},",_gift[index-8]);
         println!("{}.",_gift[index-9]);
         println!("{}.",_gift[index-10]);
         println!("and {}.",_gift[index-11]);
        }

        else{
            println!("Default");
        }

        index +=1;

       // println!("Index: {index}")
    }
}
