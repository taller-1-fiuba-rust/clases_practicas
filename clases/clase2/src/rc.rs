use std::rc::Rc;

fn main() {

    // Rc implementa Deref
    let mi_vector = Rc::new(vec![1.0, 2.0, 3.0]);
    let a = Rc::clone(&mi_vector);

    consumir(mi_vector);
    consumir(a);

    //mi_vector.push(5.0);
}

fn consumir(v: Rc<Vec<f64>>) {
    println!("{:?}", v)
}