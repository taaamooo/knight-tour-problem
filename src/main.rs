use std::{fs, io::Write};

fn main() {
    let size = 8;
    let sqr = size * size;

    let mut file = fs::File::create("ktp.cnf").unwrap();

    // let header_str = String::from("p cnf ") + size.to_string().as_str() + "\n";
    // let _ = file.write_all(header_str.as_bytes());

    for _t in 0..sqr {
        let mut clause:Vec<i32> = Vec::new();
        for _j in 0..size {
            for _k in 0..size {
                clause.push(_t*sqr + _j*size + _k + 1);
            }
        }
        let strs:Vec<String> = clause.iter().map(|x| x.to_string()).collect();
        let buf = strs.join(" ") + " 0 \n";
        let _ = file.write_all(buf.as_bytes());
    }
    for _i in 0..sqr {
        for _j in 0..size {
            for _k in 0..size {
                let origin = -(_i*sqr + _j*size + _k + 1);
                for _l in _j..size {
                    for _m in _k..size-1 {
                        let mut clause:Vec<i32> = Vec::new();
                        clause.push(origin);
                        clause.push(-(_i*sqr + _l*size + _m + 2));
                        let strs:Vec<String> = clause.iter().map(|x| x.to_string()).collect();
                        let buf = strs.join(" ") + " 0 \n";
                        let _ = file.write_all(buf.as_bytes());
                    }
                }
            }
        }
    }

    for _i in 0..sqr-1 {
        let mov = [(2, 1), (2, -1), (-2, 1), (-2, -1), (1, 2), (1, -2), (-1, 2), (-1, -2)];
        for _j in 0..size {
            for _k in 0..size {
                let mut clause:Vec<i32> = [-(_i*sqr + _j*size + _k + 1)].to_vec();
                for _m in mov.iter() {
                    let _a = _m.0 + _j;
                    let _b = _m.1 + _k;
                    if (_a < 0 || _a >= size) || (_b < 0 || _b >= size) {
                        continue;
                    }
                    clause.push((_i+1)*sqr + _a*size + _b + 1);
                }
                let strs:Vec<String> = clause.iter().map(|x| x.to_string()).collect();
                let buf = strs.join(" ") + " 0 \n";
                let _ = file.write_all(buf.as_bytes());
            }
        }
    }

    for _i in 0..size {
        for _j in 0..size {
            for _a in 0..sqr-1 {
                for _b in _a+1..sqr {
                    let mut clause:Vec<i32> = Vec::new();
                    clause.push(-(_a*sqr + _i*size + _j + 1));
                    clause.push(-(_b*sqr + _i*size + _j + 1));
                    let strs:Vec<String> = clause.iter().map(|x| x.to_string()).collect();
                    let buf = strs.join(" ") + " 0 \n";
                    let _ = file.write_all(buf.as_bytes());
                }
            }
        }
    }
}