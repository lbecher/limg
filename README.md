# limg
Conversor de imagem para o formato L-IMG.
L-IMG é um formato de desenvolvido por Luiz Fernando Becher de Araujo para 
o trabalho extra da disciplina de Processamento de Imagens Digitais.

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
A orientação dos pixels é de cima para baixo e da esquerda para direita.
