use reqwest::header::{HeaderMap};
use reqwest::{Client};
use std::fs::File;
use std::io::{BufRead, BufReader, stdout};
use crossterm::{execute, terminal::{Clear, ClearType}};
use std::thread;
use std::time::Duration;
use std::env;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
	let args: Vec<String> = env::args().collect();
	if args.len() > 1 && args[1] == "refresh" {
		loop {
			//clean screen
			execute!(stdout(), Clear(ClearType::All)).unwrap();
			curl().await?;
			thread::sleep(Duration::from_secs(1));
		}
	} else {
		curl().await?;
	}
	Ok(())
}

async fn curl() -> Result<(), reqwest::Error> {
	//read config file
	let file = File::open("config_sina.ini").expect("cannot find the file!");
	let reader = BufReader::new(file);
	let mut codes = String::from("");
	
	for line in reader.lines() {
		if codes == "" {
			codes.push_str(&line.unwrap());
		} else {
			codes.push_str(",");
			codes.push_str(&line.unwrap());
		}
	}
	//println!("codes = {}", codes);
	
	let mut url = String::from("http://hq.sinajs.cn/list=");
	url.push_str(codes.as_str());
	//println!("{}", url);
	
	let client = Client::new();
	let mut headers = HeaderMap::new();
	//headers.insert(USER_AGENT, "reqwest".parse().unwrap());
	headers.insert("Referer", "https://finance.sina.com.cn".parse().unwrap());
	let response = client.get(url).headers(headers).send().await?.text().await?;
	//print_type_of(&response);
	//println!("{:?}", response);

	println!("{:<13}|{:<12}|{:<12}|{:<12}|{:<12}|{:<14}|{:<14}|{:<14}|{:<14}", "名称", "当前", "涨幅(%)", "昨收", "今开", "当日最高", "当日最低", "成交数(手)", "成交金额(万元)");
	let stocks: Vec<&str> = response.split(";").collect();
	for item in stocks.iter() {
		//println!("{:?}, {}", item, item.len());
		if item.len() > 100 {
			let pos = item.find("\"");
			//println!("{:?}", pos);
			if pos != None {
				let section = &item[pos.unwrap()+1..];
				let data: Vec<&str> = section.split(",").collect();
				let init_price = data[1].parse::<f64>().unwrap();
				let yesterday_price = data[2].parse::<f64>().unwrap();
				let current_price = data[3].parse::<f64>().unwrap();
				let top_price = data[4].parse::<f64>().unwrap();
				let end_price = data[5].parse::<f64>().unwrap();
				let deal_num = data[8].parse::<f64>().unwrap();
				let deal_amount_1000 = data[9].parse::<f64>().unwrap();
				
				let surplus = format!("{:.2}", current_price - yesterday_price).parse::<f64>().unwrap();
				let mut rate = format!("{:.2}", surplus * 100.0 / yesterday_price);
				rate.push_str("%");
				let deal_num = format!("{:.2}", deal_num / 100.0).parse::<f64>().unwrap();
				let deal_amount = format!("{:.2}", deal_amount_1000 / 10000.0).parse::<f64>().unwrap();
				if data[0].len() == 12 {
					println!("{:<11}|{:<14}|{:<14}|{:<14}|{:<14}|{:<18}|{:<18}|{:<18}|{:<15}", data[0], current_price, rate, yesterday_price, init_price, top_price, end_price, deal_num, deal_amount);
				} else {
					println!("{:<12}|{:<14}|{:<14}|{:<14}|{:<14}|{:<18}|{:<18}|{:<18}|{:<15}", data[0], current_price, rate, yesterday_price, init_price, top_price, end_price, deal_num, deal_amount);
				}
			}
		}
	}
	Ok(())
}

fn print_type_of<T>(_: &T) {
	println!("{}", std::any::type_name::<T>())
}
