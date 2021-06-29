use std::io;
use rand::Rng;
fn main() { 
    println!("Hello, world!");
    let mut guess=String::new();
    io::stdin().read_line(&mut guess);
    println!("You guess {}",guess);
    let re={
        let mut i=1;
        let mut c=0;
        loop {
            if c>10 {break i}
            i=i*2;
            c=c+1;
            }
        };
        println!("{}",re);
        let mut s:[i32;6]=[1,2,3,4,5,6];
        let a:i32=foo (& mut s);
        println!("{}",a);


    }

    fn foo(s:& [i32])->i32{
        s[2]
    }

    struct Rec {
        a:i32
    }
    
    impl Rec {
        fn foo( self:& mut Rec){
            self.a=2;
        }
    }

fn footwo(mut a: Rec)->i32{
    let mut i1=&  a;
    let mut i2=&  a;
    let mut f=Rec{a:1};
    i1=& mut f;
   // i2=& mut ();
    1
}

enum E{
    E(i32)
}

fn foo31(e:&E){
    1+1;
    ();
}

fn foo3 (e:E)->i32{
    foo31(&e);
    match e {
        E::E( a)=>a,
    }

}