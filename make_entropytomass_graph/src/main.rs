// use gnuplot::{Figure, Caption, Color};
use std::io::{BufRead, BufReader, Write};
use std::{fs::File, str::FromStr};

fn load_data(filename: &str) -> std::io::Result<Vec<Vec<f64>>> {
    let mut data = Vec::new();
    let mut l = 0;

    for result in BufReader::new(File::open(filename)?).lines() {
        let line = Vec::new();
        data.push(line);
        for s in result?.split('\t') {
            // println!("{}", s);
            let l_s = match f64:: from_str(s) {
                Ok(value) => value,
                Err(_err) => 0.0,
            };
            data[l].push(l_s);
        }
        l += 1;
    }
    Ok(data)
}


fn make_picture (x: Vec<f64>, y: Vec<f64>, dir: String) {
    
    let dir = format!("../../alpha_vis_{}_Detail.txt",dir);
    let mut buf = File::create(dir).unwrap();
    for i in 0..(x.len() - 1) {
        let x1: f64 = x[i];
        let y1: f64 = y[i];
        let x2: f64 = x[i + 1];
        let y2: f64 = y[i + 1];

        let logx1: f64 = x1.log10();
        let logy1: f64 = y1.log10();
        let logx2: f64 = x2.log10();
        let logy2: f64 = y2.log10();
        let a = (logy1- logy2) / (logx1 - logx2);
        let b = logy1 - a * logx1;

        let mut logx  = logx1;
        let dlogx = 0.0001;
        while logx + dlogx <= logx2 {
            let logy = a * logx + b;
            let x = (10.0_f64).powf(logx);
            let y = (10.0_f64).powf(logy);
            write!(buf, "{:e}\t\t",x);
            writeln!(buf,"{:e}", y);
            logx += dlogx;
                println!("x = {:e}, y = {:e}", x,y);
        }
        // (x[i], y[i]), (x[i + 1], y[i + 1])を使って(対数グラフにおける)直線を定義。
        // 直線を描画する。
        
        // let a = (y1.log10() - y2.log10()) / (x1.log10() - x2.log10());
        // let b = y1.log10() - a * x1.log10();
        // // println!("a = {:.2e}, b = {:.2e}", a, b);
        // let mut x = x1;
        // let dx = 0.1;
        // let dx = 
        // while x + dx <= x2 {
        //     // x, yをテキスト出力。
        //     let y = (10.0f64).powf(a * x.log10() + b);
        //     write!(buf, "{:e}\t\t",x);
        //     writeln!(buf,"{:e}", y);
        //     x += dx;
        //     // println!("x = {}, y = {}", x,y);
        // }
    }
}



fn main() {
    // alpha_vis_004.txt or alpha_vis_002.txtを読み込み,配列x[], y[]に代入。
    let s_002               = vec![8.,     9.,   12.,  15.,  18.,    23.,  30.,    40.,  50.,  60.,  70.,  90.,  120., 150.,   200.];
    let Mass_per_bin_002    = vec![1e-10, 2e-4, 7e-4, 2e-3, 4.5e-3, 2e-3, 1.7e-3, 1.01e-3, 1e-3, 7e-4, 4e-4, 2e-4, 7e-5, 1.2e-5, 1e-10];

    let s_004               = vec![5.,    7.,   9.,     11.,  13.,  19.,  23.,    30.,  40.,  50.,  60.,    80.,  90.,   120.,   150., 180.,  220.,   400.];
    let Mass_per_bin_004    = vec![1e-10, 5e-3, 1.5e-2, 8e-3, 4e-3, 3e-3, 3.5e-3, 2e-3, 2e-3, 2e-3, 1.2e-3, 8e-4, 4e-4,  1.5e-4, 5e-5, 2e-5,  3.5e-6, 1e-10];

    // let input_data = format!("../../alpha_vis_002.txt");

    // let mut x = Vec::<f64>::new();
    // let mut y = Vec::<f64>::new();
    let mut x1 = s_002.clone();
    let mut y1 = Mass_per_bin_002.clone();
    let mut x2 = s_004.clone();
    let mut y2 = Mass_per_bin_004.clone();
    
    // if let Ok(data) = load_data(&input_data) {
        //     let data = load_data(&input_data).unwrap();
        //     for data in data {
            //         // println!("{} {}", data[0], data[1]);
            //         x.push(data[0]);
            //         y.push(data[1]);
            //         println!("{:e} {:e}", data[0], data[1]);
            //     }
            // }
            
    make_picture(x1, y1, "002".to_string());
    make_picture(x2, y2, "004".to_string());

}
