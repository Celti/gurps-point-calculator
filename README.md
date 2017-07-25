GURPS Point Calculator
======================

A calculator for [SJGames](http://www.sjgames.com/) [GURPS™ 4th Edition](http://www.sjgames.com/gurps/) characters. It takes a GURPS™ character sheet in plain text (as per the standard in-book format for sample characters and templates), searches for values, sums them, and prints them on standard output.

It searches for the following:
  - Standard GURPS™ point values in square brackets, e.g.: `[100]`.
  - Similar values in angle brackets, curly brackets, or pipes, e.g.: `<5>`, `{15}`, or `|50|`.
  - Dollar values, as demarcated by a preceding dollar sign (`$`) and optionally ending in `K`, `M`, `B`, or `T` to respectively indicate thousands, millions, billions, or trillions of dollars.
  - Weight values, in pounds (indicated by the suffix `lb.` or `lbs.`), ounces (`oz.`), kilograms (`kg.`), or grams (`g.`).

It will take input either on standard input, or from a file provided as an argument.

Sample Output
=============
```
40 points (-15 disadvantages)
Equipment: $800,060,402.00, 16.98 lbs. (7.70 kg.)
Other sums: <5> {95} |13.875|
```

Downloading
===========
Either clone this repository and build it with `cargo`...

```
$ git clone https://github.com/Celti/gurps-point-calculator.git
$ cargo build --release
$ target/release/gurps-point-calculator
```

...or see the [releases page](https://github.com/Celti/gurps-point-calculator/releases) for Linux and Windows 64-bit static binaries.

License
=======

GURPS is a trademark of Steve Jackson Games, and its rules and art are
copyrighted by Steve Jackson Games. All rights are reserved by Steve Jackson
Games. This game aid is the original creation of Patrick Burroughs and is
released for free distribution, and not for resale, under the permissions
granted in the [Steve Jackson Games Online Policy](http://www.sjgames.com/general/online_policy.html).

Copyright (c) 2017 Patrick L. H. Burroughs.

Permission is hereby granted, free of charge, to any person obtaining a copy of
this software and associated documentation files (the "Software"), to deal in
the Software for non-commercial use, including without limitation the rights to
use, copy, modify, merge, publish, distribute, or sublicense under a compatible
license, and to permit persons to whom the Software is furnished to do so,
subject to the following condition:

The above copyright and trademark notices and this permission notice shall be
included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

