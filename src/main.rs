extern crate reqwest;
extern crate regex;
use serde_json::Value;
use regex::Regex;
use std::fs::{write,read_to_string};
use std::path::Path;
use std::env;

fn main() {
    loop{
        run();
    }   
}

fn run(){
    let url = "https://raw.githubusercontent.com/dracarys18/Kernel_Tracker/master/data.json";
    let resp = reqwest::blocking::get(url).unwrap();
    let json: Value = serde_json::from_str(&resp.text().unwrap()).unwrap();
    let longterm = json.get("longterm").unwrap();
    let longterm_array = longterm.as_array().unwrap();
    let latest_version:String = get_the_latest_version(longterm_array);

    //Write if the file doesn't exist
    if !Path::new("data.txt").exists(){
        write_to_file(&latest_version);
    }

    //Get the bot token from env variables
    let bot_token = &env::var("BOT_TOKEN").expect("Failed to find BOT_TOKEN. export the token using export BOT_TOKEN=value");

    //Get the current version from file and compare with latest version
    let current_version:String = read_to_string("data.txt").unwrap();
    if latest_version.ne(&current_version){
        write_to_file(&latest_version);
        println!("Latest Version of the kernel is {}",&latest_version);
        let href = format!("https://git.kernel.org/stable/h/v{}",&latest_version);
        let text = format!("<strong>New 4.14 Kernel has been released \n</strong><a href='{}'>{}</a>\n@kernel_tracker",&href,&latest_version);
        post_to_telegram(&text, bot_token);
    }
    else{
        println!("Nothing to do Goodnight!");
    }
}
fn write_to_file(version:&String){
    write("data.txt",version).expect("Cant write the file");
}

fn get_the_latest_version(json_value:&Vec<serde_json::Value>)->String{
    //Only consider if the string in the array starts with 4.14
    let re = Regex::new(r"^4.14").unwrap();
    
    let mut res:&str = "";
    for i in json_value{
        let to_string = i.as_str().unwrap();
        if re.is_match(to_string){
          res = to_string;
        }
    }
    String::from(res)
}

fn post_to_telegram(text:&str,token:&String){
    let params = [
        ("chat_id", "-1001220351473"),
        ("text", text),
        ("parse_mode", "HTML"),
        ("disable_web_page_preview", "yes"),
    ];
    let url = format!("https://api.telegram.org/bot{}/sendMessage",token);
    let m = reqwest::blocking::Client::new();
    m.post(url).form(&params).send().unwrap();
}
