use algo_lib::collections::iter_ext::IterExt;
use algo_lib::io::input::Input;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs::{create_dir, File};
use std::io::{BufRead, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::{io, thread};

//noinspection RsFieldNaming
#[derive(Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
struct IOType {
    r#type: String,
    fileName: Option<String>,
    pattern: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
struct Test {
    input: String,
    output: String,
}

//noinspection RsFieldNaming
#[derive(Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
struct TaskClass {
    taskClass: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct Languages {
    java: TaskClass,
}

//noinspection RsFieldNaming
#[derive(Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
struct Task {
    name: String,
    group: String,
    url: String,
    interactive: bool,
    timeLimit: u64,
    tests: Vec<Test>,
    testType: String,
    input: IOType,
    output: IOType,
    languages: Languages,
}

impl Task {
    fn task_name(&self) -> String {
        let mut res = String::new();
        for c in self.languages.java.taskClass.chars() {
            if c.is_uppercase() {
                if !res.is_empty() {
                    res.push('_');
                }
                res.push(c.to_ascii_lowercase());
            } else {
                res.push(c);
            }
        }
        res
    }
}

fn handle(mut stream: TcpStream) {
    let mut buf = Vec::new();
    stream.read_to_end(&mut buf).unwrap();
    let request = String::from_utf8_lossy(&buf[..]).to_string();
    if request.is_empty() {
        return;
    }
    let pos = request.find("{");
    match pos {
        None => {
            println!("Bad request: {}", request);
        }
        Some(pos) => {
            process(&request[pos..]);
        }
    }
}

fn read_lines<P>(filename: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).unwrap();
    let mut res = Vec::new();
    for line in io::BufReader::new(file).lines() {
        res.push(line.unwrap());
    }
    res
}

fn try_read_lines<P>(filename: P) -> Option<Vec<String>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename);
    let file = match file {
        Ok(file) => file,
        Err(_) => {
            return None;
        }
    };
    let mut res = Vec::new();
    for line in io::BufReader::new(file).lines() {
        res.push(line.unwrap());
    }
    Some(res)
}

fn write_lines(filename: &str, lines: Vec<String>) {
    let mut file = File::create(filename).unwrap();
    for line in lines {
        file.write(line.as_bytes()).unwrap();
        file.write("\n".as_bytes()).unwrap();
    }
    file.flush().unwrap();
}

fn process(request: &str) {
    let mut stdin = std::io::stdin();
    let mut input = Input::new(&mut stdin);
    let task: Task = serde_json::from_slice(request.as_bytes()).unwrap();
    let name = task.task_name();
    let mut lines = Vec::new();
    for l in read_lines("../Cargo.toml") {
        if l.contains(format!("\"{}\"", name).as_str()) {
            println!("Already exists");
            return;
        }
        lines.push(l.clone());
        if l.as_str() == "members = [" {
            lines.push(format!("    \"{}\",", name));
        }
    }
    let site = task.url.clone();
    let re = Regex::new(r"https?://(?:www.)?([^.]*)[.].*").unwrap();
    let mut matches = re.captures_iter(site.as_str());
    let site = matches.next().unwrap();
    let site = match site.get(1) {
        None => "default",
        Some(s) => s.as_str(),
    };
    eprintln!("{}", site);
    let solve = try_read_lines(format!("{}.txt", site));
    let solve = match solve {
        None => read_lines("default.txt"),
        Some(solve) => solve,
    };
    let mut t = solve[0].parse::<i8>().unwrap();
    let mut solve = solve[1..].iter().cloned().collect_vec();
    if t == -1 {
        println!("input type? 0 - single test, 1 - num test known, 2 - until eof");
        t = input.read();
    }
    create_dir(format!("../{}", name).as_str()).unwrap();
    create_dir(format!("../{}/src", name).as_str()).unwrap();
    create_dir(format!("../{}/tests", name).as_str()).unwrap();
    for (i, test) in task.tests.iter().enumerate() {
        write_lines(
            format!("../{}/tests/{}.in", name, i + 1).as_str(),
            vec![test.input.clone()],
        );
        write_lines(
            format!("../{}/tests/{}.out", name, i + 1).as_str(),
            vec![test.output.clone()],
        );
    }
    write_lines(
        format!("../{}/build.rs", name).as_str(),
        read_lines("build.txt"),
    );
    let mut main = Vec::new();
    main.push(format!("//{}", task.group));
    main.push(format!("//{}", serde_json::to_string(&task.input).unwrap()));
    main.push(format!(
        "//{}",
        serde_json::to_string(&task.output).unwrap()
    ));
    main.append(&mut read_lines("prefix.txt"));
    main.append(&mut solve);
    main.append(&mut read_lines("after_solve.txt"));
    match t {
        0 => {
            main.push("    solve(&mut input, 1);".to_string());
        }
        1 => {
            main.push("    let t = input.read();".to_string());
            main.push("    for i in 0usize..t {".to_string());
            main.push("        solve(&mut input, i + 1);".to_string());
            main.push("    }".to_string());
        }
        2 => {
            main.push("    let mut i = 0usize;".to_string());
            main.push("    while input.peek().is_some() {".to_string());
            main.push("        solve(&mut input, i + 1);".to_string());
            main.push("        i += 1;".to_string());
            main.push("    }".to_string());
        }
        _ => {
            unreachable!();
        }
    }
    main.append(&mut read_lines("middle.txt"));
    main.push(format!(
        "    let time_limit = std::time::Duration::from_millis({});",
        task.timeLimit
    ));
    main.push(format!(
        "    let mut paths = std::fs::read_dir(\"./{}/tests/\")",
        name
    ));
    main.append(&mut read_lines("suffix.txt"));
    write_lines(format!("../{}/src/main.rs", name).as_str(), main);
    let mut toml = read_lines("toml_prefix.txt");
    toml.push(format!("name = \"{}\"", name));
    toml.append(&mut read_lines("toml_suffix.txt"));
    write_lines(format!("../{}/Cargo.toml", name).as_str(), toml);
    write_lines("../Cargo.toml", lines);
    println!("{} parsed", name);
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4244").unwrap();
    println!("Listening for connections on port {}", 4244);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| handle(stream));
            }
            Err(e) => {
                println!("Unable to connect: {}", e);
            }
        }
    }
}
