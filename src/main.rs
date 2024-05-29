use std::collections::HashSet;
use std::str::Chars;
use itertools::{Itertools, izip};
use rand::Rng;

fn main() {
    let key = "1234567";
    let mut rc4 = Rc4::init(key);


    let msg = "Litwo, ojczyzno moja";
    let cipher1 = rc4.translate(msg);
    println!("Cipher1: {}", cipher1);

    let msg = "Ile cie trzeba cenic ten tylko sie dowie";
    let cipher2 = rc4.translate(msg);
    println!("Cipher2: {}", cipher2);

    let msg = "Kto cie stracil Dzis pieknosc twa w calej ozdobie";
    let cipher3 = rc4.translate(msg);
    println!("Cipher3: {}", cipher3);


    let mut rc42 = Rc4::init("Kielbasa slaska");
    let msg = "Widze i opisuje, bo tęsknie po tobie";
    let cipher4 = rc42.translate(msg);
    println!("Cipher4: {}", cipher4);

    let mut rc4 = Rc4::init(key);
    let msg = "qwerty";
    let cipher1 = rc4.translate(msg);
    println!("Cipher1: {}", cipher1);


    let mut rc4 = Rc4::init(key);
    let msg = rc4.translate(&cipher1);
    println!("Message1: {}", msg);
    let msg = rc4.translate(&cipher2);
    println!("Message2: {}", msg);
    let msg = rc4.translate(&cipher3);
    println!("Message3: {}", msg);
    let msg = rc42.translate(&cipher4);
    println!("Message4: {}", msg);
}

#[derive(Clone)]
pub struct Rc4 {
    s: [usize; 256],
    i: usize,
    j: usize,
}

impl Rc4 {
    fn init(key: &str) -> Rc4 {
        let mut s = [0; 256];

        // s.iter_mut().zip(0..=255u8).for_each(|(mut x, y)| *x = y);
        for i in 0..=255 {
            s[i] = i;
        }

        let mut chars = key.chars().cycle();

        let mut j = 0;
        for i in 0..=255 {
            j += s[i] + chars.next().unwrap() as usize;
            j %= 256;
            s.swap(i, j)
        }
        Rc4 { s, i: 0, j: 0 }
    }


    fn next_cipher(&mut self) -> usize {
        let Rc4 { s, i, j } = self;
        *j += 1;
        *j %= 256;

        *i += s[*j];
        *i %= 256;

        s.swap(*i, *j);

        let t = (s[*i] + s[*j]) % 256;
        s[t]
    }

    fn translate(&mut self, msg: &str) -> String {
        msg.chars().zip(self).map(|(c, k)| {
            // println!("c: {}", c as u8);
            // println!("k: {}", k);
            let ch = c as u8 ^ k as u8;
            // println!("ch:{:?}", ch);
            ch as char
        }).collect()
    }
}

impl Iterator for Rc4 {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.next_cipher())
    }
}

fn as_hex(string: &str) -> String { //:02X
    string.chars().map(|c| format!("{:02X} ", c as u8)).collect()
}

fn as_char(string: &str) -> String {
    string.split_whitespace().map(|c| {
        let c = u8::from_str_radix(c, 16).unwrap();
        c as char
    }).collect()
}

fn as_num(string: &str) -> Vec<u8> {
    string.split_whitespace().map(|c| {
        let d = u8::from_str_radix(c, 16).unwrap_or_else(|_| {
            println!("Error: {}", c);
            0
        });
        d
    }).collect()
}

fn xor(cip1: &str, cip2: &str) -> String {
    cip1.chars().zip(cip2.chars()).map(|(c1, c2)| (c1 as u8 ^ c2 as u8) as char).collect()
}

fn check(cip1: &str, cip2: &str) -> bool {
    let xor = xor(&cip1, &cip2);
    let xor = as_hex(&xor);
    let num = as_num(&xor);
    let stat = num.iter().fold(0usize, |acc, x| {
        if x < &64 {
            acc + 1
        } else {
            acc
        }
    });
    let val = stat as f64 / num.len() as f64;
    println!("Stat: {}", val);
    val > 0.5
}

fn look_for_xor(look: u8) -> HashSet<char> {
    let chars = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let mut res = HashSet::new();
    for i in 0..10 {
        for j in (i)..10 {
            let xor = chars[i] as u8 ^ chars[j] as u8;
            if xor == look {
                // println!("numbs {} {}", chars[i] as u8, chars[j] as u8);
                // println!("chars {} {}", chars[i], chars[j]);
                res.insert(chars[i]);
                res.insert(chars[j]);
            }
        }
    }
    if res.is_empty() {
        println!("No options for {}", look);
    }
    res
}

fn gen_bank_numbers(q: usize) -> Vec<String> {
    let mut bank_numbers: Vec<String> = Vec::new();
    let numery_rozliczeniowe: [[u8; 8]; 5] = [
        [1, 0, 1, 0, 0, 0, 0, 0], // NBP
        [1, 1, 6, 0, 0, 0, 0, 6], // Millenium
        [1, 0, 5, 0, 0, 0, 0, 2], // ING
        [2, 1, 2, 0, 0, 0, 0, 1], // Santander
        [1, 0, 2, 0, 0, 0, 0, 3], // PKO BP
    ];
    let mut rng = rand::thread_rng();
    for nr in numery_rozliczeniowe {
        for _ in 0..q {
            let mut bank_number = String::new();
            let mut client_number = [0u8; 16];
            for i in 0..16 {
                client_number[i] = rng.gen_range(0..10);
            }
            let mut tmp: u128 = 212500;
            for i in 0..8 {
                tmp += nr[i] as u128 * 10u128.pow((7 - i + 21) as u32);
            }
            for i in 0..16 {
                tmp += client_number[i] as u128 * 10u128.pow((15 - i + 5) as u32);
            }
            tmp = tmp % 97;
            tmp = 98 - tmp;
            bank_number.push_str(format!("{:02}", tmp).as_str());
            for i in 0..8 {
                bank_number.push_str(nr[i].to_string().as_str());
            }
            for i in 0..16 {
                bank_number.push_str(client_number[i].to_string().as_str());
            }
            bank_numbers.push(bank_number);
        }
    }
    bank_numbers
}

pub fn calculte_nr_control_number(nr: [u8; 7]) -> u8 {
    let weights = [3, 9, 7, 1, 3, 9, 7];
    let mut sum = 0;
    for i in 0..7 {
        sum += nr[i] as u16 * weights[i] as u16;
    }
    (10 - (sum % 10) as u8) % 10
}

fn find_char_from_xor(chars: &Vec<char>) -> HashSet<char> {
    let mut res = HashSet::from(['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']);
    let mut iter = chars.iter();
    let first = iter.next().unwrap();
    for c in iter {
        let xor = *first as u8 ^ *c as u8;
        let options = look_for_xor(xor);
        // println!("opts {:?}", options);
        res = res.intersection(&options).map(|x| *x).collect();
    }
    res
}

fn process_ciphers(ciphers: Vec<String>) {
    let mut iter = ciphers.iter().map(|x| x.chars()).collect::<Vec<_>>();
    while let Some(encrypted) = get_slice(&mut iter) {
        let mut options = find_char_from_xor(&encrypted);
        println!("{:?}", options);

        // let mut xors = vec![0];
        // let mut iter = encrypted.iter();
        // let first = iter.next().unwrap();
        // for c in iter {
        //     let xor = *first as u8 ^ *c as u8;
        //     xors.push(xor);
        // }
        //
        // for i in 1..encrypted.len() {
        //     'opts: for first in options.clone() {
        //         let decrypted = xors.iter().map(|x| x ^ first as u8).collect::<Vec<u8>>();
        //         // println!("{:?}", decrypted);
        //         let check = decrypted[i];
        //         for k in (i + 1)..encrypted.len() {
        //             let first = check ^ decrypted[k];
        //             let second = encrypted[i] as u8 ^ encrypted[k] as u8;
        //             if first != second {
        //                 options.remove(&(first as char));
        //                 println!("Removed: {}", first as char);
        //                 continue 'opts;
        //             }
        //         }
        //     }
        //     if options.len() == 1 {
        //         break;
        //     }
        // }
    }
}

fn get_slice(iters: &mut Vec<Chars>) -> Option<Vec<char>> {
    let mut res = Vec::new();
    for iter in iters.iter_mut() {
        res.push(iter.next()?);
    }
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rc4() {
        let key = "Pierogi ruskie";
        let key2 = "Kielbasa slaska";


        let mut rc4 = Rc4::init(key);
        let msg = "Contrary to popular belief, Lorem Ipsum is not simply random text. It has roots in a piece of classical Latin literature from 45 BC, making it over 2000 years old. Richard McClintock, a Latin professor at Hampden-Sydney College in Virginia, looked up one of the more obscure Latin words, consectetur, from a Lorem Ipsum passage, and going through the cites of the word in classical literature, discovered the undoubtable source. Lorem Ipsum comes from sections 1.10.32 and 1.10.33 of \"de Finibus Bonorum et Malorum\" (The Extremes of Good and Evil) by Cicero, written in 45 BC. This book is a treatise on the theory of ethics, very popular during the Renaissance. The first line of Lorem Ipsum, \"Lorem ipsum dolor sit amet..\", comes from a line in section 1.10.32.";
        let cipher1 = rc4.translate(msg);
        // let cipher1 = as_hex(&cipher1);
        println!("Cipher1: {}", cipher1);

        let mut rc4 = Rc4::init(key2);
        let msg = "It is a long established fact that a reader will be distracted by the readable content of a page when looking at its layout. The point of using Lorem Ipsum is that it has a more-or-less normal distribution of letters, as opposed to using 'Content here, content here', making it look like readable English. Many desktop publishing packages and web page editors now use Lorem Ipsum as their default model text, and a search for 'lorem ipsum' will uncover many web sites still in their infancy. Various versions have evolved over the years, sometimes by accident, sometimes on purpose (injected humour and the like).";
        let cipher2 = rc4.translate(msg);
        // let cipher2 = as_hex(&cipher2);
        println!("Cipher2: {}", cipher2);

        let mut rc4 = Rc4::init(key);
        let msg = "There are many variations of passages of Lorem Ipsum available, but the majority have suffered alteration in some form, by injected humour, or randomised words which don't look even slightly believable. If you are going to use a passage of Lorem Ipsum, you need to be sure there isn't anything embarrassing hidden in the middle of text. All the Lorem Ipsum generators on the Internet tend to repeat predefined chunks as necessary, making this the first true generator on the Internet. It uses a dictionary of over 200 Latin words, combined with a handful of model sentence structures, to generate Lorem Ipsum which looks reasonable. The generated Lorem Ipsum is therefore always free from repetition, injected humour, or non-characteristic words etc.";
        let cipher3 = rc4.translate(msg);
        // let cipher3 = as_hex(&cipher3);
        println!("Cipher3: {}", cipher3);

        let mut rc4 = Rc4::init(key);
        // let cipher1 = as_char(&cipher1);
        let msg = rc4.translate(&cipher1);
        println!("Message1: {}", msg);

        let mut rc4 = Rc4::init(key2);
        // let cipher2 = as_char(&cipher2);
        let msg = rc4.translate(&cipher2);
        println!("Message2: {}", msg);

        let mut rc4 = Rc4::init(key);
        // let cipher3 = as_char(&cipher3);
        let msg = rc4.translate(&cipher3);
        println!("Message3: {}", msg);
    }

    #[test]
    fn test_crack() {
        let key = "Pierogi ruskie";
        let key2 = "Kielbasa slaska";


        let mut rc4 = Rc4::init(key);
        let msg = "Contrary to popular belief, Lorem Ipsum is not simply random text. It has roots in a piece of classical Latin literature from 45 BC, making it over 2000 years old. Richard McClintock, a Latin professor at Hampden-Sydney College in Virginia, looked up one of the more obscure Latin words, consectetur, from a Lorem Ipsum passage, and going through the cites of the word in classical literature, discovered the undoubtable source. Lorem Ipsum comes from sections 1.10.32 and 1.10.33 of \"de Finibus Bonorum et Malorum\" (The Extremes of Good and Evil) by Cicero, written in 45 BC. This book is a treatise on the theory of ethics, very popular during the Renaissance. The first line of Lorem Ipsum, \"Lorem ipsum dolor sit amet..\", comes from a line in section 1.10.32.";
        let cipher1 = rc4.translate(msg);
        // let cipher1 = as_hex(&cipher1);
        println!("Cipher1: {}", cipher1);

        let mut rc4 = Rc4::init(key2);
        let msg = "It is a long established fact that a reader will be distracted by the readable content of a page when looking at its layout. The point of using Lorem Ipsum is that it has a more-or-less normal distribution of letters, as opposed to using 'Content here, content here', making it look like readable English. Many desktop publishing packages and web page editors now use Lorem Ipsum as their default model text, and a search for 'lorem ipsum' will uncover many web sites still in their infancy. Various versions have evolved over the years, sometimes by accident, sometimes on purpose (injected humour and the like).";
        let cipher2 = rc4.translate(msg);
        // let cipher2 = as_hex(&cipher2);
        println!("Cipher2: {}", cipher2);

        let mut rc4 = Rc4::init(key);
        let msg = "There are many variations of passages of Lorem Ipsum available, but the majority have suffered alteration in some form, by injected humour, or randomised words which don't look even slightly believable. If you are going to use a passage of Lorem Ipsum, you need to be sure there isn't anything embarrassing hidden in the middle of text. All the Lorem Ipsum generators on the Internet tend to repeat predefined chunks as necessary, making this the first true generator on the Internet. It uses a dictionary of over 200 Latin words, combined with a handful of model sentence structures, to generate Lorem Ipsum which looks reasonable. The generated Lorem Ipsum is therefore always free from repetition, injected humour, or non-characteristic words etc.";
        let cipher3 = rc4.translate(msg);
        // let cipher3 = as_hex(&cipher3);
        println!("Cipher3: {}", cipher3);

        let mut rc4 = Rc4::init(key2);
        let msg = "Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen book. It has survived not only five centuries, but also the leap into electronic typesetting, remaining essentially unchanged. It was popularised in the 1960s with the release of Letraset sheets containing Lorem Ipsum passages, and more recently with desktop publishing software like Aldus PageMaker including versions of Lorem Ipsum.";
        let cipher4 = rc4.translate(msg);
        // let cipher4 = as_hex(&cipher4);
        println!("Cipher4: {}", cipher4);

        check(&cipher1, &cipher2);
        check(&cipher1, &cipher3);
        check(&cipher1, &cipher4);
        check(&cipher2, &cipher3);
        check(&cipher2, &cipher4);
        check(&cipher3, &cipher4);
    }

    #[test]
    fn test_3() {
        let key = "Pois nem tudo que cai do ceu e sagrado";
        let mut rc4 = Rc4::init(key);

        let num1 = "86 1050 1504 1000 0090 7583 4177";
        let num2 = "26 1030 1508 0000 0005 5000 1144";
        let num3 = "13 1010 1010 0030 1413 1210 0000";
        let num4 = "21 1010 0068 6800 0000 0000 0012";
        let num5 = "20 1020 1462 0000 7402 0313 0275";

        let mut cph = vec![String::new(); 5];
        cph[0] = rc4.clone().translate(num1);
        cph[1] = rc4.clone().translate(num2);
        cph[2] = rc4.clone().translate(num3);
        cph[3] = rc4.clone().translate(num4);
        cph[4] = rc4.clone().translate(num5);

        let mut xors = vec![vec![String::new(); 5]; 5];
        for i in 0..5 {
            for j in 0..5 {
                if j != i {
                    let xor = xor(&cph[i], &cph[j]);
                    println!("{} {} : {}", i, j, as_hex(&xor));
                    xors[i][j] = xor;
                }
            }
            println!();
        }
    }

    #[test]
    fn test4() {
        let key = "1234567";
        let mut rc4 = Rc4::init(key);

        let nums = gen_bank_numbers(20);
        println!("{:?}", nums[0]);
        let ciphs = nums.iter().map(|x| rc4.clone().translate(x)).collect::<Vec<String>>();
        // for i in ciphs.iter() {
        //     println!("{}", as_hex(&i));
        // }
        process_ciphers(ciphs);
    }
}