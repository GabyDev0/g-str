# g-str
una librería desarrollada en rust, que evita la duplicación de cadenas en tu codigo
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
