use polynomial::Polynomial;

fn main() {
    let poly = Polynomial::new(vec![1, 2, 3]);
    assert_eq!("1+2*x+3*x^2", poly.pretty("x"));
    println!()
}
