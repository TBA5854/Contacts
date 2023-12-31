use csv;
use std::io::{self, Write};
static mut ONCE: i8 = 0;
fn main() {
    back();
}

fn del() {
    let mut csv_reader = csv::Reader::from_path("./resources/contacts.csv").expect("file couldnt be opened");
    let mut csv_writer = csv::Writer::from_path("./resources/contacts.csv").expect("could not open file");
    let mut name = String::new();
    let mut number = String::new();
    let mut _sno = String::new();
    println!("Enter the name of contact to delete : ");
    io::stdin().read_line(&mut name).expect("read err");
    name = name.trim().to_string();
    println!("Enter the number of contact to delete : ");
    io::stdin().read_line(&mut number).expect("read err");
    number = number.trim().to_string();
    for result in csv_reader.records() {
        let record = result.expect("file couldnt be read");
        let sno = &record[0];
        let name1 = &record[1];
        let number1 = &record[2];
        if name1 == &name && number1 == &number {
            continue;
        }
        csv_writer.write_record(&[sno.to_string(), name.to_string(), number.to_string()]);
    }
    csv_writer.flush();
}

fn write_file() {
    let mut csv_writer = csv::Writer::from_path("./resources/contacts.csv").expect("could not open file");
    let mut name = String::new();
    let mut number = String::new();
    println!("Enter the number of contacts to append: ");
    let mut num_contacts = String::new();
    io::stdin().read_line(&mut num_contacts).expect("read err");
    let num_contacts: usize = num_contacts.trim().parse().expect("Invalid input");

    for _ in 0..num_contacts {
        println!("Enter name : ");
        io::stdin().read_line(&mut name).expect("read err");
        name = name.trim().to_string();
        println!("Enter number : ");
        io::stdin().read_line(&mut number).expect("read err");
        number = number.trim().to_string();
        let sno = count();
        csv_writer.write_record(&[sno.to_string(), name, number]);
    }

    csv_writer.flush();
}

fn start() {
	print!("\n\tContacts\n\n");
	print!("1.Read Contacts\n");
	print!("2.Append Contacts\n");
	print!("3.Count number of contacts saved\n");
	print!("4.Export Contacts\n");
	print!("5.Delete a contact\n");
	print!("6.Reset/Create CSV\n");
	print!("7.Go back to menu\n");
	print!("8.Exit\n");
	print!(">>");
    let mut ch: i32;
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("read err");
        match input.trim().parse() {
            Ok(num) => {
                ch = num;
                break;
            }
            Err(_) => println!("Please type a number !!!"),
        }
    }
	match ch {
        1 =>
	{print!("\n\tRead Contacts\n\n");
read_file();
	print!("\nOperation Comleted\n");
	back();
	start();}
	 2 =>{
	print!("\n\tAppend Contacts\n\n");
write_file();
	print!("\nOperation Comleted\n");
	back();
	start();
	}
	 3=>{
	print!("\n\tCount Contacts\n\n");
    print!("Number of contacts are :{}\n",count());
	print!("\nOperation Comleted\n");
	back();
	start();
	}
	 4=>{
	print!("\n\tExport Contacts\n\n");
	vcf_writer();
	print!("\nOperation Comleted\n");
	back();
	start();
	}
	 5=>{
	print!("\n\tDelete a Contact\n\n");
    del();
	print!("\nOperation Comleted\n");
	back();
	start();
	}
	 6=>{
	menu();
	}
	7=>{
	end();}
	_=>{
	print!("Invalid input , program is exiting !!!");
	std::process::exit(0);}
	}
}

fn vcf_writer(){
    let mut rdr = csv::Reader::from_path("contacts.csv").expect("couldnt open file");
    let mut file = std::fs::OpenOptions::new().append(true).open("contact.vcf").expect("couldnt open file");

    for result in rdr.records() {
        let record = result.expect("couldnt parse");
        let name = &record[0];
        let phone = &record[1];

        write!(file, "BEGIN:VCARD\n").expect("couldnt write");
        write!(file, "VERSION:3.0\n").expect("couldnt write");
        write!(file, "N:{}\n", name).expect("couldnt write");
        write!(file, "TEL:{}\n", phone).expect("couldnt write");
        write!(file, "END:VCARD\n").expect("couldnt write");
    }
}


fn count() -> i32 {
let mut csv_reader = csv::Reader::from_path("./resources/contacts.csv").expect("file couldnt be opened");
let mut count=0;
for _ in csv_reader.records() {
count+=1;
}
count
}

fn end() {
		println!("\t\tThanks for Trying this program \n\n\t\t  A Program by TBA5854\n");
		std::process::exit(0);
}

fn back() {
    loop{
    print!("\nType \"back\" to go previous page\n>>");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("read err");
    if input.trim() == "back" {
        return;
    }
    else {
        println!("\n\nPlease type back !!!\n");
    }}
}

fn menu() {
if unsafe { ONCE } == 0 {
    println!("Hello fellow user, you have started the Contacts Program!\n");
    unsafe{ONCE += 1;}
}
println!("\n\tMenu\n\n");
println!("Use the resources below and type 4 to start\n(Enter number to choose the option)\n\n");
println!("1. Disclaimer\n");
println!("2. Info\n");
println!("3. Contact Developer\n");
println!("4. Start\n");
println!("5. Exit\n");
println!(">>");
let mut ch: i32;
loop {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("read err");
    match input.trim().parse() {
        Ok(num) => {
            ch = num;
            break;
        }
        Err(_) => println!("Please type a number !!!"),
    }
}
    match ch
    {
        1 => {println!("\n\tDisclaimer\n\n");
        println!("-->This program works the best when\n the user follows instructions\n-->Check Info for instructions");
        back();
        menu();},
        2 => {println!("\n\tInfo\n\n");
        info();
        back();
        menu();},
        3 => {println!("\n\tContact Developer\n\n");
        println!("For any querries or reporting bugs , contact TBA58584 via github or twitter\n");
        println!("www.github.com/TBA5854\n");
        println!("www.twitter.com/TBA5854\n");
        back();
        menu();},
        4 => {start();},
        5 => {end();},
        _ => {println!("Invalid input , program is exiting !!!");
        std::process::exit(0);}
    }
}

fn info() {
	println!("1.This is a Contacts program\n");
	println!("2.This program can store , count , read contacts in csv file and them export all as vcf\n");
	println!("3.Only enter option number\n");
	println!("4.Never enter a alphabet unless asked to do so\n");
	println!("5.Never type space in names\n");
	println!("6.For 1st time use , create CSV before\n doing anything ");
	println!("7.If Contacts become corrupted , try resetting csv");
}

fn read_file() {
    let mut csv_reader = csv::Reader::from_path("./resources/contacts.csv").expect("file couldnt be opened");
    
    for result in csv_reader.records() {
        let record = result.expect("file couldnt be read");
        let sno = &record[0];
        let name = &record[1];
        let number = &record[2];
        println!("{} {} {}", sno, name, number);
    }   
}