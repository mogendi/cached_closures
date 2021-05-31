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

    fn execute(&mut self, x: U) -> V {
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn multivalue(){

        let mut c = Cacher::new(|x| x);
        let a = c.execute(1);
        let b = c.execute(2);
        let d = c.execute(2);
        assert_eq!(b, 2);
        assert_eq!(a, 1);
        assert_eq!(d, 2);
    }
}