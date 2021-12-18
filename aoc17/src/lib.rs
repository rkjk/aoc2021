use std::cmp::max;

fn nsum(n: i32) -> i32 {
    n * (n + 1) / 2
}

fn simulate(vxi: i32, vyi: i32, xmin: i32, xmax: i32, ymin: i32, ymax: i32) -> Option<i32> {
    let mut x = 0;
    let mut y = 0;
    let mut maxy = 0;
    let mut flag = false;

    let mut vx = vxi;
    let mut vy = vyi;
    loop {
        x += vx;
        y += vy;
        maxy = max(maxy, y);
        if vx > 0 {
            vx -= 1;
        }
        else if vx < 0 {
            vx += 1;
        }
        vy -= 1;
        if x >= xmin && x <= xmax && y >= ymin && y <= ymax {
            flag = true;
        }
        if (vx == 0 && vy <= 0 && y < ymin)  || (vx == 0 && x < xmin || x > xmax) {
            break;
        }
    }
    match flag {
        true => Some(maxy),
        false => None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        let mut maxval = 0;
        let mut maxx = -1;
        let mut maxy = -1;
        let (xmin, xmax, ymin, ymax) = (20, 30, -10, -5);
        for vx in 1..10 {
            for vy in 1..10 {
                match simulate(vx, vy, xmin, xmax, ymin, ymax) {
                    Some(v) => {
                        if (v > maxval)
                        {
                            maxval = v;
                            maxx = vx;
                            maxy = vy;
                        }
                    },
                    None => (),
                }
            }
        }
        println!("Part1: {}, maxx: {}, maxy: {}", maxval, maxx, maxy);
    }

    #[test]
    fn actual() {
        let mut maxval = 0;
        let mut maxx = -1;
        let mut maxy = -1;
        let mut count = 0;
        let (xmin, xmax, ymin, ymax) = (70, 125, -159, -121);
        for vx in (1..200) {
            for vy in -200..200 {
                match simulate(vx, vy, xmin, xmax, ymin, ymax) {
                    Some(v) => {
                        count += 1;
                        if (v > maxval)
                        {
                            maxval = v;
                            maxx = vx;
                            maxy = vy;
                        }
                    },
                    None => (),
                }
            }
        }
        println!("Part1: {}, maxx: {}, maxy: {}", maxval, maxx, maxy);
        println!("Part2: {}", count);
    }
}
