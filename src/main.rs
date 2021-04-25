use std::collections::HashMap;

struct Cacher<T, U, V>
where 
T: Fn(U) -> V,
U: std::cmp::Eq + std::hash::Hash + Copy,
V: Copy
{
    calc: T,
    value: HashMap<U,V>,
}

impl<T, U, V> Cacher<T, U, V>
where 
T: Fn(U) -> V,
U: std::cmp::Eq + std::hash::Hash + Copy,
V: Copy
{
    fn new(calc: T)->Cacher<T, U, V>{
        Cacher{
            calc,
            value: HashMap::new()
        }
    }

    fn value(&mut self, x: U) -> V {
        match self.value.get(&x) {
            Some(y) => *y,
            None => {
                let v = (self.calc)(x);
                self.value.entry(x).or_insert(v);
                v
            },
        }
    }
}

#[test]
#[allow(unused)]
fn multivalue(){
    let mut c = Cacher::new(|x| x);
    let a = Cacher::value(&mut c, 1);
    let b = Cacher::value(&mut c, 2);
    assert_eq!(b, 2);
    assert_eq!(a, 1);
}

fn main(){}
