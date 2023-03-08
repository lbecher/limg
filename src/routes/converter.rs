use actix_web::{post, HttpResponse, Error};
use actix_multipart::Multipart;
use futures_util::StreamExt as _;
use image::GenericImageView;
use std::path::Path;

#[post("/converter")]
pub async fn converter(mut payload: Multipart) -> Result<HttpResponse, Error> {
    let mut extension: String = ".bmp".to_string();

    let mut img_data: Vec<u8> = Vec::new();

    let mut red_length: u8 = 0;
    let mut green_length: u8 = 0;
    let mut blue_length: u8 = 0;

    while let Some(item) = payload.next().await {
        let mut field = item?;
        let content_type = field.content_disposition();

        if let Some(filename) = content_type.get_filename() {
            let path = Path::new(filename);
            if let Some(ext) = path.extension() {
                extension = ext.to_str().unwrap().to_string();
            }
        }

        if let Some(name) = content_type.get_name() {
            match name {
                "file" => {
                    while let Some(chunk) = field.next().await {
                        let bytes: Vec<u8> = bytemuck::cast_slice(&chunk.unwrap()).to_vec();
                        for byte in bytes {
                            img_data.push(byte);
                        }
                    }
                }
                "red_length" => {
                    if let Some(chunk) = field.next().await {
                        red_length = std::str::from_utf8(&chunk?).unwrap().parse::<u8>().unwrap();
                    }
                }
                "green_length" => {
                    if let Some(chunk) = field.next().await {
                        green_length = std::str::from_utf8(&chunk?).unwrap().parse::<u8>().unwrap();
                    }
                }
                "blue_length" => {
                    if let Some(chunk) = field.next().await {
                        blue_length = std::str::from_utf8(&chunk?).unwrap().parse::<u8>().unwrap();
                    }
                }
                _ => {
                    println!("Campo não esperado ignorado!");
                }
            }
        }
    }

    let temp_file_path = format!("{}/img_{}.{}", std::env::temp_dir().display(), std::process::id(), extension.as_str());

    std::fs::write(temp_file_path.as_str(), img_data).unwrap();

    let img = image::open(temp_file_path.as_str()).unwrap();
    let (width, height) = img.dimensions();
    
    let mut limg: Vec<u8> = Vec::new();

    // adiciona primeiros bytes do arquivo
    limg.push(b'L');
    limg.push(b'I');
    limg.push(b'M');
    limg.push(b'G');

    // adiciona bytes da largura e da altura
    for i in 0..4 {
        limg.push((((0xFF << (i * 8)) & (width as u32)) >> (i * 8)) as u8);
    }
    for i in 0..4 {
        limg.push((((0xFF << (i * 8)) & (height as u32)) >> (i * 8)) as u8);
    }

    // adiciona bytes dos comprimentos de cada cor
    limg.push(red_length);
    limg.push(green_length);
    limg.push(blue_length);
    
    let mut red: u16;
    let mut green: u16;
    let mut blue: u16;
    let mut pixel: u16;

    for i in 0..height {
        for j in 0..width {
            // obtém pixel
            let img_pixel = img.get_pixel(j, i);
            red = img_pixel[0] as u16;
            green = img_pixel[1] as u16;
            blue = img_pixel[2] as u16;

            // conversão dos canais de cores
            red = conv_from_8(red, red_length) as u16;
            green = conv_from_8(green, green_length) as u16;
            blue = conv_from_8(blue, blue_length) as u16;

            // ajuste da posição dos bits
            red = red << (green_length + blue_length);
            green = green << blue_length;

            // concatenação dos bits
            pixel = red + green + blue;

            // adiciona bytes do pixel
            limg.push((0x00FF & pixel) as u8);
            limg.push(((0xFF00 & pixel) >> 8) as u8);
        }
    }

    std::fs::remove_file(temp_file_path.as_str()).unwrap();

    Ok(HttpResponse::Ok()
        .content_type("application/octet-stream")
        .append_header(("Content-Disposition", "attachment; filename=\"converted.limg\""))
        .body(limg.clone()))
}

fn conv_from_8(cor: u16, comp: u8) -> f64 {
    ((cor as f64) / 255.0) * (((1 << comp) - 1) as f64)
}