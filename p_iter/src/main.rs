fn main() {
    println!("Hello, world!");

    iter_in_for();
    into_iter_in_for();
    iter_mut_in_for();
    // sum 
    iter_sum();
    // 迭代器适配器
    iter_adapter();
}

fn iter_in_for () {
    let v1 = vec![1, 2, 3, 4];
    let iter_v1 = v1.iter();
    for item in iter_v1 {
        println!("v1 GO {}", item);
    }
}

// 拥有所有权的迭代器
fn into_iter_in_for () {
    let v1 = vec![1, 2, 3, 4];
    let iter_v1 = v1.into_iter();
    for item in iter_v1 {
        println!("v1 GO {}", item);
    }
}

// 迭代可变引用
fn iter_mut_in_for () {
    let mut v1 = vec![1, 2, 3, 4];
    let iter_v1 = v1.iter_mut();
    for item in iter_v1 {
        println!("v1 GO {}", item);
    }
}

fn iter_sum () {
    let v2 = vec![1, 2, 3];
    let iter_v2 = v2.iter();
    // 调用 sum 时它会获取迭代器的所有权。
    let sum: i32 = iter_v2.sum();
    println!("sum {}", sum);

    // 不能迭代
    // iter_v2.next();
    // borrow of moved value: `iter_v2`

    // for item in iter_v2 { // iter_v2 value used here after move
    //     println!("v2 GO {}", item);
    // }
}

// 展示如何使用闭包来自定义行为同时又复用 Iterator trait
fn iter_adapter () {
    let v: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<i32> = v.iter().map(|x| x +1).collect();
    println!("iter adapter {:?}", v2);
}


#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}