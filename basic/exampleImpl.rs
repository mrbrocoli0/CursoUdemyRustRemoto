struct Example {
        number: i32,
}

impl Example {
        fn boo() {
                    println!("boo! Example::boo() was called!");
                        }

            fn answer(&mut self) {
                        self.number += 42;
                            }

                fn get_number(&self) -> i32 {
                            self.number
                                    }
}

trait Thingy {
        fn do_thingy(&self);
}

impl Thingy for Example {
        fn do_thingy(&self) {
                    println!("doing a thing! also, number is {}!", self.number);
                        }
}
