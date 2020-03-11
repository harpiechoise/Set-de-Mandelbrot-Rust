extern crate num;
use num::Complex;
extern crate crossbeam;


/// Intentamos determinar si `c` es un miembro del set de Mandelbrot,
/// Unsando el parametro limit como el limite de iteraciones que decidamos
/// Si `c` pertenece al conjunto se devuelve `Some(i)` donde i es el numero de
/// iteraciones que se dieron antes de que C se saliera del circulo de radio 2.0
/// Si no se puede probar que `c` pertenezca al conjunto luego del limite de iteraciones
/// se retorna `None`
fn escape_time(c: Complex<f64>, limit: u32) -> Option<u32> {
     let mut z = Complex { re: 0.0, im: 0.0 };
         for i in 0..limit {
         z = z*z + c;
         if z.norm_sqr() > 4.0 {
             return Some(i);
         }
     }
 None
}


use std::str::FromStr;

/// Esta funcion toma como valor una cadena de texto `s` como un par de coordenadas
/// Como por ejemplo 1.0,0.5 o 400x200 en caso de que sea la resolucion de la imagen de salida
///
/// Si s tiene la forma que buscabamos que es <left><sep><right>, donde <sep> es el carácter
/// separador del lado izquiero del lado derecho, las dos cadenas se pueden pasar a la funcion
/// T::from_str
///
/// Si S no tiene la forma correcta se returna None

fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
            None => None,
            Some(index) => {
                match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
                    (Ok(l), Ok(r)) => Some((l, r)),
                _ => None
            }
        }
    }
}

/// Esta funcion busca un par de numeros complejos separados por ,
/// La parte derecha es la parte real y la parte izquiera es la parte imaginaria
fn parse_complex(s: &str) -> Option<Complex<f64>> {
     match parse_pair(s, ',') {
         Some((re, im)) => Some(Complex { re, im }),
         None => None
    }
}


/// Dada una fila y una columna de pixeles de la imagen de salida se retorna el plano complejo respectivo
/// `bounds` es un par de coordenadas que indica el ancho y alto de la imagen en Pixeles
/// `pixel` es una par de valores que representan la (fila, columna) indicando la posicion de un pixel en particular
/// `upper_left`, `upper_right`: son puntos que designan que desigan el area que va a cubrir nuestra imagen
fn pixel_to_point(bounds: (usize, usize),
                  pixel: (usize, usize),
                  upper_left: Complex<f64>,
                  lower_right: Complex<f64>)
            -> Complex<f64>
{
 let (width, height) = (lower_right.re - upper_left.re,
                        upper_left.im - lower_right.im);
 Complex {
     re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
     im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64
 }
}


/// Renderiza un rectangulo del que contiene la imagen del conjunto de MandelBrot en un buffer de pixeles
///
/// `bounds` le da el ancho y alto que deberia tener el bufer
/// `pixels` es una variable que va a almacenar la representacion en escala de grises de la imagen
/// `upper_right` y `upper_left` indican las esquinas del buffer de pixeles
fn render(pixels: &mut [u8],
     bounds: (usize, usize),
     upper_left: Complex<f64>,
     lower_right: Complex<f64>)
    {
        assert!(pixels.len() == bounds.0 * bounds.1);
     for row in 0 .. bounds.1 {
         for column in 0 .. bounds.0 {
            let point = pixel_to_point(bounds, (column, row),
            upper_left, lower_right);
            pixels[row * bounds.0 + column] =
            match escape_time(point, 255) {
                None => 0,
                Some(count) => 255 - count as u8
            };
        }
    }
}

extern crate image;

use image::ColorType;
use image::png::PNGEncoder;
use std::fs::File;


/// Guarda el buffer de piexeles dentro de un archivo
/// `filename`: Nombre del archivo

fn write_image(filename: &str, pixels: &[u8], bounds: (usize, usize))
                    -> Result<(), std::io::Error>
{
 let output = File::create(filename)?;
 let encoder = PNGEncoder::new(output);
 encoder.encode(&pixels,
     bounds.0 as u32, bounds.1 as u32,
     ColorType::Gray(8))?;
 Ok(())
}


use std::io::Write;
/// Ejecuta todas las funciones que se describieron anteriormente
fn main() {
    // Se obtienen los argumentos desde la consola
     let args: Vec<String> = std::env::args().collect();
     if args.len() != 5 {
         writeln!(std::io::stderr(),
         "Usage: mandelbrot FILE PIXELS UPPERLEFT LOWERRIGHT")
            .unwrap();
            // En caso de error se muestra el `usage` del programa
            writeln!(std::io::stderr(),
            "Example: {} mandel.png 1000x750 -1.20,0.35 -1,0.20",
            args[0]).unwrap();
            std::process::exit(1);
     }
     // Si todo sale bien tomamos todos los argumentos y buscamos cualquier excepcion
     // Dependiendo de la excepcion mostramos un mensaje de error indicando el problema
     let bounds = parse_pair(&args[2], 'x')
     .expect("error parsing image dimensions");
     let upper_left = parse_complex(&args[3])
     .expect("error parsing upper left corner point");
     let lower_right = parse_complex(&args[4])
     .expect("error parsing lower right corner point");
     let mut pixels = vec![0; bounds.0 * bounds.1];
     let threads = 8;
     let rows_per_band = bounds.1 / threads + 1;

     {
      // Separamos todo en bandas para poder construir la imagen de manera asincrona
      let bands: Vec<&mut [u8]> =
      pixels.chunks_mut(rows_per_band * bounds.0).collect();
      // Preparamos la librria crossbeam para generare la imagen
      crossbeam::scope(|spawner| {
          // Generamos la imagen por bandas de pixeles
          // Toda la imagen se escribira dentro de la variable
          // Pïxels
          for (i, band) in bands.into_iter().enumerate() {
          let top = rows_per_band * i;
          let height = band.len() / bounds.0;
          let band_bounds = (bounds.0, height);
          let band_upper_left =
          pixel_to_point(bounds, (0, top), upper_left, lower_right);
          let band_lower_right =
          pixel_to_point(bounds, (bounds.0, top + height),
          upper_left, lower_right);
          spawner.spawn(move || {
              render(band, band_bounds, band_upper_left, band_lower_right);
          });
        }
    });
    }
  // Guardamos la imagen
  write_image(&args[1], &pixels, bounds)
 .expect("error writing PNG file");
}
