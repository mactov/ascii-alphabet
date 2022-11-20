const H_FRAME: usize = 5;
const W_FRAME: usize = 5;

const NO_FRAME: [[u8;W_FRAME];H_FRAME] = [[0,0,0,0,0],[0,0,0,0,0],[0,0,0,0,0],[0,0,0,0,0],[0,0,0,0,0]];
const A_FRAME: [[u8;W_FRAME];H_FRAME] = [[0,0,1,0,0],[0,1,0,1,0],[1,0,0,0,1],[1,1,1,1,1],[1,0,0,0,1]];
const B_FRAME: [[u8;W_FRAME];H_FRAME] = [[1,1,1,1,0],[1,0,0,0,1],[1,1,1,1,0],[1,0,0,0,1],[1,1,1,1,0]];
const C_FRAME: [[u8;W_FRAME];H_FRAME] = [[0,1,1,1,1],[1,0,0,0,0],[1,0,0,0,0],[1,0,0,0,0],[0,1,1,1,1]];
const D_FRAME: [[u8;W_FRAME];H_FRAME] = [[1,1,1,1,0],[1,0,0,0,1],[1,0,0,0,1],[1,0,0,0,1],[1,1,1,1,0]];

fn ascii_print(s: &str) {
    let mut full_frame: [Vec<u8>;H_FRAME] = [Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new()];
    for c in s.chars() {
        match c {
            ' ' => stack_ascii_char(&mut full_frame, NO_FRAME),
            'A' => stack_ascii_char(&mut full_frame, A_FRAME),
            'B' => stack_ascii_char(&mut full_frame, B_FRAME),
            'C' => stack_ascii_char(&mut full_frame, C_FRAME),
            'D' => stack_ascii_char(&mut full_frame, D_FRAME),
            _ => continue
        };
    }
    //println!("{:?}", full_frame);
    dump_ascii(full_frame);
}

fn stack_ascii_char(full_frame: &mut [Vec<u8>;H_FRAME], c:[[u8;W_FRAME];H_FRAME]) -> &[Vec<u8>; H_FRAME] {
    for h in 0..H_FRAME {
        if (*full_frame[h]).len() > 0 {
            (*full_frame)[h].push(0);
        } //add 0 col for whitespace between letters
        for w in 0..W_FRAME {
            (*full_frame)[h].push(c[h][w]);
        }
    }
    full_frame
}

fn dump_ascii(frame: [Vec<u8>;H_FRAME]) {
    for h in 0..H_FRAME {
        for w in 0..frame[h].len() {
            if frame[h][w] == 1 {
                print!("\u{2588}");
            }
            else {
                print!(" ");
            }
        }
        println!();
    }
}

fn main() {
    println!();
    ascii_print("A BCD");
    println!();
}
