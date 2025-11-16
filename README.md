# Port Scanner CLI
Um scanner de portas TCP rápido e eficiente escrito em Rust, utilizando paralelização para varredura de múltiplas portas simultaneamente.
🚀 Funcionalidades

Varredura de portas TCP em hosts locais ou remotos
Suporte a varredura por intervalo de portas ou portas específicas
Paralelização configurável com pool de threads
Timeout configurável para conexões
Identificação automática de serviços conhecidos
Resolução de nomes de domínio para endereços IP
Interface colorida e amigável no terminal

📋 Pré-requisitos

Rust 1.70 ou superior
Cargo (gerenciador de pacotes do Rust)

🔧 Instalação
Clone o repositório e compile o projeto:
bashgit clone <seu-repositorio>
cd <nome-do-projeto>
cargo build --release
O binário compilado estará disponível em target/release/.
💻 Uso
Sintaxe Básica
bash./port-scanner --host <HOST> [OPÇÕES]
Opções
OpçãoDescriçãoPadrão--host <HOST>Host alvo (IP ou domínio)Obrigatório--start-port <START>Porta inicial do intervalo---end-port <END>Porta final do intervalo--p, --ports <PORTS>Lista de portas específicas (separadas por vírgula)--t, --threads <THREADS>Número de threads para paralelização100--timeout <TIMEOUT>Timeout de conexão em milissegundos1000
Exemplos de Uso
Varrer portas comuns em um domínio:
bash./port-scanner --host example.com --start-port 1 --end-port 1000
Varrer portas específicas:
bash./port-scanner --host 192.168.1.1 -p 22,80,443,3306
Varrer todas as portas (1-65535):
bash./port-scanner --host localhost
Varredura rápida com mais threads e timeout menor:
bash./port-scanner --host 192.168.1.1 --start-port 1 --end-port 1000 -t 200 --timeout 500
Varrer múltiplas portas com vírgula:
bash./port-scanner --host scanme.nmap.org -p 21,22,23,25,80,443,8080
```

## 📊 Saída

O scanner exibe as portas abertas no seguinte formato:
```
22/tcp ABERTA - SSH
80/tcp ABERTA - HTTP
443/tcp ABERTA - HTTPS
3306/tcp ABERTA - MySQL

Portas abertas são exibidas em verde
O status "ABERTA" aparece em negrito
Serviços conhecidos são identificados automaticamente

🔍 Serviços Reconhecidos
O scanner identifica automaticamente os seguintes serviços:

FTP (21)
SSH (22)
Telnet (23)
SMTP (25)
DNS (53)
HTTP (80)
POP3 (110)
IMAP (143)
HTTPS (443)
MySQL (3306)
RDP (3389)
PostgreSQL (5432)
HTTP-Alt (8080)

🛠️ Dependências

clap - Parser de argumentos de linha de comando
rayon - Paralelização de dados
owo-colors - Colorização de output no terminal
thiserror - Tratamento de erros
once_cell - Inicialização lazy de estáticos

⚠️ Avisos

Use esta ferramenta apenas em hosts que você tem permissão para varrer
Varreduras de portas podem ser detectadas por sistemas de segurança
Alguns firewalls podem bloquear ou limitar tentativas de conexão
Respeite as leis e políticas de segurança locais

📝 Notas

Se nenhum intervalo ou lista de portas for especificado, o scanner varrerá todas as portas (1-65535)
O número de threads afeta diretamente a velocidade da varredura
Timeouts menores aceleram a varredura, mas podem gerar falsos negativos
A ferramenta resolve automaticamente nomes de domínio para endereços IP

🤝 Contribuindo
Contribuições são bem-vindas! Sinta-se à vontade para abrir issues ou pull requests.Tentar novamente
