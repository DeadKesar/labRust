
use image::{DynamicImage, GenericImageView, Pixel};


use std::io;
use std::path::Path;

fn calculate_brightness_and_contrast(image: &DynamicImage) -> [f64; 2] {
    let (width, height) = image.dimensions();
    let mut brightness_sum = 0.0;
    let mut brightness_values = Vec::new();

    // Проходим по каждому пикселю изображения
    for pixel in image.pixels() {
        let rgb = pixel.2.to_rgb();
        let r = rgb[0] as f64;
        let g = rgb[1] as f64;
        let b = rgb[2] as f64;

        // Рассчитываем яркость пикселя (взвешенная яркость)
        let brightness = 0.299 * r + 0.587 * g + 0.114 * b;
        brightness_sum += brightness;
        brightness_values.push(brightness);
    }

    // Средняя яркость
    let avg_brightness = brightness_sum / (width * height) as f64;

    // Стандартное отклонение (контрастность)
    let contrast = (brightness_values.iter()
        .map(|&b| (b - avg_brightness).powi(2))
        .sum::<f64>()
        / brightness_values.len() as f64)
        .sqrt();

    [avg_brightness, contrast]
}





fn main() {
    while true {
        let mut file_path = String::new();
        println!("введите путь к изображению c расширением или exit:\n");
        io::stdin().read_line(&mut file_path).expect("ошибка чтения строки.");
        let file_path = file_path.trim();
        if file_path == "exit" {
            break;
        }
        let path = Path::new(file_path);

        if path.exists() && path.is_file() {
            println!("файл существует: {}", file_path);
            let mut img = image::open(file_path).expect("Ошиба при открытии изображения.");

            while true {
                println!("Выберите опцию для работы с изображением (введите цифру 1-4):\n\
                1) изменить яркось.\n\
                2) Изменить контрасность. \n\
                3) Задать новый размер. \n\
                4) выйти. \n");
                let mut ans = String::new();
                io::stdin().read_line(&mut ans).expect("ошибка чтения строки.");
                let ans = ans.trim();
                match ans {
                    "1" => {
                        loop {
                            println!("текущая яркость = {}", calculate_brightness_and_contrast(&img)[0]);
                            println!("Введите значение для изменения яркости (-255 до 255):");
                            let mut val = String::new();
                            io::stdin().read_line(&mut val).expect("ошибка чтения строки.");
                            let val = val.trim().parse::<i32>();
                            if val.is_err() {
                                println!("Попробуйте снова!");
                                continue
                            }
                            let val = val.unwrap();
                            if val >= -255 && val <= 255 {
                                img = img.brighten(val);
                                img.save(file_path).unwrap();

                                break
                            }
                            println!("Попробуйте снова!")
                        }
                    }
                    "2" => {
                        loop {
                            println!("текущая контрастность = {}", calculate_brightness_and_contrast(&img)[1]);
                            println!("Введите значение для изменения контрастности (от -100 до 100):");
                            let mut val = String::new();
                            io::stdin().read_line(&mut val).expect("ошибка чтения строки.");
                            let val = val.trim().parse::<f32>();
                            if val.is_err() {
                                println!("Попробуйте снова!");
                                continue
                            }
                            let val = val.unwrap();
                            if val >= -100.0 && val <= 100.0 {
                                img = img.adjust_contrast(val);
                                img.save(file_path).unwrap();
                                break
                            }
                            println!("Попробуйте снова!")
                        }
                    },
                    "3" => {
                        loop {
                            let (mut width, mut height) = img.dimensions();
                            println!("Ширина: {}, Высота: {}", width, height);
                            println!("Введите новый ширину и высоту изображения, разделенные пробелом (например, 800 600):");
                            let mut val = String::new();
                            io::stdin().read_line(&mut val).expect("ошибка чтения строки.");
                            let dimensions: Vec<&str> = val.trim().split_whitespace().collect();
                            if dimensions.len() == 2 {
                                let w = dimensions[0].parse::<u32>();
                                let h = dimensions[1].parse::<u32>();
                                if w.is_err() || h.is_err() {
                                    println!("Попробуйте снова!");
                                    continue
                                }
                                width = w.unwrap();
                                height = h.unwrap();
                            }
                            else {
                                println!("Пожалуйста, введите корректные числовые значения для ширины и высоты.");
                                continue;
                            }
                            //println!("w = {}, h = {}", width, height);
                            img = img.resize_exact(width, height, image::imageops::FilterType::Lanczos3);
                            img.save(file_path).unwrap();
                            break
                        }
                    },
                    "4" => break,
                    _ => println!("попробуйте ещё раз"),
                }

            }

            break;

        } else if path.exists() && path.is_dir() {
            println!("введёный путь это директория а не файл...");
        } else {
            println!("Путь не найден.");
        }
    }


}