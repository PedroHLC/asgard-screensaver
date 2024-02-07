mod img;
use img::IMG; // Our artifact

use rand::Rng;
use std::thread::sleep;
use std::time::Duration;
use term_size::dimensions;
use console::{Term, Color, Style};


fn exit_handler() {
    // Undo these changes on exit
    let _ = Term::stdout().show_cursor();
    // Ctrl+C is a successfull exit
    std::process::exit(0);
}

fn get_scr_size() -> Vec<usize> {
    if let Some((width, height)) = dimensions() {
        vec![height, width]
    } else {
        vec![25, 80]
    }
}

fn get_img_size(st: &str) -> Vec<usize> {
    let lines : Vec<&str> = st.lines().collect();
    vec![lines.len(), lines.iter().map(|line| line.len()).max().unwrap_or_default()]
}

const RED2 : Color = Color::Color256(196);

fn get_color(rng: &mut impl Rng, x: usize, y: usize, t: i32, img: &[usize]) -> Color {
    let f = x as i32 - img[1] as i32 + t.abs();

    let off = rng.gen_range(0..16);

    for i in (0..=6).rev() {
        if f > y as i32 + i as i32 * 16 + off as i32 {
            if t >= 0 {
                return COLORS[1][i];
            } else if t < 0 {
                return COLORS[0][i];
            }
        }
    }
    
    return RED2;
}
const COLORS: [[Color; 7]; 2] = [
    [Color::White, Color::Green, Color::Yellow, Color::Blue, Color::Magenta, Color::Cyan, RED2],
    [Color::Green, Color::Yellow, Color::Blue, Color::Magenta, Color::Cyan, Color::White, RED2],
];

fn main() {
    let term = Term::stdout();
    let _ = term.hide_cursor();
    let _ = term.clear_screen();

    let mut rng = rand::thread_rng();
    let mut scr = get_scr_size();
    let img = get_img_size(IMG[0]);
    let img_lines = img[0];

    let _ = ctrlc::set_handler(exit_handler);

    let frames = img_lines * 6;
    let step = 6;

    loop {
        for t in (0..=step).chain((1..=frames).rev()).map(|x| x as i32 * (if x % 2 == 1 { 1 } else { -1 })) {
            let _ = term.move_cursor_to((scr[0] - img_lines) as usize / 2, 1);

            for (y, line) in IMG[0].lines().enumerate() {
                let spaces = (scr[1] - img[1]) / 2;
                let colored_line: String = line.chars().enumerate().map(|(x, c)| {
                    format!(
                        "{}",
                        Style::new().fg(get_color(&mut rng, x, y, t, &img)).apply_to(c)
                    )
                }).collect();
                let _ = term.move_cursor_right(spaces);
                println!("{}", colored_line);
            }

            if t == 0 {
                #[allow(const_item_mutation)]
                COLORS[0].rotate_left(rng.gen_range(1..=3));
                scr = get_scr_size();
                sleep(Duration::from_millis(300));
            }

            sleep(Duration::from_millis(70));
        }

        sleep(Duration::from_secs(1));
    }
}