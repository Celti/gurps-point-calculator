// GURPS is a trademark of Steve Jackson Games, and its rules and art are
// copyrighted by Steve Jackson Games. All rights are reserved by Steve Jackson
// Games. This game aid is the original creation of Patrick Burroughs and is
// released for free distribution, and not for resale, under the permissions
// granted in the Steve Jackson Games Online Policy [1].
//
// Copyright (c) 2017-2020 Patrick L. H. Burroughs.
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

use anyhow::Result;
use once_cell::sync::Lazy;
use rayon::iter::{IntoParallelRefIterator, ParallelBridge, ParallelIterator};
use regex::Regex;
use separator::FixedPlaceSeparatable;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
enum Points {
    Points(f64),
    Disads(f64),
    Angle(f64),
    Curly(f64),
    Pipe(f64),
    Money(f64),
    Weight(f64),
}

static POINTS: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"(?x)
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
        \$ (?P<one>   -?(?:\d{1,3},)*\d*\.?\d+)",
    )
    .unwrap()
});

fn main() -> Result<()> {
    let file = env::args().nth(1);
    let input: Box<dyn BufRead + Send + Sync> = match file {
        Some(file) => Box::new(BufReader::new(File::open(file).unwrap())),
        None => Box::new(BufReader::new(io::stdin())),
    };

    let points = input
        .lines()
        .par_bridge()
        .map(|line| {
            let mut acc: Vec<Points> = Vec::new();
            for item in POINTS.captures_iter(&line?) {
                if let Some(p) = item.name("points") {
                    acc.push(Points::Points(p.as_str().replace(",", "").parse::<f64>()?));
                }
                if let Some(d) = item.name("disads") {
                    acc.push(Points::Disads(d.as_str().replace(",", "").parse::<f64>()?));
                }
                if let Some(a) = item.name("angle") {
                    acc.push(Points::Angle(a.as_str().replace(",", "").parse::<f64>()?));
                }
                if let Some(c) = item.name("curly") {
                    acc.push(Points::Curly(c.as_str().replace(",", "").parse::<f64>()?));
                }
                if let Some(p) = item.name("pipe") {
                    acc.push(Points::Pipe(p.as_str().replace(",", "").parse::<f64>()?));
                }
                if let Some(l) = item.name("pounds") {
                    acc.push(Points::Weight(l.as_str().replace(",", "").parse::<f64>()?));
                }
                if let Some(o) = item.name("ounces") {
                    acc.push(Points::Weight(
                        o.as_str().replace(",", "").parse::<f64>()? / 16.,
                    ));
                }
                if let Some(k) = item.name("kilos") {
                    acc.push(Points::Weight(
                        k.as_str().replace(",", "").parse::<f64>()? * 2.205,
                    ));
                }
                if let Some(g) = item.name("grams") {
                    acc.push(Points::Weight(
                        g.as_str().replace(",", "").parse::<f64>()? / 453.593,
                    ));
                }
                if let Some(m) = item.name("one") {
                    acc.push(Points::Money(m.as_str().replace(",", "").parse::<f64>()?));
                }
                if let Some(m) = item.name("oneK") {
                    acc.push(Points::Money(
                        m.as_str().replace(",", "").parse::<f64>()? * 1000.,
                    ));
                }
                if let Some(m) = item.name("oneM") {
                    acc.push(Points::Money(
                        m.as_str().replace(",", "").parse::<f64>()? * 1000000.,
                    ));
                }
                if let Some(m) = item.name("oneB") {
                    acc.push(Points::Money(
                        m.as_str().replace(",", "").parse::<f64>()? * 1000000000.,
                    ));
                }
                if let Some(m) = item.name("oneT") {
                    acc.push(Points::Money(
                        m.as_str().replace(",", "").parse::<f64>()? * 1000000000000.,
                    ));
                }
            }
            Ok(acc)
        })
        .reduce(
            || Ok(Vec::new()),
            |a, b| -> Result<Vec<Points>> {
                match (a, b) {
                    (Err(e), Err(_)) | (Err(e), Ok(_)) | (Ok(_), Err(e)) => Err(e),
                    (Ok(mut a), Ok(mut b)) => {
                        a.append(&mut b);
                        Ok(a)
                    }
                }
            },
        )?;

    let ppipe: f64 = points
        .par_iter()
        .filter_map(|x| {
            if let Points::Pipe(i) = x {
                Some(i)
            } else {
                None
            }
        })
        .sum();
    let pcurly: f64 = points
        .par_iter()
        .filter_map(|x| {
            if let Points::Curly(i) = x {
                Some(i)
            } else {
                None
            }
        })
        .sum();
    let pangle: f64 = points
        .par_iter()
        .filter_map(|x| {
            if let Points::Angle(i) = x {
                Some(i)
            } else {
                None
            }
        })
        .sum();

    let pweight: f64 = points
        .par_iter()
        .filter_map(|x| {
            if let Points::Weight(i) = x {
                Some(i)
            } else {
                None
            }
        })
        .sum();
    let pmetric: f64 = pweight / 2.205;

    let pmoney: f64 = points
        .par_iter()
        .filter_map(|x| {
            if let Points::Money(i) = x {
                Some(i)
            } else {
                None
            }
        })
        .sum();

    let pdisads: f64 = points
        .par_iter()
        .filter_map(|x| {
            if let Points::Disads(i) = x {
                Some(i)
            } else {
                None
            }
        })
        .sum();
    let ppoints: f64 = points
        .par_iter()
        .filter_map(|x| {
            if let Points::Points(i) = x {
                Some(i)
            } else {
                None
            }
        })
        .sum();

    println!("{} points ({} disadvantages)", ppoints + pdisads, pdisads);
    println!(
        "Equipment: ${}, {} lbs. ({} kg.)",
        pmoney.separated_string_with_fixed_place(2),
        pweight.separated_string_with_fixed_place(2),
        pmetric.separated_string_with_fixed_place(2)
    );
    println!("Other sums: <{}> {{{}}} |{}|", pangle, pcurly, ppipe);

    Ok(())
}
