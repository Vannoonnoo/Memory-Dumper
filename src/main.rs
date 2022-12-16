use nix::sys::{ptrace, wait};
use nix::unistd::Pid;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom, Write};

fn convert(line: &String) -> (u64, u64, String) {
    let data: Vec<&str> = line.split(' ').collect();

    let range: Vec<&str> = data[0].split('-').collect();

    let addr = u64::from_str_radix(range[0], 16).unwrap();

    let size = u64::from_str_radix(range[1], 16).unwrap() - addr;

    let name = String::from(data[data.len() - 1]);

    return (addr, size, name);
}

fn main() {
    let pid = env::args().nth(1).expect("Needs a PID argument");
    let pid = Pid::from_raw(pid.parse().unwrap());

    let map = File::open(format!("/proc/{}/maps", pid)).unwrap();
    let map = BufReader::new(map);

    let mut mem = File::open(format!("/proc/{}/mem", pid)).unwrap();

    let mut output = File::create("/tmp/output").unwrap();

    let mut data = Vec::new();

    for line in map.lines() {
        let current = line.unwrap();

        let result = convert(&current);

        data.push(result);
    }

    ptrace::attach(pid).expect("Failed to attach");

    wait::waitpid(pid, None);

    for (addr, size, name) in data {
        if name != "[heap]" && name != "[stack]" {
            continue;
        }

        let mut buf = Vec::with_capacity(size as usize);

        mem.seek(SeekFrom::Start(addr));

        Read::by_ref(&mut mem).take(size).read_to_end(&mut buf);

        mem.rewind();

        output.write(&buf);
    }

    ptrace::detach(pid, None);
}
