
//real	0m0.009s
//user	0m0.004s
//sys	0m0.003s


// runtime: missing file -> it doesn't build
// error message is      -> forced to handle error case due to match semantics
// data is invalid       -> forced to handle error case due to match semantics
//undefined * is OK      -> it doesn't build
//null pointer           -> it doesn't build
// unhandled swtich case -> it doesn't build

/*
enum InvoiceType {
    Pending,
    Cancelled,
    Paid
}
struct Point {
    x:i64,
    y:i64,
    z:i64,
}
*/
//fn distance(a:Point,b:Point) -> i64{
//    let p1 = (a.x + a.x) * (a.x + a.x); 
//    let p2 = (a.y + a.y) * (a.x + a.x); 
//    let p3 = (a.x + a.x) * (a.x + a.x); 
//    return p1+p2+p3;
//}
fn main() {
    //let d= distance(Point{x:1,y:1,z:1},Point{x:1,y:1,z:1});
    //println!("{}",d);
    
    //let x:Option<String> = None;

    //let d = x.split(" ");

    let file = include_str!("../../input.txt");
    /*
    let invoice_status = InvoiceType::Cancelled;

    match invoice_status {
        InvoiceType::Pending => {
            println!("pending");
        }
        InvoiceType::Cancelled => {
            println!("cancelled");
        }
        InvoiceType::Paid => {
            println!("paid");
        }
    }
    */

    //let nums:Vec<i64> = vec![10,20,30];
    for _x in 1..1000 {
        let lines = file.split("\n");
        let mut sum:i64 = 0;
        for line in lines {
            if line == "" {
                continue;
            }
            let value = line.parse::<i64>();
            match value {
                Ok(v) => {
                    sum += v;
                }
                Err(e) => {
                    println!("Invalid value:{}:{}",line,e);
                }

            }
        }
        println!("SUM:{}",sum);
    }
}
