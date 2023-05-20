use::p2::es_primo;

trait numeroPrimo{}

impl numeroPrimo for Vec<i32>{
    fn es(&self) -> bool{
        let mut cont : u32 = 0;
        for i in &self{
            if es_primo(i){
                cont += 1;
            }
        }
    }
} 