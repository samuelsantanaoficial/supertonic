# SUPERTONIC TTS

## Compilação & Instalação

```bash
sudo pacman -S git git-lfs rust git lfs install
```
```bash
git clone https://github.com/samuelsantanaoficial/supertonic.git
cd supertonic
```
```bash
cargo build --release
```
```bash
git clone https://huggingface.co/Supertone/supertonic-3 assets
```
```bash
mkdir -p ~/.local/share/supertonic
cp assets ~/.local/share/supertonic/
```
```bash
cp target/release/supertonic ~/.local/bin/
```

## Como usar

```bash
supertonic "Insira o seu texto aqui. Use gramática do Brasil para o modelo fixar o sotaque brasileiro." dito.wav
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
⠒ [========================================] 8/8 (2s)                                                                                                                                                                                         -> Processando TTS completed in 2.68 sec

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