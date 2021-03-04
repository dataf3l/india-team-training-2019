

enum Mode  {
   One,
   Two,
   Tree,
}



fn printm(mode:Mode){
   
   match mode {
       Mode::One => {
           println!("UNO");
       }
       Mode::Two => {
           println!("DOS");
       }
       Mode::Tree => {
           println!("TRES");
       }
   }
}

/*
fn printm(mode:i32){

   if mode == 1 {
       println!("UNO");
   }
   if mode == 2 {
       println!("DOS");
   }
}*/

fn f(a:&str) {
   println!("A={}",a);
}
fn maybe_print(d:Option<String>){

  match d {
     Some(ds) => {
        println!("{}",ds);
     }
     None => {
        println!("NO INPUT");
     }
  }
}
fn main() {
//    let a = String::from("AAA");
//    let b = 0;
//    f(&a);

//    println!("A={}, B={}",a,b);

    let d:Option<String> = Some(String::from("x"));
    maybe_print(d);


    //printm(Mode::One);
    //printm(Mode::Two);


    //println!("{}", a + b);
}
