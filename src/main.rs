use std::io;

fn main() {
    let _soma = 0;
    let _sub = 0;
    let _multi = 0;
    let _div = 0;

    println!("\nInsira um número: \n");

    let mut numero = String::new();

    io::stdin().read_line(&mut numero).expect("Msg");

    let _numero: isize = numero.trim().parse().expect("Erro // L216");
    let mut valor_total = _numero;
    loop {
        let mut _i: usize = 0;

        let mut valores: Vec<isize> = Vec::new();

        println!("\nInsira outro número: \n");

        let mut _numm = String::new();

        io::stdin().read_line(&mut _numm).expect("Msg");

        let _numm: isize = _numm.trim().parse().expect("Erro // L25");

        valores.push(_numero.try_into().unwrap());

        println!("\nValor total armazenado: {valor_total}");
        println!("\n//OPERAÇÃO\\\n");

        println!("1. Somar");
        println!("2. Subtrair");
        println!("3. Multiplicar");
        println!("4. Dividir");
        println!("5. Apagar");

        let mut operacao = String::new();

        io::stdin().read_line(&mut operacao).expect("msg");

        let operacao: i32 = operacao.trim().parse().expect("Erro //L32");

        if operacao == 1 {
            let vecstr: isize = _numm;

            let _soma: isize = valor_total;

            for _e in &valores {
                valor_total = vecstr + _soma;

                _i += 1;
            }

            println!("Soma De {vecstr} + {_soma}: {valor_total}");
        }

        if operacao == 2 {
            let vecstr = _numm;

            let _sub = valor_total;

            for _e in &valores {
                valor_total = vecstr - _sub;

                _i += 1;
            }

            println!("Subtração De {vecstr} - {_sub}: {valor_total}");
        }

        if operacao == 3 {
            let vecstr = _numm;

            let _multi = valor_total;

            for _e in &valores {
                valor_total = vecstr * _multi;

                _i += 1;
            }

            println!("Multiplicação De {vecstr} * {_multi}: {valor_total}");
        }

        if operacao == 4 {
            let vecstr = _numm;

            let _div = valor_total;

            for _e in &valores {
                if _div > vecstr {
                    valor_total = _div / vecstr;
                    println!("Divisão De {_div} / {vecstr}: {valor_total}");
                }
                if _div < vecstr {
                    valor_total = vecstr / _div;
                    println!("Divisão De {vecstr} / {_div}: {valor_total}");
                }
                if _div == 0 || vecstr == 0 {
                    break;
                }

                _i += 1;
            }
        }

        if operacao == 5 {
            let _soma = 0;
            let _sub = 0;
            let _multi = 0;
            let _div = 0;
            valores.clear();

            println!("\nInsira um número: \n");

            let mut numero = String::new();

            io::stdin().read_line(&mut numero).expect("Msg");

            let _numero: isize = numero.trim().parse().expect("Erro // L216");
            let valor_total = _numero;

            valores.push(valor_total.try_into().unwrap());
        }

        let operacao = "";
        let mut _operacao = operacao.to_string();
    }
}
