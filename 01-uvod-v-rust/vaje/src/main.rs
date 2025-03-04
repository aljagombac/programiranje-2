use core::panic;
//use std::fmt::Display;

/// Skupaj preverite in pokomentirajte kvize iz [učbenika](https://rust-book.cs.brown.edu/ch03-00-common-programming-concepts.html)

/// ------------------------------------------------------------------------------------------------

/// Napišite funkcijo `fib`, ki sprejme začetna člena fibbonacijevega zaporedja, število `n` in vrne `n`-ti člen zaporedja

fn fib(a0: u32, a1: u32, n: u32) -> u32 {
    let mut prvi_clen = a0;
    let mut drugi_clen = a1;
    let mut index = 0;

    loop {
        if index >= n {
            return prvi_clen;
        }
        let vsota = prvi_clen + drugi_clen;
        prvi_clen = drugi_clen;
        drugi_clen = vsota;
        index += 1;    
   
    }
}

/// ------------------------------------------------------------------------------------------------

/// Napišite funkcijo `je_prestopno`, ki za podano leto preveri, ali je prestopno

fn je_prestopno (leto: u32) -> bool {
    return leto % 4 == 0 && (leto % 100 != 0 || (leto % 100 == 0 && leto % 400 == 0));
}

/// ------------------------------------------------------------------------------------------------

/// Napišite funkcijo `je_veljaven_datum(datum: Date) -> bool`, ki preveri, ali je datum veljaven

// Dan, mesec, leto
type Date = (u32, u32, u32);

fn dolzina_meseca(datum: Date) -> u32 {
    match datum.1 {
        4 | 6 | 9 | 11 => 30,
        2 => if je_prestopno(datum.2) {
            29
        } else {
            28
        },
        _ => 31,
    }
} 

fn je_veljaven_datum(datum: Date) -> bool {
    if datum.1 < 1 || datum.1 > 12 {
        false
    } else if datum.0 < 1 || datum.0 > dolzina_meseca(datum) {
        false
    } else {
        true
    }
}

/// ------------------------------------------------------------------------------------------------

/// Napišite funkcijo `iteracija(mut start: u32, fun: fn(u32) -> u32, cond: fn(u32) -> bool) -> u32`, ki sprejme iteracijsko funkcijo, zaustavitveni pogoj in začetno vrednost.
/// Iteracijsko funkcijo zaporedoma uporablja, dokler za rezultat ne velja zaustavitveni pogoj, in vrne prvi rezultat, ki zadošča zaustavitvenemu pogoju.

fn iteracija(mut start: u32, fun: fn(u32) -> u32, cond: fn(u32) -> bool) -> u32 {
    let mut rez1 = start;
    let mut rez_prej = 0;

    loop {
        if cond(rez1) {
            return rez_prej;
        }
        rez_prej = rez1;
        rez1 = fun(start);
        start += 1;
    }
}

/// ------------------------------------------------------------------------------------------------

/// Napišite funkcijo, ki izračuna ničlo zvezne funkcije s pomočjo bisekcije.
/// Postopek bisekcije je sledeč:
/// 1. Izberemo interval [a, b], kjer je f(a) * f(b) < 0
/// 2. Izračunamo sredino intervala c = (a + b) / 2
/// 3. Če je |f(c)| < prec ali je dolžina intervala manjša od določene natančnosti, vrnemo c
/// 4. Če ni, izberemo nov interval [a, b] glede na predznak f(c)
/// 5. Ponavljamo korake 2-4

fn bisekcija(a: f64, b: f64, fun: fn(f64) -> f64, prec: f64) -> f64 {
    let mut a0 = a;
    let mut b0 = b;
    let mut c = (a + b) / 2.0;

    while fun(c).abs() >= prec && (b0 - a0) >= prec {
        if fun(a0) * fun(c) < 0.0 {
            b0 = c;
        }
        else {
            a0 = c;
        }
        c = (a0 + b0) / 2.0;
    };

    return c;
}

/// ------------------------------------------------------------------------------------------------

/// Popravite igro ugibanja iz prejšnje naloge, da bo delovala sledeče
/// Uporabnika sprašujemo po novi številki, vse dokler so števila, ki jih vpisuje del 
/// nekega aritmetičnega zaporedja
/// Če uporabnik vpiše neveljavno število to ni napaka, program za pogoj aritmetičnega 
/// zaporedja upošteva samo veljavno vpisana števila.

fn guessing_game() {
    panic!("Not implemented");
}

/// ------------------------------------------------------------------------------------------------
/// Napišite funkcijo `fn mat_mul(a: [[u32; 2]; 2], b: [[u32; 2]; 2]) -> [[u32; 2]; 2]`, ki matriki `a` in `b` zmnoži in vrne rezultat

fn mat_mul(a: [[u32; 2]; 2], b: [[u32; 2]; 2]) -> [[u32; 2]; 2] {
    let mut c : [[u32; 2]; 2] = [[0,0],[0,0]];

    for i in 0..2 {
        for j in 0..2 {
            c[i][j] = a[i][0] * b[0][j] + a[i][1] * b[1][j];
        }
    }
    return c
}

/// ------------------------------------------------------------------------------------------------
/// Napišite funkcijo `ordered`, ki sprejme tabelo števil in vrne `true`, če so števila urejena (padajoče ali naraščajoče) in `false` sicer.

fn ordered(arr: &[u32]) -> bool {
    let mut pad = true;
    let mut nar = true;

    let mut i = 1;
    while (pad || nar) && i < arr.len() {
        if arr[i] > arr[i-1] {
            pad = false;
        }
        else if arr[i] < arr[i-1] {
            nar = false;
        }
        i += 1;
    }
    return pad || nar
}


fn vsebuje<T : PartialEq>(v: &Vec<T>, x : &T) -> bool {
    for y in v {
      if x == y {
        return true
      }
    }
    return false
}

/// ------------------------------------------------------------------------------------------------
/// Hitro potenciranje
/// Napišite funkcijo `fn pow(mut x: u32, mut n: u32) -> u32`, ki izračuna `x` na potenco `n` v času O(log n)
/// Hitro potenciranje izgleda tako:
/// 1. Če je `n` sodo, potem je `x^n = (x^(n/2))^2`
/// 2. Če je `n` liho, potem je `x^n = (x^2)^(n/2)`
/// 3. Če je `n = 0`, potem je `x^n = 1` 

fn pow(mut x: u32, mut n: u32) -> u32 {
    if n == 0 {
        return 1
    }
    else if n % 2 == 0 {
        n = n / 2;
        return pow(x, n) * pow(x, n);
    }
    else {
        n = (n - 1) / 2;
        return (pow(x, n) * pow(x, n)) * x;
    }
}

/// ------------------------------------------------------------------------------------------------
/// Prepišite hitro potenciranje v iterativno obliko

//fn pow_iter(mut x: u32, mut n: u32) -> u32 {}
//    let mut x0 = 1 
//    let mut x_pol = 1;
//    let mut x_prej = 1;
//
//
//    for i in 1..n {
//        if i % 2 == 0 {
//            x_pol = x_pol * x_pol;
//        }
//        else {
//            x_prej = x_pol * x;
//            x_pol = 
//        }
//    }
//    return xi
//}


/// ------------------------------------------------------------------------------------------------
/// Hitro potenciranje deluje tudi, če nas zanima samo ostanek po deljenju z nekim številom `m`
/// Napišite funkcijo `fn pow_mod(mut x: u32, mut n: u32, m: u32) -> u32`, ki izračuna `x` 
/// na potenco `n` in vrne ostanek po deljenju z `m`
/// Postopek je enak, le da pri vsakem izračunu vrnemo ostanek pri deljenju z `m`

fn pow_mod(mut x: u32, mut n: u32, m: u32) -> u32 {

    return pow(x, n) % m
}

/// ------------------------------------------------------------------------------------------------
/// Urejanje z izbiranjem
/// Napišite funkcijo `fn selection_sort(arr: &mut [u32])`, ki uredi tabelo `arr` z uporabo algoritma urejanja z izbiranjem

fn selection_sort(arr: &mut [u32]) {
    let l = arr.len();

    for i in 0..l {
        let mut min_idx = i;
        let a = arr[min_idx];

        for j in (i+1)..l {
            if arr[j] < arr[min_idx] {
                min_idx = j
            }
        }
        arr[i] = arr[min_idx];
        arr[min_idx] = a;
    }
}

/// ------------------------------------------------------------------------------------------------
/// Napišite program, ki izpiše piramido višine `n` iz zvezdic

fn pyramid(n: u32) {    
    for i in 1..(n+1) {
        let presledki = " ".repeat((n - i) as usize);
        let vrstica = "*".repeat((1 + 2 * (i-1)) as usize);
        println!("{}", [presledki, vrstica].join(""));
    }
}
///       *
///     * * *
///   * * * * *
/// * * * * * * *
///    *
///   ***
//   *****
/// *******
/// ------------------------------------------------------------------------------------------------
/// Napišite program, ki izpiše piramido črk angleške abecede višine `n`, lahkom predpostavite, da bo n največ 26.
///       A
///     A B A
///   A B C B A
/// A B C D C B A

fn abc_pyramid(n: u32) {

    for i in 1..(n+1) { 
        let presledki = " ".repeat((n - i) as usize);
        //let alpha = ('a'..='z').into_iter().collect::<Vec<char>>()
        let alpha = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

        for j in 0..alpha.len(){
            let sredina = alpha.get(j..=j);
            let desno = alpha.get(..j);
            match (sredina, desno) {
                (Some(s), Some(d)) => println!("{}", [presledki, d, s, (d.chars().rev())].join("")),
                (None, _) => None,
                (Some(_), None) => None
            }
        }
    }
}

fn main() {
    pyramid(3);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = main();
        assert_eq!(result, ());
    }

    #[test]
    fn test_fib() {
        println!("{}", fib(0, 1, 4));
    }

    #[test]
    fn test_je_prestopno() {
        println!("{}", je_prestopno(2024));
    }

    #[test]
    fn test_dolzina_meseca() {
        assert_eq!(31, dolzina_meseca((1,1,2)));
    }

    #[test]
    fn test_je_veljaven_datum() {
        println!("{}", je_veljaven_datum((0,1,2)));
    }

    #[test]
    fn test_mat_mul() {
        assert_eq!([[0,1],[6,0]], mat_mul([[1,0],[0,2]], [[0,1],[3,0]]));
    }

    #[test]
    fn test_ordered() {
        assert_eq!(true, ordered(&[1,2,3,4]));
        assert_eq!(true, ordered(&[1,1,1]));
        assert_eq!(false, ordered(&[1,0,5]));
    }

    #[test]
    fn test_selection_sort() {
        let mut a = [1,5,4,7,2,8,6,2,6,8,4,134,6,8,3,6,8];
        selection_sort(&mut a);
        assert_eq!([1, 2, 2, 3, 4, 4, 5, 6, 6, 6, 6, 7, 8, 8, 8, 8, 134], a);
    }
}
