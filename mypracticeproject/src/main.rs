use std::cell::Ref;
use std::cell::RefCell;
use std::collections::btree_map::Values;
use std::env;
use std::fs;
use std::fs::Metadata;
use std::rc::Rc;
use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use std::sync::{Arc,Mutex};

#[derive(Debug,PartialEq,Copy,Clone)]
enum ShirtColor{
    Red,
    Blue,
}
struct Inventory{
    shirts:Vec<ShirtColor>,
}
impl Inventory{
    fn giveway(&self, user_preference:Option<ShirtColor>)->ShirtColor{
        user_preference.unwrap_or_else(|| self.most_stocked())
    }
    fn most_stocked(&self)->ShirtColor{
        let mut num_red=0;
        let mut num_blue=0;
        for color in &self.shirts{
            match color{
                ShirtColor::Red=>num_red+=1,
                ShirtColor::Blue=>num_blue+=1,
            }
        }
        if num_red>num_blue{
            ShirtColor::Red
        } else{
            ShirtColor::Blue
        }
    }
}
fn pro1(){
    let file_path="poem.txt";
    println!("In file {}", file_path);
    let contents=fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    println!("With text:\n{contents}");
}
fn pro2(){
    let store=Inventory{
        shirts:vec![ShirtColor::Blue,ShirtColor::Red, ShirtColor::Blue],
    };
    let user_pref1=Some(ShirtColor::Red);
    let giveway1=store.giveway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}", user_pref1, giveway1
    );
    let user_pref2=None;
    let giveway2=store.giveway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}", user_pref2, giveway2
    );    
}
fn pro3(){
    let list=vec![1,2,3];
    println!("Before defining closure:{:?}",list);
    let only_borrows=|| println!("From closure:{:?}", list);
    println!("Before calling closure:{:?}", list);
    only_borrows();
    println!("After calling closure:{:?}", list);
}
#[derive(Debug)]
struct Rectangle{
    width:u32,
    height:u32,
}
fn pro4(){
    let mut list=[
        Rectangle{width:10, height:1},
        Rectangle{width:3, height:5},
        Rectangle{width:7, height:12},
    ];
    list.sort_by_key(|r| r.width);
    println!("{:#?}",list);
}
fn pro5(){
    let mut list=[
        Rectangle{width:10, height:1},
        Rectangle{width:3, height:5},
        Rectangle{width:7, height:12},
    ];
    let mut num_sort_operations=0;
    list.sort_by_key(|r|{
        num_sort_operations+=1;
        r.width
    });
    println!("{:#?}, sorted in {num_sort_operations} operations", list);
}
fn pro6(){
    let v1=vec![1,2,3];
    let v1_iter=v1.iter();
    let total:i32=v1_iter.sum();
    println!("sum={total}");
    assert_eq!(total,6);
}

struct CustomSmartPointer{
    data:String,
}
impl Drop for CustomSmartPointer{
    fn drop(&mut self){
        println!("Dropping CustomSmartPointer with data {}",self.data);
    }
}
fn pro7(){
    let c=CustomSmartPointer{
        data:String::from("my stuff"),
    };
    let d=CustomSmartPointer{
        data:String::from("other stuff"),
    };
    drop(c);
    println!("CustomSmartPointers created");
}


use std::rc::Weak;

#[derive(Debug)]
struct Node{
    value:i32,
    parent:RefCell<Weak<Node>>,
    children:RefCell<Vec<Rc<Node>>>,
}

fn pro8(){
    let leaf=Rc::new(Node{
        value:3,
        parent:RefCell::new(Weak::new()),
        children:RefCell::new(vec![]),
    });
    println!(
        "leaf strong={},weak={}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
    {
        let branch=Rc::new(Node{
            value:5,
            parent:RefCell::new(Weak::new()),
            children:RefCell::new(vec![Rc::clone(&leaf)]),
        });
        *leaf.parent.borrow_mut()=Rc::downgrade(&branch);
        println!(
            "branch strong={}, weak={}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );
        println!(
            "leaf strong={},weak={}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
        println!("leaf parent={:#?}", leaf.parent.borrow().upgrade());
    }
    println!("leaf parent={:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong={},weak={}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}
fn pro9(){
    let handle=thread::spawn(||{
        for i in 1..10{
            println!("hi number {} from the spawned thread!",i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..5{
        println!("hi number {} from the main thread!",i);
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap();
}
fn pro10(){
    let (tx,rx)=mpsc::channel();
    thread::spawn(move||{
        let val=String::from("hi");
        tx.send(val.clone()).unwrap();
        println!("val is {}",val);
    });
    let received=rx.recv().unwrap();
    println!("Got:{}",received);
}
fn pro11(){
    let m=Mutex::new(5);
    {
        let mut num=m.lock().unwrap();
        *num=6;
    }
    println!("m={:?}",m);
}
fn pro12(){
    let counter=Arc::new(Mutex::new(0));
    let mut handles=vec![];
    for _ in 0..10{
        let counter=Arc::clone(&counter);
        let handle=thread::spawn(move ||{
            let mut num=counter.lock().unwrap();
            *num+=1;
        });
        handles.push(handle);
    }
    for handle in handles{
        handle.join().unwrap();
    }
    println!("Result:{}",*counter.lock().unwrap());
}
fn main() {
    println!("Hello, world!");
    //pro1();
    //pro2();
    //pro3();
    //pro4();
    //pro5();
    //pro6();
    //pro7();
    //pro8();
    //pro9();
    //pro10();
    //pro11();
    pro12();
}

