use bigdecimal::BigDecimal;

fn main() {
    let one = BigDecimal::from(1i8);
    let two = one.clone() + one.clone();
    let four = two.clone() + two.clone();

    let mut a = one.clone();
    let mut b = two.sqrt().unwrap().inverse();
    let mut t = four.inverse();
    let mut p = one.clone();

    let n = 10000;

    for _ in 0..((n as f64).log2().ceil() as usize) {
        let na = (a.clone() + b.clone()) / two.clone();
        let nb = (a.clone() * b.clone()).sqrt().unwrap();
        let nt = t.clone() - p.clone() * (a.clone() - na.clone()).square();
        let np = p.clone() * two.clone();

        a = na;
        b = nb;
        t = nt;
        p = np;
    }

    let pi = (a + b).square() / (t * four);

    println!("{:.1000}", pi);
}
