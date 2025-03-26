// Napišite različne funkcije, ki kot argument sprejmejo zaprtje (closure) in ga pokličejo z različnimi argumenti, ki ustrezajo sledečim ocaml tipom:
// Tip zaprtja naj bo čim bolj splošen (Fn, FnOnce, FnMut).
//  apply_int: (int -> int) -> int -> int
//  apply: ('a -> 'b) -> 'a -> 'b
//  apply2: ('a -> 'a -> 'b) -> 'a -> 'a -> 'b
//  map: ('a -> 'b) -> 'a list -> 'b list  (Uporabite Vec<T> namesto list, predpostavite, da funkcija ne spremeni elementov seznama)
//  ponavljaj: int -> ('a -> 'a) -> 'a -> 'a // Ponovi funkcijo n-krat
//  filter: ('a -> bool) -> 'a list -> 'a list // Vrne seznam elementov, ki zadoščajo pogoju - uporabite Vec<T> namesto list in že vgrajeno funkcijo filter

fn apply_int<F>(f : F, n : i32) -> i32 where
    F: Fn(i32) -> i32 {
        f(n)
    }

fn apply<F, T, U>(f : F, a : T) -> U where
    F: Fn(T) -> U {
        f(a)
    }

fn apply2<F, T, U>(f : F, a1 : T, a2 : T) -> U where
    F: Fn(T, T) -> U {
        f(a1, a2)
    }

fn map<F, T, U>(f : F, ls : Vec<T>) -> Vec<U> where
    F: Fn(T) -> U {
        let mut list = Vec::new();

        for el in ls {
            list.push(f(el));
        }

        list
    }

fn ponavljaj<F, T>(n : i32, f : F, a : T) -> T where 
    F: Fn(T) -> T {
        let mut b = a;
        
        for i in 1..=n {
            f(b);
            b = f(b);
        }
        
        b
    }

fn filter<F,T>(f : F, ls : Vec<T>) -> Vec<T> where 
    F: FnMut(T) -> bool {
        ls.iter()
        .filter(f)
        .collect()
    }

// Vzemite zaporedja iz prejšnjih vaj in naredite nov objekt, ki sprejme zaporedje in ga naredi iterabilnega

// Iteratorji

// Napišite funkcijo, ki sprejme vektor XYZ in s pomočjo iteratorja naredi W:
// števil in izpiše vsako v svojo vrstico
// nizov in izpiše njihove dolžine
// nizov in vrne vsoto njihovih dolžin
// vektor parov (i32, i32) in vrne vsoto njihovih pozitivnih produktov
// dva vektorja <i32> in vrne vektor, ki vsebuje vsote parov
// dva vektorja <i32> in vrne vsoto poparjenih pozitivni produktov s pomočjo ene izmed prejšnjih nalog
// vektor Option<T> in izpiše vse T-je
// vektor Option<T> in vrne število Some-ov
// odfiltrira števila deljena s 3

fn iter_stevila (v : Vec<i64>) {
    v.iter().map(|n| println!("{}", n));
}



// Popravite zaporedja iz prejšnjih vaj, da bodo `iterabilna`

// Dopolnite spodnjo funkcijo, da vrne niz, kjer so vse prve črke posameznih besed velike
// ["Just,", " ", "hello", " ", "world", "!"] -> "Just, Hello World", "!"
// pub fn capitalize_words_string(words: &[&str]) -> String {
//     panic!("Not implemented");
// }
// Napišite funkcijo `fakulteta`, ki izračuna fakulteto števila n. Uporabite iteratorje (torej brez lastne for zanke, rekurzije)
// Namig: fold, reduce, `..`...

// -------------------------------------------------------------------------------------------------
// Dodatno:
// Koda vzeta iz googlvih rust vaj:
// Vse se da lepo narediti samo z iteratorji (brez indeksov, brez for zank, brez mutabilnosti)
/*
/// Calculate the differences between elements of `values` offset by `offset`,
/// wrapping around from the end of `values` to the beginning.
///
/// Element `n` of the result is `values[(n+offset)%len] - values[n]`.
fn offset_differences<N>(offset: usize, values: Vec<N>) -> Vec<N>
where
    N: Copy + std::ops::Sub<Output = N>,
{
    unimplemented!()
}

#[test]
fn test_offset_one() {
    assert_eq!(offset_differences(1, vec![1, 3, 5, 7]), vec![2, 2, 2, -6]);
    assert_eq!(offset_differences(1, vec![1, 3, 5]), vec![2, 2, -4]);
    assert_eq!(offset_differences(1, vec![1, 3]), vec![2, -2]);
}

#[test]
fn test_larger_offsets() {
    assert_eq!(offset_differences(2, vec![1, 3, 5, 7]), vec![4, 4, -4, -4]);
    assert_eq!(offset_differences(3, vec![1, 3, 5, 7]), vec![6, -2, -2, -2]);
    assert_eq!(offset_differences(4, vec![1, 3, 5, 7]), vec![0, 0, 0, 0]);
    assert_eq!(offset_differences(5, vec![1, 3, 5, 7]), vec![2, 2, 2, -6]);
}

#[test]
fn test_custom_type() {
    assert_eq!(
        offset_differences(1, vec![1.0, 11.0, 5.0, 0.0]),
        vec![10.0, -6.0, -5.0, 1.0]
    );
}

#[test]
fn test_degenerate_cases() {
    assert_eq!(offset_differences(1, vec![0]), vec![0]);
    assert_eq!(offset_differences(1, vec![1]), vec![0]);
    let empty: Vec<i32> = vec![];
    assert_eq!(offset_differences(1, empty), vec![]);
}



*/

fn main(){
    let double = |x| 2 * x;
    println!("{}", apply_int(double, 5));
}
