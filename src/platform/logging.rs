pub trait Logger{
    fn log(&mut self, msg: String);
}

pub struct Stdout{}

impl Logger for Stdout {
    fn log(&mut self, msg: String) {
        println!("{}", msg);
    }
}

pub struct NoLog{}

#[allow(unused_variables)]
impl Logger for NoLog {
    fn log(&mut self, msg: String) {}
}

