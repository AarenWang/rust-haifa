// #![feature(unboxed_closures, fn_traits)]
#![feature(unboxed_closures, fn_traits)]
struct Closure {
    env_var: u32,
}
impl FnOnce<()> for Closure {
    type Output = u32;
    extern "rust-call" fn call_once(self, args: ()) -> u32 {
        println!("call it FnOnce()");
        self.env_var + 2
    }
}
impl FnMut<()> for Closure {
    extern "rust-call" fn call_mut(&mut self, args: ()) -> u32 {
        println!("call it FnMut()");
        self.env_var + 2
    }
}
impl Fn<()> for Closure {
    extern "rust-call" fn call(&self, args: ()) -> u32 {
        println!("call it Fn()");
        self.env_var + 2
    }
    
}
fn call_it<F: Fn() -> u32>(f: &F) -> u32 {
    f()
}
fn call_it_mut<F: FnMut() -> u32>(f: &mut F) -> u32 {
    f()
}
fn call_it_once<F: FnOnce() -> u32>(f: F) -> u32 {
    f()
}
fn main() {
    let env_var = 1;
    let mut c = Closure { env_var: env_var };
    c();
    c.call(());
    c.call_mut(());
    c.call_once(());
    let mut c = Closure { env_var: env_var };
    {
        assert_eq!(3, call_it(&c));
    }
    {
        assert_eq!(3, call_it_mut(&mut c));
    }
    {
        assert_eq!(3, call_it_once(c));
    }
}
