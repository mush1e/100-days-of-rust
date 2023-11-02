fn main() {
    let arr = vec![1,2,3,4,5,21,43,56,0,-2];
    let mut largest = &arr[0];

    for element in arr {
        if (largest > element) {
            largest = element;
        }
    }

    println("{}", largest);
}