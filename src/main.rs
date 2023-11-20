// João Henrique / Vitor Hugo / Marco aurélio
extern crate sha2;
use sha2::{Sha256, Digest};
use std::fmt;
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
impl fmt::Debug for Usuario{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        f.debug_struct("Usuario")
            .field("nome", &self.nome)
            .field("senha", &self.senha)
            .finish()
    }
}
impl Usuario {
    pub fn new(nome: &String, senha: &String) -> Usuario {

	Usuario{nome: nome.to_string(), senha: sha256(senha.to_string())}

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

    pub fn adiciona_usuario(&mut self, user: Usuario) {
	    self.usuarios.push(user);
    }

    fn usuario_existe(&self, nome: &String) -> bool {
        for usuario in &self.usuarios {
            if usuario.nome.eq(nome) { return true }
        }
        return false
    }

    fn salvar(&self, i:&isize) -> std::io::Result<()> {
        let mut f = match File::options().append(true).open(&self.arquivo_usuarios){
            Err(e) => panic!("Erro ao abrir o arquivo pelo motivo {}", e),
            Ok(f) => {f}
        };

        let conteudo: &Usuario = &self.usuarios[*i as usize];

        match writeln!(&mut f, "{:?}",conteudo){
            Err(e) => panic!("Erro ao escrever no arquivo pelo motivo {}", e),
            Ok(()) => {}
        }
            
        match f.flush(){
            Err(e) => panic!("Erro ao salvar o arquivo pelo motivo {}", e),
            Ok(()) => {}
        }

        Ok(())

    }

    fn autenticar(&mut self, nome: &String, senha: &String) -> bool {
        if self.usuario_existe(&nome) {
            if self.arquivo_usuarios.exists() {
                let _ = self.carregar();

                let mut arquivo: File = match File::open(self.arquivo_usuarios) {
                    Err(e) => panic!("Erro ao abrir o arquivo pelo motivo {}", e),
                    Ok(arquivo) => {arquivo}
                };
    
                let mut conteudo: String = String::new();
    
                match arquivo.read_to_string(&mut conteudo) {
                    Err(e) => panic!("Erro ao abrir o arquivo pelo motivo {}", e),
                    Ok(_) => println!("")
                };
    
                let user: String = format!(r#"nome: "{}", senha: "{}""#, nome, senha);
    
                if conteudo.contains(&user) {
                    println!("Usuário autenticado!");
                    return true;
                } else {
                    println!("Falha ao autenticar o usuário! Verifique suas credenciais.");
                    return false;
                }
            }
        }
        println!("O usuário não existe.");
        return false;
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
       let mut arquivo: File = match File::open(self.arquivo_usuarios) {
            Err(e) => panic!("Erro ao abrir o arquivo pelo motivo {}", e),
            Ok(arquivo) => {arquivo}
        };

        let mut conteudo: String = String::new();

        match arquivo.read_to_string(&mut conteudo) {
            Err(e) => panic!("Erro ao abrir o arquivo pelo motivo {}", e),
            Ok(_) => println!("")
        };

        for line in conteudo.lines() {
            let line_str = line.to_string();
            let split_line: Vec<&str> = line_str.split('"').collect();

            if split_line.len() > 1 {
                let nome: String = String::from(split_line[1]);
                let senha: String = String::from(split_line[3]);

                let user: Usuario = Usuario::new(&nome, &senha);

                self.adiciona_usuario(user);
            }
        }

       Ok(()) // Não apague esta linha
    }
}

fn main() {
        let path = Path::new("foo.txt");
        if !path.exists() {
            let mut _f = match File::create(path){
                Err(e) => panic!("Erro ao ler o arquivo pelo motivo {}", e),
                Ok(f) => {f}
            };
        }
        
        let mut cadastro: Cadastro = Cadastro::new(&path);
        let _ = cadastro.carregar();
        println!("{:?}", &cadastro.usuarios);
        loop {
            let mut menu: String = String::new();

            println!("Digite a ação que você deseja:\n 1 - Autenticar usuário.\n 2 - Cadastrar usuário.\n 3 - Sair.\n");
            io::stdin().read_line(&mut menu).expect("Erro ao ler a entrada");

            let menu_s: String = menu.trim().parse().expect("Erro ao converter a opção selecionada");

            if menu_s == "1".to_string() {
                let mut nome: String = String::new();
                let mut senha: String = String::new();

                println!("Digite o seu username: ");
                io::stdin().read_line(&mut nome).expect("Erro ao ler o username");
                println!("Digite a sua senha: ");
                io::stdin().read_line(&mut senha).expect("Erro ao ler a senha");

                let nome_s: String = nome.trim().parse().expect("Erro ao transformar o username");
                let senha_s:String = senha.trim().parse().expect("Erro ao transformar a senha");

                cadastro.autenticar(&nome_s, &sha256(senha_s));
            } else if menu_s == "2".to_string() {
                let mut nome: String = String::new();
                let mut senha: String = String::new();

                println!("Digite o seu username: ");
                io::stdin().read_line(&mut nome).expect("Erro ao ler o username");
                println!("Digite a sua senha: ");
                io::stdin().read_line(&mut senha).expect("Erro ao ler a senha");

                let nome_s: String = nome.trim().parse().expect("Erro ao transformar o username");
                let senha_s:String = senha.trim().parse().expect("Erro ao transformar a senha");

                if cadastro.usuario_existe(&nome_s).eq(&false){
                    let user: Usuario = Usuario::new(&nome_s, &senha_s);
                    cadastro.adiciona_usuario(user);
                    let i = cadastro.retorna_indice_usuario(&nome_s);
                    println!("Usuario {} cadastrado com sucesso", &nome_s);
                    let _ = cadastro.salvar(&i);
                } else {
                    println!("Usuário já existe.");
                }
            } else if menu_s == "3".to_string() {
                println!("Fechando o programa.");
                break;
            } else {
                println!("Comando inválido!");
            }
        }
}
