# SUPERTONIC TTS
Inspirado em https://github.com/supertone-inc/supertonic

## Amostras de Voz (PT-BR)

Confira abaixo a qualidade e a entonação das vozes disponíveis utilizando o mesmo texto de teste.

 - [`M1` exemplo](https://raw.githubusercontent.com/samuelsantanaoficial/supertonic/main/examples/M1.wav)
 - [`M2` exemplo](https://raw.githubusercontent.com/samuelsantanaoficial/supertonic/main/examples/M2.wav)
 - [`M3` exemplo](https://raw.githubusercontent.com/samuelsantanaoficial/supertonic/main/examples/M3.wav)
 - [`M4` exemplo](https://raw.githubusercontent.com/samuelsantanaoficial/supertonic/main/examples/M4.wav)
 - [`M5` exemplo](https://raw.githubusercontent.com/samuelsantanaoficial/supertonic/main/examples/M5.wav)
 - [`F1` exemplo](https://raw.githubusercontent.com/samuelsantanaoficial/supertonic/main/examples/F1.wav)
 - [`F2` exemplo](https://raw.githubusercontent.com/samuelsantanaoficial/supertonic/main/examples/F2.wav)
 - [`F3` exemplo](https://raw.githubusercontent.com/samuelsantanaoficial/supertonic/main/examples/F3.wav)
 - [`F4` exemplo](https://raw.githubusercontent.com/samuelsantanaoficial/supertonic/main/examples/F4.wav)
 - [`F5` exemplo](https://raw.githubusercontent.com/samuelsantanaoficial/supertonic/main/examples/F5.wav)


## Compilação & Instalação

### Instale as dependências
```bash
sudo pacman -S git git-lfs rust
```
```bash
git lfs install
```

### Clone o repositório e compile
> Se for no **Termux+proot** faça tudo na home `cd ~`
```bash
git clone https://github.com/samuelsantanaoficial/supertonic.git
cd supertonic
```
```bash
cargo build --release
```

### Clone os pesos (VOZES) e copie para um local seguro.
```bash
git clone https://huggingface.co/Supertone/supertonic-3 assets
```
```bash
mkdir -p ~/.local/share/supertonic
cp -r assets ~/.local/share/supertonic/
```

#### Copie o binario para um local onde você possa chamar de qualquer lugar.
- Desktop (usuário normal, precisa de sudo):
```bash
sudo cp target/release/supertonic /usr/local/bin/
```
- Termux+proot como root (sem sudo):
```bash
cp target/release/supertonic /usr/local/bin/
```

## Como usar

```bash
supertonic "Insira o seu texto aqui. Use gramática do Brasil para o modelo fixar o sotaque brasileiro." teste.wav
```
```bash
supertonic -f roteiro.txt narracao.wav
```
```bash
supertonic -f roteiro.txt narracao.wav --voice F2
```
```bash
supertonic "This is an english test." test.wav --lang en --voice M2
```
```bash
Using CPU for inference

Gerando áudio (Voz: M1, Idioma: pt)
Processando TTS...
⠒ [========================================] 8/8 (2s)
  -> Processando TTS completed in 2.68 sec

Salvo com sucesso em: saida.wav
```

> **Nota sobre o Sotaque (PT-BR vs PT-PT):** A tag do idioma é a mesma (`pt`). O modelo infere o sotaque correto pelo contexto. Textos muito curtos ou neutros podem soar como Portugal. Para garantir o sotaque do Brasil, escreva da forma mais natural possível com o nosso vocabulário (uso de gerúndios, gírias locais, "você", etc).

## Vozes
https://huggingface.co/Supertone/supertonic-3

### Vozes Masculinas

|Voz|Características|Melhor para|
|---|---|---|
|**M1**|Animado, enérgico, tom claro e confiante|Vídeos promocionais, explicações informais, anúncios gerais|
|**M2**|Grave, robusto, calmo e sério|Conteúdo corporativo, anúncios sérios, documentários|
|**M3**|Polido, autoritativo, confiável|Apresentações de negócios, mensagens de liderança|
|**M4**|Suave, neutro, jovem e amigável|Conteúdo educacional, guias de onboarding|
|**M5**|Caloroso, calmo, tom de contador de histórias|Audiobooks, conteúdo relaxante, narrativa emocional|

### Vozes Femininas

|Voz|Características|Melhor para|
|---|---|---|
|**F1**|Calma, tom levemente grave, estável|Atendimento ao cliente, instruções guiadas|
|**F2**|Alegre, animada, jovem, enérgica|Conteúdo jovem, anúncios divertidos, redes sociais|
|**F3**|Clara, estilo locutora, articulada|Comerciais, documentários, estilo jornalístico|
|**F4**|Nítida, confiante, expressiva|Explicações de negócios, vídeos de treinamento|
|**F5**|Gentil, suave, calmante|Audiobooks, mensagens de apoio, conteúdo de bem-estar|
