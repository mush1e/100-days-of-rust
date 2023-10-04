fn remove_zeros(v: &mut Vec<i32>) {
    for (i, t) in v.iter().enumerate().rev() {
        if *t == 0 {
            v.remove(i);
            v.shrink_to_fit();
        }
    }
}
 fn main() {
    let mut a = vec![0,1,2,3];
    remove_zeros(&mut a);
 } 