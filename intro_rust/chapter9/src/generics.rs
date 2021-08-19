pub fn sample(){
    println!("[START] Generics Sample");
    sample_9_1_1();
    sample_9_1_2();
    sample_9_1_3();
    println!("[END] Generics Sample");
}

fn sample_9_1_1(){
    let v = vec![1,2,3,4,5];
    for i in &v{ println!("{}",i); }

    let mut v :Vec<i32> = Vec::new();
    v.push(1);
    for i in &v{ println!("{}",i); }

    let mut v = Vec::<i32>::new();
    v.push(2);
    for i in &v{ println!("{}",i); }
}

// Not Generics Code
// 
fn sample_9_1_2(){
    let v = [10,20,30,40];
    print_i32(&v);
    let v = ['A','B','C'];
    print_char(&v);
}

fn print_i32(a:&[i32]){
    print!("a is ");
    for i in a {print!("{} ",i); }
    println!("");
}
fn print_char(a:&[char]){
    print!("a is ");
    for i in a {print!("{} ",i); }
    println!("");
}

/*
 *  Genrics
 * */
fn sample_9_1_3(){
    let v = [10,20,30,40];
    print(&v);
    let v= ['A','B','C','D'];
    print(&v);
    let v = ["one","two","three","four"];
    print(&v);
}

/**
 *  
 */
fn print<T>( a:&[T] ) where T : std::fmt::Debug{
    print!("a is ");
    for i in a {
        print!("{:?} " , i );
    }
    println!("");
}

