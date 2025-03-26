use std::fmt::{format, Display};
use std::ops::{Add, Mul, Sub};
use std::cmp::PartialEq;

struct AritmeticnoZaporedje<T> {
    a0 : T, //(začetni člen)
    d : T, //(diferenca)
    ai : T //(trenutni)
}

use AritmeticnoZaporedje as AZ;

impl<T: PartialEq> PartialEq for AZ<T> {
    fn eq(&self, other: &Self) -> bool {
        self.a0 == other.a0 && self.d == other.d
    }
}

impl<T> AritmeticnoZaporedje<T>
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

pub trait Zaporedje<T> {
    fn name(&self) -> String;
    fn start(&self) -> T;
    fn k_th(&self, k : i64) -> T;
    fn contains(&self, a : T) -> T;
}

struct KonstantnoZaporedje<T> {
    a0 : T, //(začetni člen)
}

struct GeometrijskoZaporedje<T> {
    a0 : T, //(začetni člen)
    k : T, //(kvocient)
    ai : T //(trenutni)
}

struct FibonacijevoZaporedje<T> {
    f0 : T, //(začetni člen)
    f1 : T, //(prvi člen)
}


impl<T> Zaporedje<T> for KonstantnoZaporedje<T>
where
    T: PartialEq + Copy + Display,
{
    fn name(&self) -> String{
        format!("Konstantno zaporedje: člen {}", self.a0)
    }

    fn start(&self) -> T{
        self.a0
    }

    fn k_th(&self, k : i64) -> T{
        self.a0
    }

    fn contains(&self, a : T) -> bool{
       return self.a0 == a
    }
}

impl<T> Zaporedje<T> for AritmeticnoZaporedje<T>
where
    T: Add<Output = T> + PartialEq + Copy + Display,
{
    fn name(&self) -> String{
        format!("Aritmetično zaporedje: {}, {}, {}", self.a0, self.ai, self.d)
    }

    fn start(&self) -> T{
        self.a0
    }

    fn k_th(&self, k : i64) -> T{
        let mut k_d = self.d;

        for i in 0..k {
            k_d = k_d + self.d
        }

        return self.a0 + k_d
    }

    fn contains(&self, a : T) -> bool{
        let mut clen = self.a0;

        while a != clen {
            clen = clen + self.d
        }

        true
    }
}

impl<T> Zaporedje<T> for GeometrijskoZaporedje<T>
where
    T: Mul<Output = T> + Add<Output = T> + PartialEq + Copy + Display,
{
    fn name(&self) -> String{
        format!("Geometrijsko zaporedje: {}, {}, {}", self.a0, self.ai, self.k)
    }

    fn start(&self) -> T{
        self.a0
    }

    fn k_th(&self, j : i64) -> T{
        let mut an = self.a0;

        for i in 0..=j {
            an = an * self.k
        }
        return an
    }

    fn contains(&self, a : T) -> bool{
        let mut clen = self.a0;

        while a != clen {
            clen = clen * self.k
        }
    
        true
    }
}

impl<T> Zaporedje<T> for FibonacijevoZaporedje<T>
where
    T: Mul<Output = T> + Add<Output = T> + PartialEq + Copy + Display,
{
    fn name(&self) -> String{
        format!("Konstantno zaporedje: {}, {}", self.f0, self.f1)
    }

    fn start(&self) -> T{
        self.f0
    }

    fn k_th(&self, k : i64) -> T{
        let mut f_n = self.f0;
        let mut f_n1 = self.f1;
        let mut f_n2 = self.f0;


        for i in 1..=k {
            f_n = f_n1 + f_n2;
            f_n2 = f_n1;
            f_n1 = f_n; 
        }
        return f_n
    }

    fn contains(&self, a : T) -> bool{
        let mut clen = self.f0;
        let mut i = 0;

        while a != clen {
            clen = self.k_th(i);
            i += 1
        }

        true        
    }
}

fn zamaknjeno_zaporedje(zap : Zaporedje<T>, n : i64){
    
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

