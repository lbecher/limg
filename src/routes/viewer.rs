use actix_web::{post, HttpResponse, Error};
use actix_multipart::Multipart;
use futures_util::StreamExt as _;
use std::fs;

#[post("/viewer")]
pub async fn viewer(mut payload: Multipart) -> Result<HttpResponse, Error> {
    let mut limg_data: Vec<u8> = Vec::new();

    while let Some(item) = payload.next().await {
        let mut field = item?;
        let content_type = field.content_disposition();

        if let Some(name) = content_type.get_name() {
            match name {
                "file" => {
                    while let Some(chunk) = field.next().await {
                        let bytes: Vec<u8> = bytemuck::cast_slice(&chunk.unwrap()).to_vec();
                        for byte in bytes {
                            limg_data.push(byte);
                        }
                    }
                }
                "red_length" => {}
                "green_length" => {}
                "blue_length" => {}
                _ => {
                    println!("Campo não esperado ignorado!");
                }
            }
        }
    }

    let width =
        (limg_data[4] as u32) +
        ((limg_data[5] as u32) << 8) +
        ((limg_data[6] as u32) << 16) +
        ((limg_data[7] as u32) << 24);
    let height =
        (limg_data[8] as u32) +
        ((limg_data[9] as u32) << 8) +
        ((limg_data[10] as u32) << 16) +
        ((limg_data[11] as u32) << 24);
    
    let red_length = limg_data[12].clone();
    let green_length = limg_data[13].clone();
    let blue_length = limg_data[14].clone();
    
    let mut red: u16;
    let mut green: u16;
    let mut blue: u16;

    let mut pixels: Vec<u8> = Vec::new();

    for i in 0..((width as usize) * (height as usize)) {
        // lê bits do buffer
        red = (limg_data[(i * 2) + 15] as u16) + ((limg_data[(i * 2) + 16] as u16) << 8);
        green = (limg_data[(i * 2) + 15] as u16) + ((limg_data[(i * 2) + 16] as u16) << 8);
        blue = (limg_data[(i * 2) + 15] as u16) + ((limg_data[(i * 2) + 16] as u16) << 8);

        // zera bits de outras cores
        green = green & (0xFFFF >> red_length);
        blue = blue & (0xFFFF >> (red_length + green_length));

        // ajusta posição dos bits
        red = red >> (green_length + blue_length);
        green = green >> blue_length;

        // conversão para cores de 8 bits
        red = conv_to_8(red, red_length) as u16;
        green = conv_to_8(green, green_length) as u16;
        blue = conv_to_8(blue, blue_length) as u16;

        // adiciona cores ao vetor
        pixels.push(red as u8);
        pixels.push(green as u8);
        pixels.push(blue as u8);
        pixels.push(255);
    }

    let mut html = fs::read_to_string("static/html/viewer.html").unwrap();
    html = html.replace("{2}", &format!("{}", width));
    html = html.replace("{3}", &format!("{}", height));
    html = html.replace("{1}", &format!("{:?}", pixels));
    Ok(HttpResponse::Ok().body(html))
}

fn conv_to_8(cor: u16, comp: u8) -> f64 {
    ((cor as f64) / (((1 << comp) - 1) as f64)) * 255.0
}