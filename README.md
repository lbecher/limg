# L-IMG

Conversor de imagem para o formato L-IMG. L-IMG é um formato de 
imagem desenvolvido por Luiz Fernando Becher de Araujo para o 
trabalho extra da disciplina de Processamento de Imagens Digitais.

# Como executar

Execute um dos comandos abaixo na raiz do projeto, de acordo com
o seu sistema operacional.

No Windows:

```
.\run.bat
```

No Linux:

```
./run.sh
```

# Como compilar e executar

Certifique-se de ter o compilador do Rust instalado: [Instale o Rust](https://www.rust-lang.org/tools/install).

Depois, execute o comando abaixo na raiz do projeto:

```
cargo run -r
```

Por fim, abra a interface da aplicação em [http://localhost:8080](http://localhost:8080).

# Especificação do formato L-IMG

| byte de início | tamanho em bytes | descrição |
| --- | --- | --- |
| 0 | 4 | Assinatura do arquivo. Sempre "LIMG". |
| 4 | 4 | Largura da imagem em pixels. |
| 8 | 4 | Altura da imagem em pixels. |
| 12 | 1 | Comprimento em bits do canal da cor vermelha. |
| 13 | 1 | Comprimento em bits do canal da cor verde. |
| 14 | 1 | Comprimento em bits do canal da cor azul. |
| 15 | - | Vetor de pixels. |

Cada pixel possui 2 bytes e as cores são ordenadas na ordem vermelho, verde e azul.
A orientação dos pixels é da esquerda para direita e de cima para baixo.
