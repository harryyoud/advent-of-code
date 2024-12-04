use aoc_lib::get_input_year;

pub fn get_input(day: usize) -> String {
    get_input_year(2024, day)
}

pub mod skip_nth {
    pub struct SkipNth<I: Iterator> {
        iter: I,
        n: usize,
        current: usize,
    }

    impl<I: Iterator> Iterator for SkipNth<I> {
        type Item = I::Item;

        fn next(&mut self) -> Option<Self::Item> {
            if self.n == self.current {
                self.current += 1;
                let _ = self.iter.next();
            }

            self.current += 1;
            self.iter.next()
        }
    }

    pub trait AocItertools: Iterator {
        fn skip_nth(self, n: usize) -> SkipNth<Self>
        where
            Self: Sized,
        {
            SkipNth {
                iter: self,
                n,
                current: 0,
            }
        }
    }

    impl<T> AocItertools for T where T: Iterator + ?Sized {}
}
