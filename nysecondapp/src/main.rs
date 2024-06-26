
mod back_of_house{
    pub struct Breakfast{
        pub toast:String,
        seasonal_fruit:String,
    }
    impl Breakfast{
        pub fn summer(toast:&str)->Breakfast{
            Breakfast{
                toast:String::from(toast),
                seasonal_fruit:String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant(){
    let mut meal=back_of_house::Breakfast::summer("Rye");
    meal.toast=String::from("wheat");
    println!("I'd like {} toast please", meal.toast);
}
fn pro_common_collections(){
    let mut v=vec![100,32,57];
    for i in &mut v {
        *i+=50;
        println!("{i}");
    }
    for i in &mut v {
        println!("{i}");
    }
}

fn pro_using_enum_to_store_multipleTypes(){
    #[derive(Debug)]
    enum SpreadSheetCell{
        Int(i32),
        Float(f64),
        Text(String),
    }
    let low=vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Float(10.10),
        SpreadSheetCell::Text(String::from("blue")),
    ];
    println!("{:?},{:?},{:?}",low[0],low[1],low[2]);
}
fn pro1(){
    let s1=String::from("hello, ");
    let s2=String::from("world!");
    let s3=s1.clone()+&s2;
    let s4=s1+&s2;
    println!("{s3}  ");
    println!("{s4}");
}
fn pro2(){
    use std::collections::HashMap;
    let text="hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace(){
        let count=map.entry(word).or_insert(0);
        *count+=1;
    }
    println!("{:?}",map);
}
fn pro3(){
    use std::fs::File;
    use std::io::ErrorKind;
    let greeting_file=File::open("hello.txt").unwrap_or_else(|error|{
        if error.kind()==ErrorKind::NotFound{
            File::create("hello.txt").unwrap_or_else(|error|{
                panic!("Problem creating the file: {:?}", error);
            })
        } else{
            panic!("Problem opening the file: {:?}", error);
        }
    });
}
// fn largest(list:&[i32])->&i32{
//     let mut largest=&list[0];
//     for item in list{
//         if item>largest{
//             largest=item;
//         }
//     }
//     largest
// }

//fn largest<T>(list:&[T])->&T{
    // let mut largest=&list[0];
    // for item in list{
    //     if item>largest{
    //         largest=item;
    //     }
    // }
    // largest
//}
fn pro4(){
    let number_list=vec![34,50,25,100,65];
    // let mut largest=&number_list[0];
    // for number in &number_list{
    //     if number>largest{
    //         largest=number;
    //     }
    // }
    //let result=largest(&number_list);
    //println!("The largest number is {}",result);
}
fn pro5(){
    let number_list=vec![34,50,25,100,65];
    //let result=largest(&number_list);
    //println!("The largest number is {}",result);
}

struct Point<A,B>{
    x:A,
    y:B,
}
impl<A,B> Point<A,B>{
    fn x(&self)->&A{
        &self.x
    }
    fn y(&self)->&B{
        &self.y
    }
    fn mixup<A1,B1>(self,other:Point<A1,B1>)->Point<A,B1>{
        Point{
            x:self.x,
            y:other.y,
        }
    }
}



fn pro6(){
    let both_integer=Point{x:5,y:10};
    let both_float=Point{x:5.1,y:10.1};
    let mix_int_float=Point{x:3, y:8.1};
    println!("both integer:{},{}",both_integer.x,both_integer.y);
    println!("both float:{},{}",both_float.x,both_float.y);
    println!("mix int and float:{},{}",mix_int_float.x(),mix_int_float.y());
    let p1=Point{x:3,y:10.4};
    let p2=Point{x:"hello",y:'c'};
    let p3=p1.mixup(p2);
    println!("p3.x={},p3.y={}", p3.x,p3.y);
}

fn pro7(){
    let integer=Some(5);
}

use std::fmt::Display;

struct Pair<T>{
    x:T,
    y:T,
}
impl<T> Pair<T>{
    fn new(x:T,y:T)->Self{
        Self{x,y}
    }
}
impl<T:Display+PartialOrd> Pair<T>{
    fn cmp_display(&self){
        if self.x>=self.y{
            println!("The largest number is x={}", self.x);
        } else{
            println!("The largest number is y={}", self.y);
        }
    }
}
fn pro8(){
    let p1=Pair{x:5.1,y:8.5};
    let p2=Pair{x:6.1,y:2.2};
    p1.cmp_display();
    p2.cmp_display();
}
#[cfg(test)]
mod tests{
    #[test]
    fn exploration(){
        assert_eq!(2+2,4);
    }
    #[test]
    fn another(){
        panic!("Make this test fail");
    }
}

//#[derive(Debug)]
struct Rectangle{
    width:u32,
    height:u32,
}
impl Rectangle{
    fn can_hold(&self, other:&Rectangle)->bool {
        self.width>other.width&&self.height>other.height
    }
}
#[cfg(test)]
mod tests1{
    use super::*;
    #[test]
    fn larger_can_hold_smaller(){
        let larger=Rectangle{
            width:8,
            height:7,
        };
        let smaller=Rectangle{
            width:5,
            height:1,
        };
        assert!(larger.can_hold(&smaller));
    }
}

fn pro9(){
    use std::env;
    let args:Vec<String>=env::args().collect();
    let query=&args[1];
    let file_path=&args[2];
    println!("Searching for {}", query);
    println!("In file {}", file_path);
}
fn main(){
    //pro_using_enum_to_store_multipleTypes();
    //pro1();
    //pro2();
    //pro3();
    //pro4();
    //pro5();
    //pro6();
    //pro7();
    //pro8();
    pro9();
}


