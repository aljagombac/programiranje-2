use std::fmt::format;
use std::ops::{Add, Mul, Sub};
use std::cmp::PartialEq;

struct ArtmeticnoZaporedje<T> {
    a0 : T, //(začetni člen)
    d : T, //(diferenca)
    ai : T //(trenutni)
}

use ArtmeticnoZaporedje as AZ;

impl<T: PartialEq> PartialEq for AZ<T> {
    fn eq(&self, other: &Self) -> bool {
        self.a0 == other.a0 && self.d == other.d
    }
}

impl<T> ArtmeticnoZaporedje<T>
where
    T: Add<Output = T> + Copy,
{
    fn new (a0 : T, d : T) -> AZ<T>{
        return AZ {
            a0 : a0, 
            d : d,
            ai : a0
        }
    }

    fn next (&mut self) -> T {
        let old = self.ai;
        self.ai = self.ai + self.d;
        old
    }

    fn n_th (&self, n : i64) -> T {
        let mut n_d = self.d;

        for i in 0..n {
            n_d = n_d + self.d
        }

        return self.a0 + n_d
    }

    fn reset (&mut self) {
        self.ai = self.a0
    }

    fn current(&self) -> T {
        return self.ai
    }

    fn sum(&self, n : i64) -> T {
        let mut vsota = self.a0;

        for i in 1..=n {
            vsota = vsota + self.n_th(i)
        }
        return vsota
    }

    fn vsota (&self, other : AZ<T>) -> AZ<T> {
        return AZ::new ( self.a0 + other.a0, self.d + other.d)
    }
}

enum BinOperacija {
    Plus,
    Minus,
    Times,
}

enum Izraz<T> {
    Konstanta(T),
    Operacija(Box<Izraz<T>>, BinOperacija, Box<Izraz<T>>),
}

//1 + (2 * 3) = Operacija(Box<Konstanta(1)>, Plus, Box<Operacija(Box<Konstanta(2)>, Times, Box<Konstanta(3)>)>)

impl<T> Izraz<T> 
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Copy, 
{
    fn eval (&self) -> T {
        match self {
            Izraz::Konstanta(n ) => *n,
            Izraz::Operacija(exp_l,opr,exp_d ) =>
                match opr {
                    BinOperacija::Minus => (exp_l).eval() - (exp_d).eval(),
                    BinOperacija::Plus => (exp_l).eval() + (exp_d).eval(),
                    BinOperacija::Times => (exp_l).eval() * (exp_d).eval()
                }
        }
    }

    fn collect (&self) -> u32 {
        match self {
            Izraz::Konstanta(n) => 1,
            Izraz::Operacija(exp_l,opr,exp_d ) => exp_l.collect() + exp_d.collect()
        }
    }

    fn izpis (&self) -> String {
        match self {
            Izraz::Konstanta(n) => format!("{}", n),
            Izraz::Operacija(exp_l,opr,exp_d ) => 
            match opr {
                BinOperacija::Minus => "( " +  (exp_l).izpis() + " - "  + (exp_d).izpis(),
                BinOperacija::Plus => (exp_l).izpis() + " + "  + (exp_d).izpis(),
                BinOperacija::Times => (exp_l).izpis() + " * "  + (exp_d).izpis()
            }
        }
    }

}





fn main() {
    println!("Hello, world!");
}

