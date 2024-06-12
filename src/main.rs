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
            let _output = Command::new("curl").args([&format!("{}/new", args.host), "-H", "Content-Type: application/json", "-d", &format!("{{\"ctx\" : \"{}\", \"user\": \"{}\", \"password\": \"{}\"}}", args.ctx.unwrap(), args.user, args.password), "-v"]).output().unwrap().stdout;
        }
        else{
            println!("CTX is needed for this command");
        }
    }
    else if args.cmd == "l"{
        let mut output = Command::new("curl").args([&format!("{}/list", args.host), "-H", "Content-Type: application/json", "-d", &format!("{{\"user\": \"{}\", \"password\": \"{}\"}}", args.user, args.password)]).output().unwrap().stdout;
        output.remove(0);
        output.pop();
        let mut data = false;
        let mut c = 0;
        for i in output{
            if i as char == '"' && !data{
                data = true;
            }
            else if i as char == '"' && data{
                data = false;
                c+=1;
                if c == 2{
                    c = 0;
                    println!("");
                }
                else{
                    print!("  ");
                }
            }
            else if data{
                print!("{}", i as char);
            }
            
        }
    }
    else if args.cmd == "r"{
        if args.ctx.is_some(){
            let mut _output = Command::new("curl").args([&format!("{}/delete", args.host), "-H", "Content-Type: application/json", "-d", &format!("{{\"id\": \"{}\", \"user\": \"{}\", \"password\": \"{}\"}}", args.ctx.unwrap() ,args.user, args.password)]).output().unwrap().stdout;
        }
        else{
            println!("CTX is needed for this command");
        }
    }


}
