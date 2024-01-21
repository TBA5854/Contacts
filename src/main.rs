use csv;
use std::{io::{self, Write}, path::Path};
static mut ONCE: i8 = 0;
fn main() {
    let mut path_of_oper=String::new();
    loop {
        print!("Enter Path to store files \t(DON'T USE ~)\n>>");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut path_of_oper).expect("read err");
    path_of_oper=path_of_oper.trim().to_string();
    if std::path::Path::new(&path_of_oper).exists()
    {
        std::env::set_current_dir(path_of_oper).expect("couldnt change path");
        break;}
        else {
            println!("Path Doesnt exists !!!");
            path_of_oper=String::new();
        }
    }
    menu();
}

fn edit() -> bool{
    let mut edited:bool = false;
    println!("Enter the SNO of the contact to edit: ");
    let mut sno_edit = String::new();
    io::stdin().read_line(&mut sno_edit).expect("read err");
    sno_edit=sno_edit.trim().to_string();
    if sno_edit.parse::<i32>().is_ok() && sno_edit.parse::<i32>().expect("it shouldnt raise err")>count()
    {return false;}
    let mut new_name=String::new();
    println!("Enter the new Name: ");
    io::stdin().read_line(&mut new_name).expect("read err");
    new_name=new_name.trim().to_string();
    let mut new_num =String::new();
    println!("Enter the new Number: ");
    io::stdin().read_line(&mut new_num).expect("read err");
    new_num=new_num.trim().to_string();
    let file_path_source = "./contacts.csv";
    let file_path_target = "./contacts_temp.csv";
    std::fs::File::create("./contacts_temp.csv").expect("couldnt create file");
    reset_or_create("contacts_temp");
    {   
        let file_source = std::fs::OpenOptions::new().read(true).open(file_path_source).expect("couldnt open file");
        let file_target = std::fs::OpenOptions::new().append(true).open(file_path_target).expect("couldnt open file");
        let mut reader_source = csv::Reader::from_reader(file_source);
        let mut writer_target = csv::Writer::from_writer(file_target);
        for record in reader_source.records()
        {
            let record=record.expect("couldnt read");
            if sno_edit==record[0]
            {
                edited=true;
            writer_target.write_record(&[record[0].to_string(),new_name.clone(),new_num.clone()]).expect("couldnt write");
            writer_target.flush().expect("couldnt write");
            continue;
            }
            writer_target.write_record(&record).expect("couldnt write");
            writer_target.flush().expect("couldnt write");
        }
    }
    std::fs::remove_file("./contacts.csv").expect("couldnt remove temp file");
    std::fs::File::create("./contacts.csv").expect("couldnt create file");
    reset_or_create("contacts");
    {
        let file_source = std::fs::OpenOptions::new().read(true).open(file_path_target).expect("couldnt open file");
        let file_target = std::fs::OpenOptions::new().append(true).open(file_path_source).expect("couldnt open file");
        let mut reader_source = csv::Reader::from_reader(file_source);
        let mut writer_target = csv::Writer::from_writer(file_target);
        for record in reader_source.records()
        {
            let record=record.expect("couldnt read");
            let sno = count()+1;
            let name=&record[1];
            let number=&record[2];
            writer_target.write_record(&[sno.to_string(), name.to_string(), number.to_string()]).expect("couldnt write");
            writer_target.flush().expect("couldnt write");
        }
    }
    std::fs::remove_file("./contacts_temp.csv").expect("couldnt remove temp file");
    return edited;
}

fn reset_or_create(s:&str) {
    let path=format!("./{s}.csv");
    let file_obj = std::fs::OpenOptions::new().write(true).open(path).expect("couldnt open file");
    let mut csv_writer = csv::Writer::from_writer(file_obj);
    csv_writer.write_record(["SNO".to_string(), "NAME".to_string(), "NUMBER".to_string()]).expect("couldnt write");
    csv_writer.flush().expect("couldnt flush changes");
}

fn del() -> bool {
    let mut deletion:bool = false;
    println!("Enter the SNO of the contact to delete: ");
    let mut sno_del = String::new();
    io::stdin().read_line(&mut sno_del).expect("read err");
    sno_del=sno_del.trim().to_string();
    if sno_del.parse::<i32>().is_ok() && sno_del.parse::<i32>().expect("it shouldnt raise err")>count()
    {return false;}
    let file_path_source = "./contacts.csv";
    let file_path_target = "./contacts_temp.csv";
    std::fs::File::create("./contacts_temp.csv").expect("couldnt create file");
    reset_or_create("contacts_temp");
    {   
        let file_source = std::fs::OpenOptions::new().read(true).open(file_path_source).expect("couldnt open file");
        let file_target = std::fs::OpenOptions::new().append(true).open(file_path_target).expect("couldnt open file");
        let mut reader_source = csv::Reader::from_reader(file_source);
        let mut writer_target = csv::Writer::from_writer(file_target);
        for record in reader_source.records()
        {
            let record=record.expect("couldnt read");
            if sno_del==record[0]{deletion=true;continue;}
            writer_target.write_record(&record).expect("couldnt write");
            writer_target.flush().expect("couldnt write");
        }
    }
    std::fs::remove_file("./contacts.csv").expect("couldnt remove temp file");
    std::fs::File::create("./contacts.csv").expect("couldnt create file");
    reset_or_create("contacts");
    {
        let file_source = std::fs::OpenOptions::new().read(true).open(file_path_target).expect("couldnt open file");
        let file_target = std::fs::OpenOptions::new().append(true).open(file_path_source).expect("couldnt open file");
        let mut reader_source = csv::Reader::from_reader(file_source);
        let mut writer_target = csv::Writer::from_writer(file_target);
        for record in reader_source.records()
        {
            let record=record.expect("couldnt read");
            let sno = count()+1;
            let name=&record[1];
            let number=&record[2];
            writer_target.write_record(&[sno.to_string(), name.to_string(), number.to_string()]).expect("couldnt write");
            writer_target.flush().expect("couldnt write");
        }
    }
    std::fs::remove_file("./contacts_temp.csv").expect("couldnt remove temp file");
    return deletion;
}

fn write_file() {
    let file_obj = std::fs::OpenOptions::new().append(true).open("./contacts.csv").expect("couldnt open file");
    let mut csv_writer = csv::Writer::from_writer(file_obj);
    println!("Enter the number of contacts to append: ");
    let mut num_contacts = String::new();
    io::stdin().read_line(&mut num_contacts).expect("read err");
    let num_contacts: usize = num_contacts.trim().parse().expect("Invalid input");
    for _ in 0..num_contacts {
        println!("Enter name : ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("read err");
        let name = input.trim().to_string();
        println!("Enter number : ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("read err");
        let number = input.trim().to_string();
        let sno = count()+1;
        csv_writer.write_record(&[sno.to_string(), name, number]).expect("couldnt write");
        csv_writer.flush().expect("couldnt flush changes");
    }
}

fn start() {
    let precheck:bool = Path::new("./contacts.csv").exists();
    if !precheck
    {
        std::fs::File::create("./contacts.csv").expect("couldnt create file");
        reset_or_create("contacts");
    }
	print!("\n\tContacts\n\n");
	print!("1.Read Contacts\n");
	print!("2.Append Contacts\n");
	print!("3.Count number of contacts saved\n");
	print!("4.Export Contacts\n");
	print!("5.Edit Contact\n");
	print!("6.Delete a contact\n");
	print!("7.Go back to menu\n");
	print!("8.Exit\n");
	print!(">>");
    io::stdout().flush().unwrap();
    let ch: i32;
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
        1 => {
            print!("\n\tRead Contacts\n\n");
            println!("\n{}\n", "-".repeat(50));
            read_file();
            println!("\n{}\n", "-".repeat(50));
            print!("\nOperation Completed\n");
            back();
            start();
        }
	 2 =>{
	print!("\n\tAppend Contacts\n\n");
            println!("\n{}\n", "-".repeat(50));
            write_file();
            println!("\n{}\n", "-".repeat(50));
            print!("\nOperation Comleted\n");
	back();
	start();
	}
	 3=>{
	print!("\n\tCount Contacts\n\n");
    print!("Number of contacts are : {}\n",count());
	print!("\nOperation Comleted\n");
	back();
	start();
	}
	 4=>{
	print!("\n\tExport Contacts\n\n");
	if vcf_writer(){
	print!("\nOperation Comleted\n");}
    else {
        println!("No Contacts Appended !!! , Nothing to Export\n\tOperation Failed")
    }
	back();
	start();
	}
    5=>{
        print!("\n\tEdit a Contact\n\n");
        if edit(){
        print!("\nOperation Comleted\n");
        }
        else {
        print!("\nOperation Failed , SNO not found\n");
        }
        back();
        start();
        }
	 6=>{
	print!("\n\tDelete a Contact\n\n");
    if del(){
	print!("\nOperation Comleted\n");
    }
    else {
	print!("\nOperation Failed , SNO not found\n");
    }
    back();
	start();
	}
	 7=>{
	menu();
	}
	8=>{
	end();}
	_=>{
	print!("Invalid input , program is exiting !!!");
	std::process::exit(0);}
	}
}

fn vcf_writer() -> bool{
    if count()==0
    {
        return false;
    }
    let precheck:bool = Path::new("./contacts.vcf").exists();
    if precheck
    {
        std::fs::remove_file("./contacts.vcf").expect("couldnt remove temp file");
    }
std::fs::File::create("./contacts.vcf").expect("couldnt create file");
    let mut csv_file = csv::Reader::from_path("./contacts.csv").expect("couldnt open file");
    let mut file = std::fs::OpenOptions::new().write(true).open("./contacts.vcf").expect("couldnt open file");

    for result in csv_file.records() {
        let record = result.expect("couldnt parse");
        let name = &record[1];
        let phone = &record[2];

        write!(file, "BEGIN:VCARD\n").expect("couldnt write");
        write!(file, "VERSION:3.0\n").expect("couldnt write");
        write!(file, "N:{}\n", name).expect("couldnt write");
        write!(file, "TEL:{}\n", phone).expect("couldnt write");
        write!(file, "END:VCARD\n").expect("couldnt write");
        file.flush().expect("couldnt write");
    }
    return true;
}

fn count() -> i32 {
let mut csv_reader = csv::Reader::from_path("./contacts.csv").expect("file couldnt be opened");
let mut count=0;
for _ in csv_reader.records() {
count+=1;
}
return count;
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
print!("\n\tMenu\n\n");
print!("Use the resources below and type 4 to start\n(Enter number to choose the option)\n\n");
print!("1. Disclaimer\n");
print!("2. Info\n");
print!("3. Contact Developer\n");
print!("4. Start\n");
print!("5. Exit\n");
print!(">>");
io::stdout().flush().unwrap();
let ch: i8;
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
}

fn read_file() {
    let mut csv_reader = csv::Reader::from_path("./contacts.csv").expect("file couldnt be opened");
    println!("SNO\t\t\tNAME\t\t\tNUMBER");
    
    for result in csv_reader.records() {
        let record = result.expect("file couldnt be read");
        let sno = &record[0];
        let name = &record[1];
        let number = &record[2];
        println!("{}\t\t\t{}\t\t\t{}", sno, name, number);
    }   
}