use rand::Rng;

const FORMAT_0:&str = "1234567890";
const FORMAT_A:&str = "abcdefghijklmnopqrstuvwxyz";
const FORMAT_AA:&str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const FORMAT_S:&str = "-_";

#[derive(Debug)]
struct GenPass {
    // Длина нового пароля
    length:u8,
    // Из каких символов
    // 0 - Только цифры
    // 0a - цифры и буквы в нижнем регистре
    // 0aA - цифры и буквы в нижнем|верхнем регистре
    // 0aAs - цифры, буквы в нижнем|верхнем регистре и -_ символы
    format:String
}
impl GenPass {
    pub fn new()-> GenPass {
        GenPass {
            // Длина нового пароля
            length:8,
            // Из каких символов
            // 0 - Только цифры
            // 0a - цифры и буквы в нижнем регистре
            // 0aA - цифры и буквы в нижнем|верхнем регистре
            // 0aAs - цифры, буквы в нижнем|верхнем регистре и -_ символы
            format:"0aAs".to_string()
        }
    }
    /// Установить длину пароля по параметру аргумента
    pub fn set_len_arg(&mut self, arg:&str){
        self.length = if arg.contains(":") {
            let dep = arg
                .split(":")
                .map(|num|{ num.parse::<u8>().unwrap_or(8) })
                .collect::<Vec<u8>>();
            match dep.len() {
                0 => 8,
                1 => dep.get(0).unwrap_or(&8).clone(),
                _ => {
                    use rand::Rng;
                    rand::thread_rng().gen_range(
                        dep.get(0).unwrap_or(&8),
                        dep.get(1).unwrap_or(&8)
                    )
                }
            }
        }else{
            arg.parse::<u8>().unwrap_or(8)
        };
    }
    /// Установить формат пароля
    /// Из каких символов
    ///  0 - Только цифры
    ///  0a - цифры и буквы в нижнем регистре
    ///  0aA - цифры и буквы в нижнем|верхнем регистре
    ///  0aAs - цифры, буквы в нижнем|верхнем регистре и -_ символы
    pub fn set_format_arg(&mut self, arg:&str){
        self.format =  match arg {
                "0"|"0a"|"0aA"|"0aAs"=>{ arg },
                _=>{"0aAs"}
            }.to_string();
    }
    pub fn gen(&self)->String{
        let format = self.format.chars().map(|ch|{
            match ch {
                '0' => FORMAT_0,
                'a' => FORMAT_A,
                'A' => FORMAT_AA,
                's' => FORMAT_S,
                _=>{""}
            }.to_string()
        })
        .collect::<Vec<String>>()
        .join("")
        .chars()
        .collect::<Vec<char>>();
        let endindex = format.len() - 1;

        (0..=self.length)
            .map(|_|{ format[rand::thread_rng().gen_range(0,endindex)] })
            .collect::<String>()
    }
}

fn main() {
    // Настройки генирации пароля
    let mut pass = GenPass::new();
    // Аргументы из командной строки
    std::env::args().skip(1).for_each(|arg|{
            // Обработка аргументов
            match &arg {
                // Длина нового пароля
                a if a.contains("len=") => { a.split("=").skip(1).last().map(|pam|{ pass.set_len_arg(pam); }); },
                // Формат пароля
                a if a.contains("chars=") => { a.split("=").skip(1).last().map(|pam|{ pass.set_format_arg(pam); }); },
                // Неизвестный аргумент
                _=>{}
            };
        });
    println!("{}", pass.gen());
}
