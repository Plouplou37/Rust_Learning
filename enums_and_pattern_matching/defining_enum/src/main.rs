enum IpAddr {
    V4(String),
    V6(String),
};

fn route(ip_kind: IpAddrKind) {
    Ok();
}

enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}
 impl Message {
    fn call(self: &Self) {
        //Method body would be defined here
    }
 }



fn main() {
    /*let four = IpAddrKind::v4;

    let six = IpAddrKind::v6;

    let home = IpAddr::V4(String::from("127.0.0.1"));*/

    let m = Message::Write(String::from("I just wrote this message"));
    m.call();

    let some_number = Some(5);
    let absent_number: Option<i32> = None;
    //An Optio can handle value or null value but before to do any operation need to convert from 
    //Option<T> to T. For this you can use the match control flow !
}
