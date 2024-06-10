use clap::Parser;
use std::process::Command;

#[derive(Parser)]
struct Cli{
    host: String,
    user: String,
    password: String,
    cmd: String,
    ctx: Option<String>,
}


fn main() {
    let args = Cli::parse();
    if args.cmd == "a"{
        if args.ctx.is_some(){
            let _output = Command::new("curl").args([&format!("http://{}/new", args.host), "-H", "Content-Type: application/json", "-d", &format!("{{\"ctx\" : \"{}\", \"user\": \"{}\", \"password\": \"{}\"}}", args.ctx.unwrap(), args.user, args.password), "-v"]).output().unwrap().stdout;
        }
        else{
            println!("CTX is needed for this command");
        }
    }
    else if args.cmd == "l"{
        let mut output = Command::new("curl").args([&format!("http:/{}/list", args.host), "-H", "Content-Type: application/json", "-d", &format!("{{\"user\": \"{}\", \"password\": \"{}\"}}", args.user, args.password)]).output().unwrap().stdout;
        output.remove(0);
        output.pop();
        for i in output{
            print!("{}", i as char);
        }
    }


}
