# g-str
GStr es una librería diseñada para ser simple y rápida. Proporciona un **internado de cadenas**, lo que significa que su función es evitar la duplicación innecesaria de cadenas.

```rust
	fn main() {
		let string1 = GStr::new("hola");
		let string2 = GStr::new("hola");
		println!("{string1} Jhon!");
		if string1 == string2{
			println!("Somos iguales");
		}
	}
```

GStr ofrece una estructura y una Trait:

- GStr: estructura que envuelve la cadena de manera segura y evita la duplicación de la cadena.

- StringInfo: Es una Trait con el propósito principal de manejar valores de manera generalizada. Esto significa que proporciona métodos que la estructura `GStr` puede utilizar sin importar el **tipo de valor que se le pase**.

para mas información puedes buscar la documentación detallada en [docs.rs/g-str/](https://docs.rs/g-str/).

