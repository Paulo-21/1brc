use std::{collections::HashMap, time::Instant};
use memmap2::Mmap;
use ahash::AHashMap;

fn main() {
    let file_path = "../1brc/data/measurements1b.txt";
    //let file_path = "../1brc/measurements.txt";
    let start = Instant::now();
    let mmap: Mmap;
    let mut hash: AHashMap<&[u8], (f32, f32, f32, u16)> = AHashMap::new(); 

    let data;
    {
        let file = std::fs::File::open(file_path).unwrap();
        mmap = unsafe { Mmap::map(&file).unwrap() };
        data = &*mmap;
    }
    let mut i = 0;
    let mut startn = 0;
    let mut endn = 0;
    loop  {
        startn = i;
        if data.len() == i {
            break;
        }
        unsafe {
            loop {
                i += 1;
                if *data.get_unchecked(i) == b';' {
                    endn = i;
                    break;
                }
            }
            i+=1;
            /*let Some(separator) = memchr(b';', &data[startn..]) else {panic!("oups")};
            i+=separator;
            endn = i;
            i+=1;*/
            
            //let a = String::from_utf8_lossy(&data[startn..endn]);
            //print!("|{} : ", a);
            let mut neg = false;
            if *data.get_unchecked(i) == b'-' {
                neg = true;
                i+=1;
            }
            let mut temperature = *data.get_unchecked(i) & 0x0f;
            i+=1;
            if *data.get_unchecked(i) != b'.' {
                temperature *= 10;
                temperature += *data.get_unchecked(i) & 0x0f;
                i+=1;
            }
            i+=1;
            //print!("{}", *data.get_unchecked(i));
            let mut temperature = temperature as f32;
            temperature += ((*data.get_unchecked(i) & 0x0f) as f32) / 10.;
            i+=2;
            if neg { temperature *= -1.; }
            //println!("{}|", temperature);
            match hash.get_mut(&data[startn..endn]) {
                Some(v) => {
                    if v.0 > temperature {
                        v.0 = temperature;
                    }
                    else if v.2 < temperature {
                        v.2 = temperature;
                    }
                    v.1 += temperature;
                    v.3 += 1;
                },
                None => {
                    hash.insert(&data[startn..endn], (temperature, temperature, temperature, 1));
                }
            }
            //yeah
        }
    }
    let end = start.elapsed().as_millis();
    /*print!("{{");
    for h in hash {
        unsafe {
            let s = str::from_utf8_unchecked(h.0);
            print!("{};{};{};{},",s, h.1.0, (h.1.1/h.1.3 as f32), h.1.2);
        }
    }
    println!("}}");*/

    println!("Time taken : {}", end as f64 / 1000.);
}
