pub fn sample(){
    println!("[START] 9.2 trait");
    sample_9_2_2();
    sample_9_2_3();
    sample_9_2_4();
    println!("[END] 9.2 trait");
}


struct Rectangle{
width : f32,
      height:f32,
}
struct Triangle{
base:f32,
         height :f32,
}
struct Circle{
radius :f32
}

trait CalcArea{
    fn calc_area(&self) -> f32;
}

impl CalcArea for Rectangle{
    fn calc_area( &self ) -> f32{ self.width * self.height }
}
impl CalcArea for Triangle{
    fn calc_area( &self ) -> f32{ self.base * self.height * 0.5  }
}

impl CalcArea for Circle{
    fn calc_area( &self ) -> f32{ self.radius * self.radius * 3.14 }
}

/**
 * trait 
 */
fn sample_9_2_2(){
    let rect = Rectangle{ width: 10.0 , height: 10.0};
    let tri = Triangle{ base: 10.0 , height: 10.0};
    let cir = Circle{ radius : 10.0 };


    println!("are is {}" , rect.calc_area());
    println!("are is {}" , tri.calc_area());
    println!("are is {}" , cir.calc_area());

}


trait ExprString{ fn expr_str(&self)->String{ "幅 X 高さ = ".to_string()}}

impl ExprString for Rectangle{}
impl ExprString for Triangle{
    fn expr_str(&self) ->String{
        "底辺 X 高さ ÷ 2 =".to_string()
    }
}
impl ExprString for Circle{
    fn expr_str(&self) ->String{
        "半径 X 半径 X π =".to_string()
    }
}


fn sample_9_2_3(){
    let rect = Rectangle{ width: 10.0 , height: 10.0};
    let tri = Triangle{ base: 10.0 , height: 10.0};
    let cir = Circle{ radius : 10.0 };


    println!("are {} {}" , rect.expr_str(),rect.calc_area());
    println!("are {} {}" , tri.expr_str(), tri.calc_area());
    println!("are {} {}" , cir.expr_str() , cir.calc_area());
}

trait ToNumber{
    fn to_i(&self) -> i32;
}
impl ToNumber for String{
    fn to_i(&self) -> i32{
        match self.parse::<i32>(){
            Ok(n) => n ,
            Err(_) => 0,
        }
    }
}

fn sample_9_2_4  (){
    let s = String::from("100");
    println!("{}", s.to_i());
    
}
