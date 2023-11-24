/* Suggestions: 1. Write a function for every command
                2. Start with the pwd command
                3. Continue with the other commands that do not have parameters
*/
use std::env;
use std::fs;
use std::fs::Permissions;
use std::io;
use std::io::Read;
use std::io::Write;
use std::os::unix::prelude::PermissionsExt;
use std::process::exit;
use std::fs::File;
use std::path::Path;

fn pwd() {
    if let Ok(current_dir) = env::current_dir() {
        match current_dir.to_str() {
            Some(str) => println!("{}", str),
            None => (),
        }
    } else {
        println!("Error");
    }
}

fn echo(arg : &Vec<String>) {

    let mut a = 2;

    if arg[2] == "-n" {
        a += 1;
    }

    for i in a..arg.len() {
        print!("{}", arg[i]);
        if i != arg.len() - 1 {
            print!(" ");
        }
    }

    if arg[2] != "-n" {
        println!();
    }
}

fn mkdir(path: &str) -> Result<(), std::io::Error> {

    match fs::create_dir(path) {
        Ok(()) => Ok(()),
        Err(e) => Err(e),
    }
}

fn rmdir(path: &str) -> Result<(), std::io::Error> {

    match fs::remove_dir(path) {
        Ok(()) => Ok(()),
        Err(e) => Err(e),
    }
}

fn mv(path1: &str, path2: &str) -> Result<(), std::io::Error> {

    match fs::rename(path1, path2) {
        Ok(()) => Ok(()),
        Err(e) => Err(e),
    }
}

fn dir_is_empty(path: &str) -> bool {

    let items = fs::read_dir(path);

    match items {
        Ok(items) => {
            for item in items{
                if let Ok(file) = item {
                    if let Ok(file_type) = file.file_type() {
                        if file_type.is_file() || file_type.is_dir() {
                            return false;
                        }
                    }
                }
            }
            return true;
        },
        Err(_) => false,
    }
}

fn rmfile(path: &str) -> Result<(), std::io::Error> {

    match fs::remove_file(path) {
        Ok(()) => Ok(()),
        Err(e) => Err(e),
    }
}

fn rm_recursive(path: &str) -> Result<(), std::io::Error> {

    match fs::remove_dir_all(path) {
        Ok(()) => Ok(()),
        Err(e) => Err(e),
    }

}

fn ln(path: &str, link: &str) -> Result<(), std::io::Error> {

    match fs::hard_link(path, link) {
        Ok(()) => Ok(()),
        Err(e) => Err(e),
    }

}

fn ln_symbolic(path: &str, link: &str) -> Result<(), std::io::Error> {

    match std::os::unix::fs::symlink(path, link) {
        Ok(()) => Ok(()),
        Err(e) => Err(e),
    }

}

fn cat(path: &str) -> Result<(), std::io::Error> {

    let path = File::open(&path);

    match path {
        Ok(mut file) => {
            let mut text = String::new();

            match file.read_to_string(&mut text) {
                Ok(_) => {
                    print!("{}", text);
                    Ok(())
                },
                Err(e) => Err(e),
            }
        },
        Err(_) => exit(-20),
    }
}

fn ls(directory: &str) -> Result<(), std::io::Error> {

    let contents = fs::read_dir(directory);

    match contents {
        Ok(dir) => {
            for content in dir {
                if let Ok(entry) = content {
                    let path = entry.path();
                    let file_name = path.file_name().unwrap_or_default();
                    match file_name.to_str() {
                        Some(str) => {
                            if !str.starts_with(".") {
                                println!("{}", str)
                            }
                        },
                        None => (),
                    }
                }
            }
            Ok(())
        },
        Err(e) => Err(e),
    }
}

fn ls_all(directory: &str) -> Result<(), std::io::Error> {

    let contents = fs::read_dir(directory);
    
    println!(".");
    println!("..");

    match contents {
        Ok(dir) => {
            for content in dir {
                if let Ok(entry) = content {
                    let path = entry.path();
                    let file_name = path.file_name().unwrap_or_default();

                    match file_name.to_str() {
                        Some(str) => println!("{}", str),
                        None => (),
                    }
                }
            }
            Ok(())
        },
        Err(e) => Err(e),
    }
}

fn ls_r(directory: &str) -> Result<(),std::io::Error> {

    println!("{}:", directory);

    let contents = fs::read_dir(directory);

    match contents {
        Ok(dir) => {
            for content in dir {
                if let Ok(entry) = content {
                    let path = entry.path();
                    let file_name = path.file_name().unwrap_or_default();
                    match file_name.to_str() {
                        Some(str) => {
                            if !str.starts_with(".") {
                                println!("{}", str)
                            }
                        },
                        None => (),
                    }
                }
            }
        },
        Err(_e) => (),
    }

    let entries = fs::read_dir(directory)?;

    for entry in entries {
        match entry {
            Ok(entry) => {
                let path = entry.path();

                if path.is_dir() {
                    let _var = match path.to_str() {
                        Some(p) => ls_r(p),
                        None => Ok(()),
                    };
                }
            },
            Err(_) => (),
        }
    }
    Ok(())
}

fn ls_r_all(directory: &str) -> Result<(),std::io::Error> {

    println!("{}:", directory);

    let contents = fs::read_dir(directory);
    println!(".");
    println!("..");

    match contents {
        Ok(dir) => {
            for content in dir {
                if let Ok(entry) = content {
                    let path = entry.path();
                    let file_name = path.file_name().unwrap_or_default();
                    match file_name.to_str() {
                        Some(str) => {
                                println!("{}", str);
                        },
                        None => (),
                    }
                }
            }
        },
        Err(_e) => (),
    }

    let entries = fs::read_dir(directory)?;

    for entry in entries {
        match entry {
            Ok(entry) => {
                let path = entry.path();

                if path.is_dir() {
                    let _var = match path.to_str() {
                        Some(p) => ls_r_all(p),
                        None => Ok(()),
                    };
                }
            },
            Err(_) => (),
        }
    }
    Ok(())

}

fn interpret_chmod(perm: &str) -> u32 {

    let mut number = 0;
    let mut permision = 0;

    if perm.contains("r") {
        number += 4;
    }

    if perm.contains("w") {
        number += 2;
    }

    if perm.contains("x") {
        number += 1;
    }

    if perm.contains("u") {
        permision += 100 * number;
    }

    if perm.contains("g") {
        permision += 10 * number;
    }

    if perm.contains("o") {
        permision += number;
    }

    if perm.contains("a") {
        permision += 111 * number;
    }

    return permision;

}

fn modify_permissons(n: u32, mut perm: u32, sign: &str) -> u32 {

    if sign.contains("+") {
        if sign.contains("u") || sign.contains("a") {
            if n / 100 >= 4 && perm / 100 >= 4 {
                perm -= 400;
            }

            if (n / 100 == 2 || n / 100 == 3 || n / 100 == 6 || n / 100 == 7) &&
                (perm / 100 == 2 || perm / 100 == 3 || perm / 100 == 6 || perm / 100 == 7) {
                    perm -= 200;
                }

            if n / 100 % 2 == 1 && perm / 100 % 2 == 1 {
                perm -= 100;
            }
        }

        if sign.contains("g") || sign.contains("a") {
            if n / 10 % 10 >= 4 && perm / 10 % 10 >= 4 {
                perm -= 40;
            }

            if (n / 10 % 10 == 2 || n / 10 % 10 == 3 || n / 10 % 10 == 6 || n / 10 % 10 == 7) &&
                (perm / 10 % 10 == 2 || perm / 10 % 10 == 3 || perm / 10 % 10 == 6 || perm / 10 % 10 == 7) {
                    perm -= 20;
                }

            if n / 10 % 10 % 2 == 1 && perm / 10 % 10 % 2 == 1 {
                perm -= 10;
            }
        }

        if sign.contains("o") || sign.contains("a") {
            if n % 10 >= 4 && perm % 10 >= 4 {
                perm -= 4;
            }

            if (n % 10 == 2 || n % 10 == 3 || n % 10 == 6 || n % 10 == 7) &&
                (perm % 10 == 2 || perm % 10 == 3 || perm % 10== 6 || perm % 10 == 7) {
                    perm -= 2;
                }

            if n % 2 == 1 && perm % 2 == 1 {
                perm -= 1;
            }
        }
    }

    if sign.contains("-") {
        if sign.contains("u") || sign.contains("a") {
            if !(n / 100 >= 4) && perm / 100 >= 4 {
                perm -= 400;
            }

            if !(n / 100 == 2 || n / 100 == 3 || n / 100 == 6 || n / 100 == 7) &&
                (perm / 100 == 2 || perm / 100 == 3 || perm / 100 == 6 || perm / 100 == 7) {
                    perm -= 200;
                }

            if !(n / 100 % 2 == 1) && perm / 100 % 2 == 1 {
                perm -= 100;
            }
        }

        if sign.contains("g") || sign.contains("a") {
            if !(n / 10 % 10 >= 4) && perm / 10 % 10 >= 4 {
                perm -= 40;
            }

            if !(n / 10 % 10 == 2 || n / 10 % 10 == 3 || n / 10 % 10 == 6 || n / 10 % 10 == 7) &&
                (perm / 10 % 10 == 2 || perm / 10 % 10 == 3 || perm / 10 % 10 == 6 || perm / 10 % 10 == 7) {
                    perm -= 20;
                }

            if !(n / 10 % 10 % 2 == 1) && perm / 10 % 10 % 2 == 1 {
                perm -= 10;
            }
        }

        if sign.contains("o") || sign.contains("a") {
            if !(n % 10 >= 4) && perm % 10 >= 4 {
                perm -= 4;
            }

            if !(n % 10 == 2 || n % 10 == 3 || n % 10 == 6 || n % 10 == 7) &&
                (perm % 10 == 2 || perm % 10 == 3 || perm % 10== 6 || perm % 10 == 7) {
                    perm -= 2;
                }

            if !(n % 2 == 1) && perm % 2 == 1 {
                perm -= 1;
            }
        }
    }

    return perm;
}

fn copy_normal(s: &str, d: &str) -> Result<(), std::io::Error> {
    let path = fs::metadata(&s);

    match path {
        Ok(file) => {
            if file.is_dir() {
                exit(-90);
            }

            if file.is_file() {
                let mut source = match fs::File::open(&s) {
                    Ok(source) => source,
                    Err(_) => exit(-90),
                };
                let mut dest = match fs::File::open(&d) {
                    Ok(dest) => dest,
                    Err(_) => {
                        match fs::File::create(&d) {
                            Ok(dest) => dest,
                            Err(_) => exit(-90),
                        }
                    },
                };

                match io::copy(&mut source, &mut dest) {
                    Ok(_) => (),
                    Err(_) => {
                        let last_slash = match s.rfind("/") {
                            Some(last_slash) => last_slash,
                            None => 10,
                        };

                        let substring = &s[(last_slash + 1)..];

                        let new_file = format!("{}/{}", d, substring);

                        let mut new_dest = match fs::File::create(&new_file) {
                            Ok(new_dest) => new_dest,
                            Err(_) => {println!("here");exit(-90);},
                        };
                        
                        match io::copy(&mut source, &mut new_dest) {
                            Ok(_) => (),
                            Err(_) => exit(-90),
                        }
                    },
                }
            }
            Ok(())
        },
        Err(_) => exit(-90),
    }
}

fn copy_directory_recursive(source_dir: &str, destination_dir: &str) -> Result<(), io::Error> {

    let source_path = Path::new(source_dir);
    let destination_path = Path::new(destination_dir);

    if source_path.is_dir() {
        fs::create_dir_all(destination_path)?;

        let entries = fs::read_dir(source_path)?;

        for entry in entries {
            let entry = entry?;
            let source_entry_path = entry.path();
            let destination_entry_path = destination_path.join(entry.file_name());

            if source_entry_path.is_dir() {
                copy_directory_recursive(&source_entry_path.to_str().unwrap(), &destination_entry_path.to_str().unwrap())?;
            } else {
                fs::copy(&source_entry_path, &destination_entry_path)?;
            }
        }
    } else {
        match copy_normal(&source_dir, &destination_dir) {
            Ok(_) => (),
            Err(_) => exit(-90),
        }
    }

    Ok(())
}


fn main() {

    let arguments: Vec<String> = env::args().collect();

    match arguments[1].as_str() {
        "pwd" => {
            if arguments.len() != 2 {
                println!("Invalid command");
                exit(-1);
            }
            pwd();
        },
        "echo" => {
            echo(&arguments);
        },
        "mkdir" => {
            if arguments.len() == 2 {
                println!("Invalid command");
                exit(-1);
            }

            for i in 2..arguments.len() {
                match mkdir(&arguments[i]) {
                    Ok(()) => (),
                    Err(_error) => exit(-30),
                }
            }
        },
        "rmdir" => {
            if arguments.len() == 2 {
                println!("Invalid command");
                exit(-1);
            }

            for i in 2..arguments.len() {
                match rmdir(&arguments[i]) {
                    Ok(()) => (),
                    Err(_error) => exit(-60),
                }
            }
        },
        "mv" => {
            if arguments.len() != 4 {
                println!("Invalid command");
                exit(-1);
            }

            match mv(&arguments[2], &arguments[3]) {
                Ok(()) => (),
                Err(_error) => exit(-40)
            }
        },
        "rm" => {
            if arguments.len() < 3 {
                println!("Invalid command");
                exit(-1);
            }

            let mut flags = 0;
            let mut k = 2;
            while k < arguments.len() && (arguments[k] == "-r" || arguments[k] == "-R"
                    || arguments[k] == "--recursive" || arguments[k] == "-d" || arguments[k] == "--dir") {
                
                if arguments[k] == "-d" || arguments[k] == "--dir" {
                    flags += 1;
                }

                if arguments[k] == "-r" || arguments[k] == "-R" || arguments[k] == "--recursive" {
                    flags += 2;
                }

                k += 1;
            }

            if k == arguments.len() {
                println!("Invalid command");
                exit(-1);
            }

            // daca n am argumente trebuie sa nu pot sterge directoare
            if flags == 0 {
                let mut ok = 1;
                for i in 2..arguments.len() {
                    let data = fs::metadata(&arguments[i]);
                    match data{
                        Ok(data_type) => {
                            if data_type.is_dir() {
                                ok = 0;
                            } else if data_type.is_file() {
                                match rmfile(&arguments[i]) {
                                    Ok(()) => (),
                                    Err(_) => exit(-70),
                                }
                            }
                        },
                        Err(_) => exit(-70),
                    }
                }
                if ok == 0 {
                    exit(-70);
                }
            }

            // nu sterg nimi recursiv, deci trebuie verificat ca directorul e gol
            if flags == 1 {

                let mut ok = 1;
                for i in 3..arguments.len() {

                    let data = fs::metadata(&arguments[i]);

                    if let Ok(data_type) = data {
                        if data_type.is_dir() {
                            if dir_is_empty(&arguments[i]) {
                                match rmdir(&arguments[i]) {
                                    Ok(()) => (),
                                    Err(_error) => exit(-70),
                                }
                            } else {
                                ok = 0;
                            }
                        } else if data_type.is_file() {
                            match rmfile(&arguments[i]) {
                                Ok(()) => (),
                                Err(_) => exit(-70),
                            }
                        }
                    }
                }
                if ok == 0 {
                    exit(-70);
                }
            }
            
            // in cazul acesta 100% am un flag -r deci pot sterge orice mi se da
            if flags == 2 || flags == 3 {

                for i in k..arguments.len() {
                    let data = fs::metadata(&arguments[i]);
                    if let Ok(data_type) = data {
                        if data_type.is_dir() {

                            match rm_recursive(&arguments[i]) {
                                Ok(()) => (),
                                Err(_) => exit(-70),
                            }
                        } else if data_type.is_file(){
                            match rmfile(&arguments[i]) {
                                Ok(()) => (),
                                Err(_) => exit(-70),
                            }
                        }
                    }
                }
            }
        },
        "ln" => {

            if arguments.len() != 4 && arguments.len() != 5 {
                println!("Invalid command");
                exit(-1);
            }

            if arguments[2].starts_with("-") && !(arguments[2] == "-s"  || arguments[2] == "--symbolic" ) {
                println!("Invalid command");
                exit(-1);
            }

            if arguments[2] == "-s"  || arguments[2] == "--symbolic" {
                match ln_symbolic(&arguments[3], &arguments[4]) {
                    Ok(()) => (),
                    Err(_) => exit(-50),
                }
            } else {
                match ln(&arguments[2], &arguments[3]) {
                    Ok(()) => (),
                    Err(_) => exit(-50),
                }
            }
        },
        "cat" => {

            if arguments.len() < 3{
                println!("Invalid command");
                exit(-1);
            }

            for i in 2..arguments.len() {
                match cat(&arguments[i]) {
                    Ok(()) => (),
                    Err(_) => exit(-20),
                }
            }
        },
        "ls" => {

            let mut flags = 0;
            let mut k = 2;
            while k < arguments.len() && (arguments[k] == "-a" || arguments[k] == "-all"
                    || arguments[k] == "--recursive" || arguments[k] == "-r" || arguments[k] == "-R") {
                
                if arguments[k] == "-a" || arguments[k] == "-all" {
                    flags += 1;
                }

                if arguments[k] == "-r" || arguments[k] == "-R" || arguments[k] == "--recursive" {
                    flags += 2;
                }

                k += 1;
            }

            if flags == 0 {
                if arguments.len() == 3 {
                    let data = fs::metadata(&arguments[2]);
                    match data {
                        Ok(data_type) =>{
                            if data_type.is_dir() {

                                match ls(&arguments[2]) {
                                    Ok(()) => (),
                                    Err(_) => exit(-80),
                                }
                            } else if data_type.is_file() {
                                println!("{}", arguments[2]);
                            } 
                        },
                        Err(_) => exit(-80),
                    }
                } else {
                    if let Ok(current_dir) = env::current_dir() {

                        match current_dir.to_str() {
                            Some(str) => {
                                match ls(str) {
                                    Ok(()) => (),
                                    Err(_) => exit(-80),
                                }
                            }
                            None => (),
                        }
                    }
                }   
            }

            if flags == 1 {

                if arguments.len() == 4 {
                    let data = fs::metadata(&arguments[3]);
                    match data {
                        Ok(data_type) => {
                            if data_type.is_dir() {

                                match ls_all(&arguments[3]) {
                                    Ok(()) => (),
                                    Err(_) => exit(-80),
                                }
                            } else if data_type.is_file() {
                                println!("{}", arguments[3]);
                            }   
                        },
                        Err(_) => exit(-80),
                    }
                } else {
                    if let Ok(current_dir) = env::current_dir() {

                        match current_dir.to_str() {
                            Some(str) => {
                                match ls_all(str) {
                                    Ok(()) => (),
                                    Err(_) => exit(-80),
                                }
                            }
                            None => (),
                        }
                    }
                }
            }

            if flags == 2 {
                
                if arguments.len() == 4 {
                    let data = fs::metadata(&arguments[3]);
                    match data {
                        Ok(data_type) => {
                            if data_type.is_dir() {

                                match ls_r(&arguments[3]) {
                                    Ok(()) => (),
                                    Err(_) => exit(-80),
                                }
                            } else if data_type.is_file() {
                                println!("{}", arguments[3]);
                            }   
                        },
                        Err(_) => exit(-80),
                    }
                } else {
                    if let Ok(current_dir) = env::current_dir() {

                        match current_dir.to_str() {
                            Some(str) => {
                                match ls_r(str) {
                                    Ok(()) => (),
                                    Err(_) => exit(-80),
                                }
                            },
                            None => (),
                        }
                    }
                }
            }


            if flags == 3 {

                if arguments.len() == 5 {
                    let data = fs::metadata(&arguments[4]);
                    match data {
                        Ok(data_type) => {
                            if data_type.is_dir() {

                                match ls_r_all(&arguments[4]) {
                                    Ok(()) => (),
                                    Err(_) => exit(-80),
                                }
                            } else if data_type.is_file() {
                                println!("{}", arguments[4]);
                            }   
                        },
                        Err(_) => exit(-80),
                    }
                } else {
                    if let Ok(current_dir) = env::current_dir() {

                        match current_dir.to_str() {
                            Some(str) => {
                                match ls_r_all(str) {
                                    Ok(()) => (),
                                    Err(_) => exit(-80),
                                }
                            },
                            None => (),
                        }
                    }
                }  
            }
        },
        "touch" => {
            if arguments.len() < 3 {
                println!("Invalid command");
                exit(-1);
            }

            let mut flags = 0;
            let mut k = 2;
            while k < arguments.len() && (arguments[k] == "-c" || arguments[k] == "--no-create"
                    || arguments[k] == "-a" || arguments[k] == "-m" ) {
                
                if arguments[k] == "-a" {
                    flags += 1;
                }

                if arguments[k] == "-m" {
                    flags += 2;
                }

                if arguments[k] == "-c" || arguments[k] == "--no-create" {
                    flags += 3;
                }

                k += 1;
            }

            if flags == 0 {
                let data = fs::metadata(&arguments[2]);

                match data {
                    Ok(_data) => {
                        let data_file = fs::File::create(&arguments[2]);

                        let mut fisier = match data_file {
                            Ok(file) => file,
                            Err(_) => exit(-100),
                        };

                        let _contents = fs::read_to_string(&arguments[2]);
                        match fisier.write_all(b"sourceforge.net\n") {
                            Ok(_) => (),
                            Err(_) => println!("hello"), 
                        }
                    },
                    Err(_) => {
                        match File::create(&arguments[2]) {
                            Ok(_) => (),
                            Err(_) => exit(-100),
                        }
                    },
                }
            }

            if flags == 1 {
                let data = fs::metadata(&arguments[3]);

                match data {
                    Ok(_data) => {
                        let data_file = fs::File::create(&arguments[3]);

                        let mut _fisier = match data_file {
                            Ok(file) => file,
                            Err(_) => exit(-100),
                        };

                        let _contents = fs::read_to_string(&arguments[3]);
                    },
                    Err(_) => {
                        match File::create(&arguments[2]) {
                            Ok(_) => (),
                            Err(_) => exit(-100),
                        }
                    },
                }
            }
            
            if flags == 2 {
                let data = fs::metadata(&arguments[3]);

                match data {
                    Ok(_data) => {
                        let data_file = fs::File::create(&arguments[3]);

                        let mut fisier = match data_file {
                            Ok(file) => file,
                            Err(_) => exit(-100),
                        };
                        
                        match fisier.write_all(b"sourceforge.net\n") {
                            Ok(_) => (),
                            Err(_) => println!("hello"), 
                        }
                    },
                    Err(_) => {
                        match File::create(&arguments[2]) {
                            Ok(_) => (),
                            Err(_) => exit(-100),
                        }
                    },
                }
            }
        },
        "chmod" => {
            if arguments.len() < 4 {
                println!("Invalid command");
                exit(-1);
            }

            let parsed_number: Result<u32, _ > = arguments[2].parse();

            match parsed_number {
                Ok(mut number) => {
                    match fs::metadata(&arguments[3]) {
                        Ok(_file) => {
                            number = number % 10 + number / 10 % 10 * 8 + number / 100 * 64;
                            let perm = Permissions::from_mode(number);
                            match fs::set_permissions(&arguments[3], perm) {
                                Ok(_) => (),
                                Err(_) => exit(-25),
                            }
                        },
                        Err(_) => exit(-25),
                    }
                },
                Err(_) => {

                    if !(arguments[2].contains("r") || arguments[2].contains("w") || arguments[2].contains("x")) {
                        println!("Invalid command");
                        exit(-1);
                    }
                    

                   match fs::metadata(&arguments[3]) {
                        Ok(file) => {
                            let permissions = file.permissions();
                            let perm_as_number = permissions.mode();

                            let mut perm = interpret_chmod(&arguments[2]);

                            let str_perm = format!("{:o}", perm_as_number);
                            let parsed_octal: Result<u32, _> = str_perm.parse();
                            let mut n = match parsed_octal {
                                Ok(n) => n,
                                Err(_) => exit(-25),
                            };

                            n = n % 1000;
                            
                            perm = modify_permissons(n, perm, &arguments[2]);

                            if arguments[2].contains("+") {
                                perm = n + perm;
                            } else {
                                perm = n - perm;
                            }

                            perm = perm % 10 + perm /10 % 10 * 8 + perm / 100 * 64;
                            
                            let permi = Permissions::from_mode(perm);

                            match fs::set_permissions(&arguments[3], permi) {
                                Ok(_) => (),
                                Err(_) => exit(-25),
                            }
                        },
                        Err(_) => exit(-25),
                    }
                },
            }
        },
        "cp" => {
            if arguments.len() < 4 {
                println!("Invalid command");
                exit(-1);
            }

            let mut flags = 0;

            if arguments[2].contains("-r") || arguments[2].contains("-R") || arguments[2].contains("--recusive") {
                flags += 1;
            }

            if flags == 0 {
                
                match copy_normal(&arguments[2], &arguments[3]) {
                    Ok(_) => (),
                    Err(_) => exit(-90),
                } 
            }

            if flags == 1 {
                match copy_directory_recursive(&arguments[3],&arguments[4]) {
                    Ok(_) => (),
                    Err(_) => exit(-90),
                }
            }
        },
        "grep" => {
            if arguments.len() < 4 {
                println!("Invalid command");
                exit(-1);
            }
        },
        _ => {
            println!("Invalid command");
            exit(-1);
        },
    };
}