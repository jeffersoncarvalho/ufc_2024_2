use std::io;

fn main() {
    //QUESTÃO 01
    /*let numeros = [2, -3, 7, 0, 8, -1, 5, -4, 6, 10];

    match media_positivos(numeros) {
        Some(media) => println!("Média dos números positivos: {:.2}", media),
        None => println!("Não há números positivos."),
    }

    let produto = produto_pares(numeros);
    println!("Produto dos números pares (excluindo o zero): {}", produto);*/

    //QUESTÃO 02
    //lendo e criando a tupla
    let a = read_integer();
    let b = read_integer();
    let c = read_integer();
    
    /*let res_tupla = analisar_tupla((a,b,c));
    println!("A soma é {}", res_tupla.0);
    println!("O maior é {}", res_tupla.1);
    println!("O menor é {}", res_tupla.2);*/

    let (soma,maior,menor) = analisar_tupla((a,b,c));
    println!("A soma é {}", soma);
    println!("O maior é {}", maior);
    println!("O menor é {}", menor);
     
}

fn media_positivos(array:[i32;10]) -> Option<f32> {
    
    let mut sum = 0;
    let mut count = 0;

    /*for &number in array.iter() {
        if number > 0 {
            sum = sum + number;
            count += 1;
        }
    }*/

    for number in array.iter() {
        if *number > 0 {
            sum = sum + *number;
            count += 1;
        }
    }

    if count == 0 {
        None
        //return None;
    } else {
        let avg : f32 = (sum as f32)/(count as f32);
        Some(avg)
        //return Some(avg);
    }
}

fn produto_pares(array:[i32;10]) -> i32 {
    let mut product : i32 = 1;
    for &number in array.iter() {
        if number!=0 && number % 2 == 0 {
            product *= number;
        }
    }
    product
    //return product;
}

fn read_integer() -> i32 {
    //keyboard input
    println!("Please, input your number: ");
    let mut number = String::new();
    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");

    //parse to integer (shadowing)
    let number: i32 =  match number
                            .trim()
                            .parse() //Enumeration (Result)
                            {
                                Ok(num) => num,
                                Err(_) => { 
                                    println!("Enter a valid number!");
                                    return -1; 
                                }
                            };
    return number;

}

fn analisar_tupla(tupla : (i32,i32,i32)) -> (i32,i32,i32) {
    
    let soma = tupla.0 + tupla.1 + tupla.2;
    let mut maior = tupla.0;
    let mut menor = std::cmp::min( tupla.0, std::cmp::min(tupla.1,tupla.2));
    
    if tupla.0 > tupla.1 && tupla.0 > tupla.2 {
        maior = tupla.0;
    }else if tupla.1 > tupla.0 && tupla.1 > tupla.2 {
        maior = tupla.1;
    }else {
        maior = tupla.2;
    }

    return (soma, maior, menor);
}
