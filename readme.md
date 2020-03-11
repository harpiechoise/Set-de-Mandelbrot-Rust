# Implementación del ejercicio del Set de Mandelbrot en Rust

![Imagen Generada con el programa](./README.png)

El set de Mandelbrot (__conjunto de Mandelbrot__) es él más estudiado de los fractales. Se conoce así gracias en honor al matemático Benoît Mandelbrot, que investigó sobre él  en los años 60's.

Este conjunto se define en el plano complejo fijando un número complejo `c` cualquiera. A partir de `c` se construye una sucesión por recursión:

![Ecuación condicional](./x.svg)

Si esta sucesión queda acotada, entonces se dice que `c` pertenece al conjunto de Mandelbrot, y si no, queda excluido del mismo.

Por ejemplo, si c = 1 obtenemos la sucesión 0, 1, 2, 5, 26..., que diverge. Como no esta acotada, 1 no es un elemento del conjunto de Mandelbrot.

En cambio, si c=-1 obtenemos la sucesión 0, -1. 0, -1 ..., que si es acotada, y por tanto -1 si pertenece al conjunto de Mandelbrot,

A menudo se representa el conjunto mediante el __algoritmo de tiempo de escape__. En ese caso, los colores de los puntos que no pertenecen al conjunto indican la velocidad con la que este diverge (tiende al infinito, en modulo).

# Motivación para implementar el algoritmo

A parte de estar aprendiendo Rust, realmente cuando leemos código hecho en otro programa, para tener que una pequeña idea de lo que el programa esta realizando y como funciona a fondo hay que desglosar un poco de forma matemática el problema, y la motivación de implementar el set de Mandelbrot en Rust, fue probar la concurrencia de Rust, ya que en otros lenguajes de programación suele ser un proceso muy lento generar una imagen de las proporciones en que se generara luego de esta explicación a detalle del código en si.

## Tiempo de Escape

Rust como otros lenguajes tiene una sintaxis para ciclos infinitos
los cuales nosotros decidimos cuando salir de estos loops, aquí va la sinxais del ciclo infinito en Rust

~~~rust
fn square_loop(mut x: f64) {
  loop {
    x = x * 2;
  }
}
~~~

En la vida real, Rust no puede sabes si esa variable X se usa para eso, tampoco se va a computar el valor si nosotros ejecutamos el programa, pero asumamos que si se va a usar el valor, ¿entonces que pasa con el valor de x?, cualquier número al cuadrado menor a 1 lo hace más pequeño, y así se acerca a cero; y si elevamos un número mayor a 1 lo hace más grande, entonces este se acerca al infinito; y elevar un número negativo lo hace positivo dependiendo del comportamiento del mismo.

Entonces dependiendo del valor que le pasemos a esta función se va a aproximar a cero, seguirá siendo 1 o va a tender al infinito

Ahora considera este otro ciclo:

~~~rust
fn square_add_loop(c: f64) {
  let mut x = 0;
  loop {
    x = x * x + c;
  }
}
~~~

En este caso, x comienza en cero y vamos haciendo un ajuste en cada iteración, sumando c después de elevarlo. Hace más difícil saber el valor de X, después de un tanto de experimentación si c es mayor que 0.25 y menor que -2.0, x eventualmente se hace infinitamente grande, de otro modo, se queda cerca del cero.

En este caso, en vez de usar valores `f64` vamos a usar valores complejos.

Ahora vamos a explicar como funciona la versión preliminar de la función de escape

~~~rust
fn complex_square_add_loop(c: Complex<f64>) {
  let mut z = Complex {re: 0.0, im: 0.0};
  loop {
    z = z * z + c
  }
}
~~~
La forma tradicional de denominar a los números complejos es z, en este caso z es un número complejo que nos provee la `crate num` que es parte de las dependencias, este tipo de dato está definido de la siguiente manera

~~~rust
struct Compex<t> {
  /// Porcion real del numero complejo
  re: T,

  /// Porcion imaginaria del numero complejo
  im: T
}
~~~

El código anterior define un struct llamada Complex, que tiene dos campos re e im, y una estructura genérica <T>, que quiere decir que este es cualquier tipo de valor, por ejemplo puedo decir Complex<f64> es un número complejo que acepta solo números de tipo flotante en su parte real e imaginaria

La versión final del algoritmo de tiempo de escape se ve así

~~~rust
extern crate num;
use num::Complex;

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
~~~

Esta función toma el número complejo `c` que queremos que sea el miembro que vamos a ver si pertenece al conjunto de mandelbroot, como C es probable que no sea un miembro del conjunto devolvemos un `enum` llamado `Option` que contiene 2 variantes

~~~rust
enum Option<T> {
  None,
  Some(T)
}
~~~

## El resto del código

El resto de código se puede entender por cualquier programador de Rust, si estas aprendiendo Rust te recomiendo leerlo detalladamente para luego buscar cada parte del codigo en Google :D.


## Ejecutar el programa

~~~sh
git clone <repo_url> mandelbrot
cd mandelbrot
cargo build --release
cd target
cd release
./mandelbrot prueba.png 1920x1080 -1.20,0.35 -1,0.20
~~~
