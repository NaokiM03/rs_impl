struct Sample {
}

impl Sample {
    fn a() {
        println!("a");
    }

    fn b() {
        println!("b");
    }

    fn c() {
        println!("c");
    }
}

mod fake_module {
    use super::Sample;

    impl Sample {
        fn x() {
            println!("x");
        }

        fn y() {
            println!("y");
        }

        fn z() {
            println!("z");
        }
    }
}

fn main() {
    Sample::a();
    Sample::b();
    Sample::c();
    Sample::x();
    Sample::y();
    Sample::z();
}
