struct ArtmeticnoZaporedje {
    a0 : i64, //(začetni člen)
    d : i64, //(diferenca)
    ai : i64 //(trenutni)
}

use std::fmt::format;

use ArtmeticnoZaporedje as AZ;

impl ArtmeticnoZaporedje {
    fn new (a0 : i64, d : i64) -> AZ{
        return AZ {
            a0 : a0, 
            d : d,
            ai : a0
        }
    }

    fn next (&mut self) -> i64 {
        self.ai += self.d;
        self.ai -self.d
    }

    fn n_th (&self, n : i64) -> i64 {
        return self.a0 + n * self.d
    }

    fn reset (&mut self) {
        self.ai = self.a0
    }

    fn current(&self) -> i64 {
        return self.ai
    }

    fn sum(&self, n : i64) -> i64 {
        let mut vsota = 0;

        for i in 0..=n {
            vsota += self.n_th(i)
        }
        return vsota
    }

    fn vsota (&self, other : AZ) -> AZ {
        return AZ::new ( self.a0 + other.a0, self.d + other.d)
    }
}

struct GeometrijskoZaporedje {
    a0 : i64, //(začetni člen)
    k : i64, //(kvocient)
    ai : i64 //(trenutni)
}

use GeometrijskoZaporedje as GZ;

impl GeometrijskoZaporedje {
    fn new (a0 : i64, k : i64) -> GZ{
        return GZ {
            a0 : a0, 
            k : k,
            ai : a0
        }
    }

    fn next (&mut self) -> i64 {
        self.ai *= self.k;
        self.ai / self.k
    }

    fn n_th (&self, n : i64) -> i64 {
        let mut an = self.a0;

        for i in 0..=n {
            an *= self.k
        }
        return an
    }

    fn reset (&mut self) {
        self.ai = self.a0
    }

    fn current(&self) -> i64 {
        return self.ai
    }

    fn sum(&self, n : i64) -> i64 {
        let mut vsota = 0;

        for i in 0..=n {
            vsota += self.n_th(i)
        }
        return vsota
    }
    
    fn produkt (&self, other : GZ) -> GZ {
        return GZ::new (self.a0 * other.a0, self.k * other.k)
    }
}

enum BinOperacija {
    Plus,
    Minus,
    Times,
}

enum Izraz {
    Konstanta(u32),
    Operacija(Box<Izraz>, BinOperacija, Box<Izraz>),
}

//1 + (2 * 3) = Operacija(Box<Konstanta(1)>, Plus, Box<Operacija(Box<Konstanta(2)>, Times, Box<Konstanta(3)>)>)

impl Izraz {
    fn eval (&self) -> u32 {
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
