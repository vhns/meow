use sha2::{Sha256, Digest};
use std::fs::File;
use std::io;
use std::io::{Write, Read};
use std::path::Path;

/// Estrutura para representar um usuário
struct Usuario {
    nome: String,
    senha: String
}

/// Estrutura para representar o cadastro.
/// O parâmetro 'a representa um tempo de vida diferente do que havíamos discutido em aula.
struct Cadastro<'a> {
    usuarios: Vec<Usuario>,
    arquivo_usuarios: &'a Path,
}

/// Função para converter uma String em uma String contendo a representação hexadecimal do SHA256 da variável de entrada
fn sha256(texto: String) -> String {
    // Obtendo o hash da variável 'texto' em formato de bytes
    let dados = Sha256::digest(texto.as_bytes());

    // Funções anônimas
    let to_hex = |x: &u8| {format!("{:02X}", x)}; 
    let join   = |acc: String, i: String| {acc + &i};
    
    // Convertendo os dados de bytes para uma String da representação hexadecimal de cada byte
    // .to_vec()                  -> Transforma em vetor (Vec<T>)
    // .iter()                    -> Transforma o vetor em um iterador, ou seja, um tipo que irá permitir 
    //                               utilizarmos as funções abaixo
    // .map(to_hex)               -> Irá iterar por todos os elementos do array de bytes retornado na linha 19
    //                               aplicando a função 'to_hex' para cada elemento
    // .fold(String::new(), join) -> Irá concatenar o resultado à "String::new()" através da função 'join'. 
    // Finalmente, o tipo do retorno de todas essas chamadas será uma String, pois todo o resultado é concatenado
    // a uma string vazia (String::new()).
    dados.to_vec().iter().map(to_hex).fold(String::new(), join)
}

/// Função para envelopar as leituras da entrada do usuário. Além disso, ela mostra a informação ```mensagem: &str``` na mesma linha e retorna o resultado (```String```).
fn input(mensagem: &str) -> String {
    print!("{}", mensagem);
    io::stdout().flush().unwrap();

    let mut ret: String = String::new();
    io::stdin().read_line(&mut ret).unwrap();

    return ret.trim().to_string();
}

// Implementações para a struct 'Usuario'
impl Usuario {
    pub fn new(nome: String, senha: String) -> Usuario {
	
	Usuario{nome, senha:sha256(senha)}

        // Não se esqueça de armazenar a senha do usuário como sha256
    }

    fn mostrar(&self) {
        println!("<Usuário nome={:5} | senha={}>", self.nome, self.senha);
    }
}

impl <'a> Cadastro<'a> {
    pub fn new(arquivo_usuarios: &Path) -> Cadastro {
        Cadastro{ usuarios: vec![], arquivo_usuarios }
    }

    pub fn adiciona_usuario(&mut self, nome: String, senha: String) {
	&self.usuarios.push(Usuario{nome, senha});
    }

    fn usuario_existe(&self, nome: &String) -> bool {
        for usuario in &self.usuarios {
            if usuario.nome.eq(nome) { return true }
        }
        return false
    }

    fn salvar(&self) -> std::io::Result<()> {
        todo!("Implementar o salvamento de arquivos!");
    }

    fn autenticar(&self, nome: String, senha: String) -> bool {
        todo!("Implementar a autenticação de usuários!");
    }

    fn retorna_indice_usuario(&self, nome: &String) -> isize {
        if !self.usuario_existe(&nome) {
            return -1
        }

        for (i, user) in (&self.usuarios).into_iter().enumerate() {
            if user.nome.eq(nome) {
                return i as isize
            }
        }
        -1
    }

    fn carregar(&mut self) -> std::io::Result<()>{
       todo!("Implementar o carregamento de arquivos!");

       Ok(()) // Não apague esta linha
    }
}

fn main(){
    todo!("Implemente o sistema a partir daqui");
}
