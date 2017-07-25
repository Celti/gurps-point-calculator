// GURPS is a trademark of Steve Jackson Games, and its rules and art are
// copyrighted by Steve Jackson Games. All rights are reserved by Steve Jackson
// Games. This game aid is the original creation of Patrick Burroughs and is
// released for free distribution, and not for resale, under the permissions
// granted in the Steve Jackson Games Online Policy [1].
// 
// Copyright (c) 2017 Patrick L. H. Burroughs.
// 
// Permission is hereby granted, free of charge, to any person obtaining a copy of
// this software and associated documentation files (the "Software"), to deal in
// the Software for non-commercial use, including without limitation the rights to
// use, copy, modify, merge, publish, distribute, or sublicense under a compatible
// license, and to permit persons to whom the Software is furnished to do so,
// subject to the following condition:
// 
// The above copyright and trademark notices and this permission notice shall be
// included in all copies or substantial portions of the Software.
// 
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//
// [1] http://www.sjgames.com/general/online_policy.html

#![recursion_limit = "1024"]
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate separator;

use errors::*;
use regex::Regex;
use separator::FixedPlaceSeparatable;
use std::env;
use std::fs::File;
use std::io::{self, Read, BufRead};

mod errors {
    error_chain! {
        foreign_links {
            Io(::std::io::Error);
            Parse(::std::num::ParseFloatError);
        }
    }
}

struct Input<'a> {
    source: Box<BufRead + 'a>,
}

impl<'a> Input<'a> {
    fn console(stdin: &'a io::Stdin) -> Input<'a> {
        Input { source: Box::new(stdin.lock()) }
    }

    fn file(path: &str) -> io::Result<Input<'a>> {
        File::open(path).map(|file| Input { source: Box::new(io::BufReader::new(file)) })
    }
}

impl<'a> Read for Input<'a> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.source.read(buf)
    }
}

impl<'a> BufRead for Input<'a> {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        self.source.fill_buf()
    }
    fn consume(&mut self, amt: usize) {
        self.source.consume(amt);
    }
}

lazy_static! {
    static ref POINTS: Regex = Regex::new(r"(?x)
        \[ (?P<points>  (?:\d{1,3},)*\d*\.?\d+) \]          |
        \[ (?P<disads> -(?:\d{1,3},)*\d*\.?\d+) \]          |
         < (?P<angle> -?(?:\d{1,3},)*\d*\.?\d+)  >          |
        \{ (?P<curly> -?(?:\d{1,3},)*\d*\.?\d+) \}          |
        \| (?P<pipe>  -?(?:\d{1,3},)*\d*\.?\d+) \|          |
           (?P<pounds>  (?:\d{1,3},)*\d*\.?\d+) \s* lbs?\.  |
           (?P<ounces>  (?:\d{1,3},)*\d*\.?\d+) \s* oz\.    |
           (?P<kilos>   (?:\d{1,3},)*\d*\.?\d+) \s* kg\.    |
           (?P<grams>   (?:\d{1,3},)*\d*\.?\d+) \s* g\.     |
        \$ (?P<oneT>  -?(?:\d{1,3},)*\d*\.?\d+)T            |
        \$ (?P<oneB>  -?(?:\d{1,3},)*\d*\.?\d+)B            |
        \$ (?P<oneM>  -?(?:\d{1,3},)*\d*\.?\d+)M            |
        \$ (?P<oneK>  -?(?:\d{1,3},)*\d*\.?\d+)K            |
        \$ (?P<one>   -?(?:\d{1,3},)*\d*\.?\d+)
    ").unwrap();
}

fn run() -> Result<()> {
    let stdin = io::stdin();

    let input = if let Some(arg) = env::args().nth(1) {
        Input::file(&arg)?
    } else {
        Input::console(&stdin)
    };

    let mut points: Vec<f64> = Vec::new();
    let mut disads: Vec<f64> = Vec::new();
    let mut angle:  Vec<f64> = Vec::new();
    let mut curly:  Vec<f64> = Vec::new();
    let mut pipe:   Vec<f64> = Vec::new();
    let mut money:  Vec<f64> = Vec::new();
    let mut weight: Vec<f64> = Vec::new();

    for line in input.lines() {
        for item in POINTS.captures_iter(&line?) {
            if let Some(p) = item.name("points") {
                points.push(p.as_str().replace(",", "").parse::<f64>()?);
            }
            if let Some(d) = item.name("disads") {
                disads.push(d.as_str().replace(",", "").parse::<f64>()?);
            }
            if let Some(a) = item.name("angle") {
                angle.push(a.as_str().replace(",", "").parse::<f64>()?);
            }
            if let Some(c) = item.name("curly") {
                curly.push(c.as_str().replace(",", "").parse::<f64>()?);
            }
            if let Some(p) = item.name("pipe") {
                pipe.push(p.as_str().replace(",", "").parse::<f64>()?);
            }
            if let Some(l) = item.name("pounds") {
                weight.push(l.as_str().replace(",", "").parse::<f64>()?);
            }
            if let Some(o) = item.name("ounces") {
                weight.push(o.as_str().replace(",", "").parse::<f64>()? / 16.);
            }
            if let Some(k) = item.name("kilos") {
                weight.push(k.as_str().replace(",", "").parse::<f64>()? * 2.205);
            }
            if let Some(g) = item.name("grams") {
                weight.push(g.as_str().replace(",", "").parse::<f64>()? / 453.593);
            }
            if let Some(m) = item.name("one") {
                money.push(m.as_str().replace(",", "").parse::<f64>()?);
            }
            if let Some(m) = item.name("oneK") {
                money.push(m.as_str().replace(",", "").parse::<f64>()? * 1000.);
            }
            if let Some(m) = item.name("oneM") {
                money.push(m.as_str().replace(",", "").parse::<f64>()? * 1000000.);
            }
            if let Some(m) = item.name("oneB") {
                money.push(m.as_str().replace(",", "").parse::<f64>()? * 1000000000.);
            }
            if let Some(m) = item.name("oneT") {
                money.push(m.as_str().replace(",", "").parse::<f64>()? * 1000000000000.);
            }
        }
    }

    let ppipe:   f64 = pipe.iter().sum();
    let pcurly:  f64 = curly.iter().sum();
    let pangle:  f64 = angle.iter().sum();

    let pweight: f64 = weight.iter().sum();
    let pmetric: f64 = pweight / 2.205;

    let pmoney:  f64 = money.iter().sum();

    let pdisads: f64 = disads.iter().sum();
    let ppoints: f64 = points.iter().sum();

    println!("{} points ({} disadvantages)", ppoints + pdisads, pdisads);
    println!("Equipment: ${}, {} lbs. ({} kg.)",
        pmoney.separated_string_with_fixed_place(2),
        pweight.separated_string_with_fixed_place(2),
        pmetric.separated_string_with_fixed_place(2));
    println!("Other sums: <{}> {{{}}} |{}|", pangle, pcurly, ppipe);

    Ok(())
}

fn main() {
    if let Err(ref e) = run() {
        use std::io::Write;
        let stderr = &mut ::std::io::stderr();
        let errmsg = "Error writing to stderr";

        writeln!(stderr, "error: {}", e).expect(errmsg);

        for e in e.iter().skip(1) {
            writeln!(stderr, "caused by: {}", e).expect(errmsg);
        }

        // The backtrace is not always generated. Try to run this example
        // with `RUST_BACKTRACE=1`.
        if let Some(backtrace) = e.backtrace() {
            writeln!(stderr, "backtrace: {:?}", backtrace).expect(errmsg);
        }

        ::std::process::exit(1);
    }
}
