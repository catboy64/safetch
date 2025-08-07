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

fn separator(place:u8) -> String {
    let length = 10;
    let mut bar = String::from("");
    bar.push_str("\x1b[0m");
    return match place {
        0 => {bar.push_str("╭");
        for _x in 0..length-1 {
            bar.push_str("─");
        }
        bar.push_str("╮");
        bar},
        1 => {bar.push_str("├");
        for _x in 0..length-1 {
            bar.push_str("─");
        }
        bar.push_str("┤");
        bar},
        2 => {bar.push_str("╰");
        for _x in 0..length-1 {
            bar.push_str("─");
        }
        bar.push_str("╯");
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

    let os_name = better_parse(read_file("/etc/os-release"), "PRETTY_NAME");
    let os_id = better_parse(read_file("/etc/os-release"), "ID");
    //let os_id = String::from("qubes"); // for testing purposes.

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
        &_ => format!("{COLOR_END}"),
    };

    // hostname 
    let mut hostname = read_file("/etc/hostname");
    if hostname.as_str() != "" {
        hostname.pop();
    } else {
        hostname = better_parse(read_file("/etc/conf.d/hostname"), "hostname"); //f u gentoo
        if hostname.as_str() != "" {
            hostname.pop();
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


    print_info(username, hostname, os_name, os_id, distro_color, kernel_version, uptime_str, current_desktop, cpu_model_name, mem_total, mem_used);

}

fn create_line(info_name: &str, info: String, distro_color: String) -> String {
    let mut line = String::from(info_name);
    line = "│ ".to_owned() + &distro_color + &line + COLOR_END + "│  " + &info;
    line
}


fn print_info(username:String, hostname:String, os_name:String, os_id:String, distro_color:String, kernel_version:String, uptime_str:String, current_desktop:String, cpu_model_name:String, mem_total:String, mem_avail:String) {

    let binding = os_id.clone();
    let ascii = ascii::get_ascii(&binding);
    let empty = String::from("");
    let mut vec = Vec::new();
    
    vec.push(String::from(""));

    vec.push(separator(0));
    let color_line = format!("{BLACK}⬤ {RED} ⬤ {GREEN} ⬤ {YELLOW} ⬤ {BLUE} ⬤ {PURPLE} ⬤ {CYAN} ⬤ {WHITE} ⬤ {COLOR_END}");

    if username != empty {
        vec.push(create_line("User    ", username, distro_color.clone()));
    }
    if hostname != empty {
        vec.push(create_line("Host    ", hostname, distro_color.clone()));
    }

    vec.push(separator(1));

    

    if os_name != empty {
            vec.push(create_line("OS      ", os_name, distro_color.clone()));
    }
    if kernel_version != empty {
            vec.push(create_line("Kernel  ", kernel_version, distro_color.clone()));
    }
    if uptime_str != empty {
            vec.push(create_line("Uptime  ", uptime_str, distro_color.clone()));
    }
    if current_desktop != empty {
            vec.push(create_line("DE      ", current_desktop, distro_color.clone()));
    }
    if cpu_model_name != empty {
            vec.push(create_line("CPU     ", cpu_model_name, distro_color.clone()));
    }

    let mut mem_line = format!("{distro_color}Mem{COLOR_END}     │  ");
    mem_line = "│ ".to_owned() + &mem_line + COLOR_END + &mem_avail + "/" + &mem_total + " MiB";
    vec.push(mem_line.clone());

    vec.push(separator(1));

    vec.push(create_line("Colors  ", color_line, distro_color.clone()));
    
    vec.push(separator(2));

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
