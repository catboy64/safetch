use std::env;
mod ascii;

// colors
const BLACK: &str = "\x1b[1;30m";
const RED: &str = "\x1b[1;31m";
const GREEN: &str = "\x1b[1;32m";
const YELLOW: &str = "\x1b[1;33m";
const BLUE: &str = "\x1b[1;34m";
const PURPLE: &str = "\x1b[1;35m";
const CYAN: &str = "\x1b[1;36m";
const WHITE: &str = "\x1b[1;37m";
const COLOR_END: &str = "\x1b[0m";


fn read_file(path:&str) -> String {
    // return the content of a file
    use std::fs;

    let file_path = String::from(path);

    let content =  match fs::read_to_string(file_path) {
        Ok(x) => x,
        Err(_) => String::from(""),
    };
    content
}

fn parse_content(content: String, starting_point: i8, starting_char: char) -> String {
    // return a selected string from the content of a file

    let mut result = String::new();
    let mut x = 0;
    for c in content.chars() {
        if x == starting_point && c != starting_char{
            result.push_str(&(String::from(c)));
        }
        if c == starting_char {
            x+=1
        }
        if x > starting_point+1 {
            return result
        }
    }
    panic!("Error: couldn't parse content. (fn:parse_content)");
}

fn parse_content_with_end_char(content: String, starting_point: i8, starting_char: char, end_char: char) -> String {
    // return a selected string from the content of a file, when starting_char =/= end_char

    let mut result = String::new();
    let mut x = 0;

    for c in content.chars() {
        if x == starting_point && c != end_char && c != starting_char{
            result.push_str(&(String::from(c)));
        }
        if c == starting_char {
            x+=1
        }
        if c == end_char && x >= starting_point {
            return result
        }
    }
    panic!("Error: couldn't parse content. (fn:parse_content_with_end_char)");
}


fn better_parse(content:String, info_title:&str) -> String {
    for line in content.lines() {
        let line_vector: Vec<&str> = line.split('=').collect();
        if line_vector[0] == info_title {
            let info = format!("{}", line_vector[1]);
            return str::replace(info.as_str(), "\"", "");
        }
    }
    return String::from("");
}

fn get_uptime(content: String) -> String{
    // special function to parse for uptime

    let v: Vec<&str> = content.split(".").collect();
    let uptime_str = String::from(v[0]);
    let mut uptime_int = uptime_str.parse::<i32>().unwrap();
    uptime_int /= 60;
    let mut uptime_str = uptime_int.to_string();
    uptime_str.push_str(" mins");
    uptime_str
}

fn i_love_numbers(content: String) -> String {
    // filters out everthing that isn't a number from a string
    // probably not the best way to do that...
    
    let number_array = ['0','1','2','3','4','5','6','7','8','9'];
    let mut result = String::new();
    for c in content.chars() {
        if number_array.contains(&c) {
            result.push_str(&(String::from(c)));
        }
    }
    result
}

fn separator(place:u8, border_custom: String) -> String {
    let length = 10;
    let mut bar = String::from("");
    let border_characters: [&str; 7] =  match border_custom.as_str() {
        "round"  => ["╭", "─", "╮", "├", "┤", "╰", "╯"],
        "square" => ["┌", "─", "┐", "├", "┤", "└", "┘"],
        "none"   => [" ", " ", " ", " ", " ", " ", " "],
        _        => ["╭", "─", "╮", "├", "┤", "╰", "╯"]
    };
    bar.push_str("\x1b[0m");
    return match place {
        0 => {bar.push_str(border_characters[0]);
        for _x in 0..length-1 {
            bar.push_str(border_characters[1]);
        }
        bar.push_str(border_characters[2]);
        bar},
        1 => {bar.push_str(border_characters[3]);
        for _x in 0..length-1 {
            bar.push_str(border_characters[1]);
        }
        bar.push_str(border_characters[4]);
        bar},
        2 => {bar.push_str(border_characters[5]);
        for _x in 0..length-1 {
            bar.push_str(border_characters[1]);
        }
        bar.push_str(border_characters[6]);
        bar},
        _ => String::from("Nothing ever happens."),
    };
}

fn convert_memory(mem_str: String, unity: &str) -> String {
    // convert memory from kib to the selected unity, default is mib
    let mut mem_int: usize = mem_str.parse().expect("Failed to parse string named 'kib_str'.");
    match unity {
        "mib"   => mem_int = mem_int/1000,
        "gib"   => mem_int = mem_int/1000000,
        _       => mem_int = mem_int/1000
    }
    mem_int.to_string()
}

fn get_mem_used(mem_total: String, mem_avail: String) -> String {
    let mem_total_int: u32 = mem_total.parse().unwrap();
    let mem_avail_int: u32 = mem_avail.parse().unwrap();

    let mem_used: u32 = mem_total_int - mem_avail_int;
    mem_used.to_string()
}

fn main() {
    let username = env::var("LOGNAME").unwrap();

    //load config file
    let config_file_path: &str = &("/home/".to_owned() + &username + "/.config/safetch.conf");
    let mut ascii_custom = String::new();
    let mut border_custom = String::new();
    if check_if_file_exist(config_file_path) {
        ascii_custom = conf_parse("ascii", config_file_path);
        border_custom = conf_parse("border", config_file_path);
    } else {
        let _  = create_config_file(config_file_path);
    }

    let os_name = better_parse(read_file("/etc/os-release"), "PRETTY_NAME");

    let os_id = match ascii_custom.as_str() {
        "distro" => better_parse(read_file("/etc/os-release"), "ID"),
        "arch" => String::from("arch"),
        "debian" => String::from("debian"),
        "linuxmint" => String::from("linuxmint"),
        "kali" => String::from("kali"),
        "fedora" => String::from("fedora"),
        "manjaro" => String::from("manjaro"),
        "ubuntu" => String::from("ubuntu"),
        "slackware" => String::from("slackware"),
        "paran" => String::from("paran"),
        "gentoo" => String::from("gentoo"),
        "nixos" => String::from("nixos"),
        "opensuse" => String::from("opensuse"),
        "endeavouros" => String::from("endeavouros"),
        "trisquel" => String::from("trisquel"),
        "void" => String::from("void"),
        "qubes" => String::from("qubes"),
        &_ => better_parse(read_file("/etc/os-release"), "ID")
    };

    let distro_color: String =  match os_id.as_str() {
        "arch" => format!("{CYAN}"),
        "debian" => format!("{RED}"),
        "linuxmint" => format!("{GREEN}"),
        "kali" => format!("{BLUE}"),
        "fedora" => format!("{BLUE}"),
        "manjaro" => format!("{GREEN}"),
        "ubuntu" => format!("{RED}"),
        "slackware" => format!("{BLUE}"),
        "paran" => format!("{PURPLE}"),
        "gentoo" => format!("{PURPLE}"),
        "nixos" => format!("{CYAN}"),
        "opensuse" => format!("{GREEN}"),
        "endeavouros" => format!("{PURPLE}"),
        "trisquel" => format!("{CYAN}"),
        "void" => format!("{GREEN}"),
        "qubes" => format!("{BLUE}"),
        &_ => format!("{COLOR_END}")
    };

    // hostname 
    let mut hostname = read_file("/etc/hostname");
    if hostname.as_str() != "" {
        hostname.pop();
    } else {
        hostname = better_parse(read_file("/etc/conf.d/hostname"), "hostname"); //f u gentoo
        if hostname.as_str() != "" {
        } else {
            hostname = os_id.clone()
        }
    }

    let kernel_version = parse_content(read_file("/proc/version"), 2, ' '); 
    let uptime_str = get_uptime(read_file("/proc/uptime"));
    
    let current_desktop_res = env::var("XDG_CURRENT_DESKTOP");
    let current_desktop = match current_desktop_res {
        Ok(x) => x,
        Err(..) => String::from("")
    };

    let mut cpu_model_name = parse_content_with_end_char(read_file("/proc/cpuinfo"), 5, ':', '\n');
    cpu_model_name.remove(0);



    let mut mem_total = i_love_numbers(parse_content(read_file("/proc/meminfo"), 1, ':'));
    let mem_avail = i_love_numbers(parse_content(read_file("/proc/meminfo"), 3, ':'));
    let mut mem_used = get_mem_used(mem_total.clone(), mem_avail);
    mem_total = convert_memory(mem_total, "mib");
    mem_used = convert_memory(mem_used, "mib");


    print_info(username, hostname, os_name, os_id, distro_color, kernel_version, uptime_str, current_desktop, cpu_model_name, mem_total, mem_used, border_custom);

}

fn create_line(info_name: &str, info: String, distro_color: String, border_custom: String) -> String {
    let mut line = String::from(info_name);

    let vertical_char: &str = match border_custom.as_str() {
        "none" => " ",
        _      => "│"
    };

    line = vertical_char.to_owned() + " " + &distro_color + &line + COLOR_END + vertical_char + "  " + &info;
    line
}


fn print_info(username:String, hostname:String, os_name:String, os_id:String, distro_color:String, kernel_version:String, uptime_str:String, current_desktop:String, cpu_model_name:String, mem_total:String, mem_avail:String, border_custom:String) {

    let binding = os_id.clone();
    let ascii = ascii::get_ascii(&binding);
    let empty = String::from("");
    let mut vec = Vec::new();
    
    vec.push(String::from(""));

    vec.push(separator(0, border_custom.clone()));
    let color_line = format!("{BLACK}⬤ {RED} ⬤ {GREEN} ⬤ {YELLOW} ⬤ {BLUE} ⬤ {PURPLE} ⬤ {CYAN} ⬤ {WHITE} ⬤ {COLOR_END}");

    if username != empty {
        vec.push(create_line("User    ", username, distro_color.clone(), border_custom.clone()));
    }
    if hostname != empty {
        vec.push(create_line("Host    ", hostname, distro_color.clone(), border_custom.clone()));
    }

    vec.push(separator(1, border_custom.clone()));

    

    if os_name != empty {
            vec.push(create_line("OS      ", os_name, distro_color.clone(), border_custom.clone()));
    }
    if kernel_version != empty {
            vec.push(create_line("Kernel  ", kernel_version, distro_color.clone(), border_custom.clone()));
    }
    if uptime_str != empty {
            vec.push(create_line("Uptime  ", uptime_str, distro_color.clone(), border_custom.clone()));
    }
    if current_desktop != empty {
            vec.push(create_line("DE      ", current_desktop, distro_color.clone(), border_custom.clone()));
    }
    if cpu_model_name != empty {
            vec.push(create_line("CPU     ", cpu_model_name, distro_color.clone(), border_custom.clone()));
    }

    //todo
    let mem_line = mem_avail + "/" + &mem_total + " MiB";
    vec.push(create_line("Mem     ", mem_line, distro_color.clone(), border_custom.clone()));


    vec.push(separator(1, border_custom.clone()));
    vec.push(create_line("Colors  ", color_line, distro_color.clone(), border_custom.clone()));
    vec.push(separator(2, border_custom.clone()));

    let vec_length = vec.len();
    //let ascii_line_length = ascii.lines().count();
    
    let mut counter = 0;

    for x in ascii.lines() {
        if counter < vec_length {
                println!("{x}{}",vec[counter]);
        } else {
            println!("{x}");
        }
        counter+=1;
    }
    println!("{COLOR_END}");
}

fn create_config_file(config_file_path: &str) -> std::io::Result<()> {
    use std::fs::File;
    use std::io::Write;
    let config_file_content = "
#ascii (default=distro)
ascii=distro
#border [round, square, none] (default=round)
border=round

";
    let mut config_file = File::create_new(config_file_path)?;
    config_file.write_all(config_file_content.as_bytes())?;

    Ok(())
}

fn conf_parse(info_title:&str, conf_file_path: &str) -> String {
    use std::fs;

    let content =  match fs::read_to_string(conf_file_path) {
        Ok(x) => x,
        Err(_) => String::from(""),
    };
    
    if content != String::from("") {
        for line in content.lines() {
            if line != "" {
                if line.chars().nth(0).unwrap() != '#' {
                    let line_vector: Vec<&str> = line.split('=').collect();
                    if line_vector[0] == info_title {
                        let info = format!("{}", line_vector[1]);
                        return info;
                    }   
                }
            }
        }
    return String::from("default");
    } else {
        return String::from("default");
    }

}

fn check_if_file_exist(file_path: &str) -> bool {
    use std::path::Path;

    let path = Path::new(file_path);
    if path.exists() {
        return true;
    } else {
        return false;
    }
}