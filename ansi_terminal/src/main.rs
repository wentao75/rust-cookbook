//! # ANSI Terminal
//! 介绍如何使用`ansi_term`库和控制ANSI终端上的颜色以及格式，如蓝色粗体文本或黄色下划线文本。
//!
//! 两个重要的数据结构`ansi_term::ANSIString`和`Style`。
//! `Style`控制样式信息：颜色，是否粗体，是否闪烁等等；
//! 一个`ANSIString`是和一个`Style`配对的字符串。
//!
use ansi_term::Colour;
use ansi_term::Style;

fn main() {
    println!(
        "This is {} in color, {} in color and {} in color",
        Colour::Red.paint("red"),
        Colour::Blue.paint("blue"),
        Colour::Green.paint("green")
    );

    println!(
        "{} and this is not",
        Style::new().bold().paint("This is Bold")
    );

    println!(
        "{}, {} and {}",
        Colour::Green.paint("This is coloured"),
        Style::new().bold().paint("this is bold"),
        Colour::Green.underline().paint("This is bold and colored")
    );

    println!(
        "{}, {}, {}, {}, {}, {}",
        Style::new().dimmed().paint("dimmed"),
        Style::new().italic().paint("italic"),
        Style::new().blink().paint("blink"),
        Style::new().reverse().paint("reverse"),
        Style::new().hidden().paint("hidden"),
        Style::new().on(Colour::Blue).paint("on BLUE")
    );
}
