use anyhow::Result;
use clap::Parser;
use std::mem;
use indicatif::{ProgressBar, ProgressStyle};

mod helper;

use helper::{
    load_text_to_speech, load_voice_style, timer, write_wav_file, count_chunks,
};

#[derive(Parser, Debug)]
#[command(name = "supertonic")]
#[command(about = "CLI do Supertonic TTS", long_about = None)]
struct Args {
    /// O texto que será transformado em áudio (se não usar -f)
    #[arg(default_value = "")]
    text: String,

    /// O caminho do arquivo de saída (ex: audio.wav)
    #[arg(default_value = "saida.wav")]
    output: String,

    /// Caminho para ler um arquivo de texto (.txt)
    #[arg(short, long)]
    file: Option<String>,

    /// Idioma para a síntese (padrão: pt)
    #[arg(long, default_value = "pt")]
    lang: String,

    /// Estilo de voz (F1-F5, M1-M5)
    #[arg(long, default_value = "M1")]
    voice: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // 1. Processa de onde virá o texto (da linha de comando ou do arquivo .txt)
    let texto_final = if let Some(caminho_arquivo) = &args.file {
        std::fs::read_to_string(caminho_arquivo).expect("Erro ao ler o arquivo .txt")
    } else if !args.text.is_empty() {
        args.text.clone()
    } else {
        eprintln!("Erro: Forneça um texto entre aspas ou use a flag -f para ler um arquivo.");
        std::process::exit(1);
    };

    let home = std::env::var("HOME").expect("Erro ao ler a variável HOME do sistema");
    let onnx_dir = format!("{}/.local/share/supertonic/assets/onnx", home);
    let voice_style_path = format!("{}/.local/share/supertonic/assets/voice_styles/{}.json", home, args.voice);

    // --- Carrega os modelos --- //
    let mut text_to_speech = load_text_to_speech(&onnx_dir, false)?;
    let style = load_voice_style(&[voice_style_path], false)?;

    println!("Gerando áudio (Voz: {}, Idioma: {})", args.voice, args.lang);

    // --- Sintetiza --- //
    let n_chunks = count_chunks(&texto_final, &args.lang);
    let total_steps = (n_chunks * 8) as u64; // 8 = total_step usado abaixo

    let pb = ProgressBar::new(total_steps);
    pb.set_style(
        ProgressStyle::with_template("{spinner:.green} [{bar:40.cyan/blue}] {pos}/{len} ({elapsed})")
            .unwrap()
            .progress_chars("=>-"),
    );

    let mut on_step = || {
        pb.inc(1);
    };

    let (wav, duration) = timer("Processando TTS", || {
        text_to_speech.call(&texto_final, &args.lang, &style, 8, 1.05, 0.3, Some(&mut on_step))
    })?;

    eprintln!(); // pula linha depois dos pontinhos de debug

    // --- Salva o arquivo --- //
    let actual_len = (text_to_speech.sample_rate as f32 * duration) as usize;
    let wav_slice = &wav[..actual_len.min(wav.len())];

    write_wav_file(&args.output, wav_slice, text_to_speech.sample_rate)?;
    println!("Salvo com sucesso em: {}", args.output);

    mem::forget(text_to_speech);
    unsafe {
        libc::_exit(0);
    }
}