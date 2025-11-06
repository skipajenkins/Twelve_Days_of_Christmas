# 🎶 Twelve Days of Christmas (Rust)

A Rust program that prints the lyrics to the classic Christmas carol **“The Twelve Days of Christmas.”**  
Each verse builds upon the previous one, adding a new gift for every day — following the traditional structure of the song.

---

## ⚙️ Setting Up the Environment

Before running the program, make sure **Rust** and **Cargo** are installed on your system.

### 🦀 Step 1: Check for Rust Installation
```bash
rustc --version
```
If Rust isn’t installed, you can install it using rustup:

```bash
curl https://sh.rustup.rs -sSf | sh
```
After installation, confirm:

```bash
rustc --version
cargo --version
```

### 📁 Step 2: Create the Project Folder
Create a new Rust project using Cargo:

---

```bash
cargo new Twelve_Days_of_Christmas
Then navigate to the project directory:
```

---

```bash
cd Twelve_Days_of_Christmas
Replace the contents of src/main.rs with the following code:
```

---

## 🎵 Rust Code
rust

```bash
fn main() {
    twelve_days_of_christmas();
}

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
         println!("{}.",_gift[index-9]); println!("{}.",_gift[index-10]);
         println!("and {}.",_gift[index-11]);
        }

        else{
            println!("Default");
        }

        index +=1;
    
    //CHATGPT'S VERSION
    //     while index < 12 {
    //     println!("On the {} day of Christmas, my true love gave to me", _days[index]);
        
    //     // Print gifts in reverse order up to the current day
    //     let mut gift_index = index + 1;
    //     while gift_index > 0 {
    //         gift_index -= 1;
    //         if index != 0 && gift_index == 0 {
    //             println!("and {}.", _gift[gift_index]);
    //         } else {
    //             println!("{},", _gift[gift_index]);
    //         }
    //     }

    //     println!();
    //     index += 1;
    // }

       // println!("Index: {index}")
    }
}
```

---

### ▶️ Step 3: Build and Run
🧱 Build
```bash
Copy code
cargo build
```
🎬 Run
```bash
Copy code
cargo run
```

---

## 🧾 Example Output

```bash
On the first day of Christmas, my true love gave to me
A partridge in a pear tree.

On the second day of Christmas, my true love gave to me
Two turtle doves,
and A partridge in a pear tree.

On the third day of Christmas, my true love gave to me
Three French hens,
Two turtle doves,
and A partridge in a pear tree.
...
```

---

## 🧠 Concept Breakdown
Concept	Explanation
Arrays ([&str; 12])	Used to store the 12 days and their corresponding gifts
Loops (while)	Iterates through each day and prints all gifts up to that day
Reverse indexing	Allows printing the previous gifts for each verse
Conditional logic	Handles when to print "and" in front of the last gift

---

## 🚀 Future Enhancements
Allow user input to print up to a specific day.

Add colorized terminal output using the colored crate.

Support custom carol variations.

---

## 🦀 Built With
Rust

Cargo

---

## 📄 License
This project is open-source and available under the MIT License.

