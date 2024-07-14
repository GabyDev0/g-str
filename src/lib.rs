//! La estructura `GStr` es una envoltura segura para cadenas que evita la duplicación de las mismas.
//! ```rust
//! use g_str::GStr;
//! fn main() {
//!     let cadena = GStr::new("hola");
//! }
//! ```
//! GStr ofrece metodos para:

//! - **Evitar la duplicación de cadenas**: 
//! Al internar cadenas, se asegura de que cada cadena única 
//! se almacene solo una vez, evitando duplicaciones innecesarias.

//! <br/><br/>

//! - **Contar caracteres rápidamente**: Proporciona una forma rápida de obtener el número de caracteres en una cadena.

//! <br/><br/>

//! - **Comparación eficiente**: Permite comparar cadenas sin tener que recorrer cada carácter, utilizando referencias para una comparación más rápida.

//! <br/><br/>

//! - **Envoltorio seguro**: Permite que una cadena esté presente en varias variables a la vez, utilizando un contador de referencias para gestionar la vida útil de la cadena. Esto asegura que la cadena se mantenga en memoria mientras sea necesaria y se elimine automáticamente cuando ya no se use.

//! 
//! `GStr` utiliza un **contador de referencias** para gestionar la vida útil de las cadenas (Similar a lo que hace la estructura Rc). Esto significa que cada vez que una nueva variable hace referencia a una cadena, el contador de referencias aumenta. Cuando una variable deja de usar la cadena, el contador disminuye.
mod g_norep;
use crate::g_norep::*;
use std::ops::Deref;
use std::ptr;
use std::fmt::{
    Display,Formatter
};
use std::thread;

/// La trait `StringInfo` es esencial para trabajar con GStr. 
/// Su propósito principal es manejar valores de manera 
/// generalizada. Esto significa que proporciona métodos que 
/// la estructura `GStr` puede utilizar sin importar el **tipo de 
/// valor que se le pase**. En otras palabras, `StringInfo` define una serie de **métodos y comportamientos** que `GStr` puede aplicar a cualquier valor, asegurando que la manipulación de cadenas sea **eficiente y sin duplicación**.
pub trait StringInfo {
    /// Devuelve una referencia a una cadena constante utilizada para la búsqueda de cadenas existentes.
    fn get_str_ref(&self) -> &str;

    /// Esta función se invoca cuando se ha realizado una búsqueda y no se ha encontrado ninguna coincidencia. En este caso, la función debe proceder a crear una nueva cadena.
    fn get_str(self) -> String; 
}
#[doc(hidden)]
impl StringInfo for String {
    fn get_str_ref(&self) -> &str {
        self.as_ref()
    }
    fn get_str(self) -> String {
        self
    } 
}
#[doc(hidden)]
impl StringInfo for &String {
    fn get_str_ref(&self) -> &str {
        self.as_ref()
    }
    fn get_str(self) -> String {
        String::from(self)
    }
}
#[doc(hidden)]
impl StringInfo for &str {
    fn get_str_ref(&self) -> &str {
        self
    }
    fn get_str(self) -> String {
        self.to_string()
    }
}
/// estructura que envuelve la cadena de manera segura y evita la duplicación de la cadena.
/// ```rust
/// use g_str::GStr;
/// fn main() {
///     let cadena = GStr::new("hola");
/// }
/// ```
/// GStr utiliza un contador de referencias para gestionar la vida útil de las cadenas (Similar a lo que hace la estructura Rc). Esto significa que cada vez que una nueva variable hace referencia a una cadena, el contador de referencias aumenta. Cuando una variable deja de usar la cadena, el contador disminuye.
pub struct GStr {
    value: *mut GStrInterner
}

impl GStr {
    fn search_value(strn: &str, len: usize, hash: u32, mut value: *mut GStrInterner) -> Option<GStr>{
        // recorrer la lista
        unsafe {
            while value != ptr::null_mut() {
                if (*value).compare(hash,len, strn) {

                    (*value).count+=1;
                    return Some(GStr {
                        value: value
                    });

                }
                value = (*value).next;
            }
        }
        None
    }
    fn create_gstr(vstr: String, len: usize, hash: u32) -> GStr {
        unsafe {    
            let gstr = GStrInterner {
                value: vstr,
                len,hash,
                count: 1,
                next: ptr::null_mut(),
                prev: GLIST_NODO.end
            };
            let pun_str = Box::into_raw(Box::new(gstr));
        
            // si hay un elemento anterior, actualizar su valor next
            if GLIST_NODO.end != ptr::null_mut(){
                (*GLIST_NODO.end).next = pun_str;
            }
            GLIST_NODO.end = pun_str;
            // se debe colocar un valor a begin si no tiene
            if GLIST_NODO.begin == ptr::null_mut(){
                GLIST_NODO.begin = GLIST_NODO.end;
            }
            GStr {
                value: GLIST_NODO.end
            }
        }
    }
    /// Esta función realiza una búsqueda en las cadenas previamente creadas para encontrar una que coincida con la cadena recibida.
    /// - **Si la cadena existe**, la función te devuelve un `GStr` que apunta a esa cadena existente.
    /// - **Si la cadena no existe**, la función crea una nueva cadena y te devuelve un `GStr` que apunta a esta nueva cadena.
    pub fn new<T: StringInfo>(strn: T) -> GStr{
        let mut len : usize = 0;
        let hash : u32 = ohash(strn.get_str_ref(),&mut len);
        unsafe {
            // bloquear mutex
            let _unused = GNOREP_LOOCK.lock().expect("No se pudo bloquear el mutex");
            let value : *mut GStrInterner = GLIST_NODO.begin;

            match GStr::search_value(strn.get_str_ref(), len, hash, value) {
                Some(v) => return v,
                _=> {}
            } 

           
            GStr::create_gstr(strn.get_str(), len, hash)
        }
    }
    /// Esta función devuelve la cantidad de caracteres que contiene la cadena.
    pub fn chars_count(&self) -> usize {
        unsafe { (*self.value).len } 
    }
}
impl AsRef<str> for GStr {
    /// Esta función retorna una referencia inmutable a la cadena.
    fn as_ref(&self) -> &str {
        unsafe { &(*self.value).value }
    }
}
impl Clone for GStr {
    /// Clona el `GStr`. No crea una copia de la cadena, simplemente crea un nuevo `GStr` que apunta a la **cadena existente**.
    fn clone(&self) -> Self{ 
        unsafe {(*self.value).count+= 1;}
        GStr {
            value: self.value
        }
    }
}   

impl Drop for GStr {
    /// funcion para determinar si eliminar la cadena.
    fn drop(&mut self) {
        let value = self.value;
        unsafe {
            (*value).count-= 1;
            if (*value).count == 0 {
                (*value).remove();
                let _ = Box::from_raw(value); // dejar que rust elimine el valor
            }
        }
    }
}

impl Display for GStr {
    /// permite imprimir el `GStr` en una cadena directamente sin acceder a la cadena.
    /// ```rust
    /// use g_str::GStr;
    /// fn main() {
    ///     let nombre = GStr::new("hola");
    ///     println!("hola {nombre}");
    /// }
    /// ```
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}",self.as_ref())
    }
}

impl PartialEq for GStr {
    /// sobrecarga al operador `==` para ver si dos cadenas son las mismas, comparando su ubicacion de memoria, es decir, comparar dos referencias en vez de hacer un analisis caracter por caracter.
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
    /// igual que lo anterior, pero sobrecargando el operador `!=` para ver si dos cadenas son diferentes
    fn ne(&self, other: &Self) -> bool {
        self.value != other.value
    }
}

#[doc(hidden)]
impl Deref for GStr {
    type Target = str;
    /// devuelve la cadena que envuelve
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}





#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
       let handle = thread::spawn(|| {
        let mut temp = Vec::new();
        for i in 50..150 {
            temp.push(
                GStr::new(format!(
                    "hola{i}"
                ))
            );
        }
        for u in 0..100 {
            for i in 50..150 {
                let vtemp = GStr::new(format!(
                    "hola{i}"
                ));
                println!("{vtemp}={i}");
                assert_eq!(temp.get(i-50).unwrap().clone() == vtemp,true );
            }
        }
       });
       let mut temp = Vec::new();
       for i in 0..100 {
            temp.push(
                GStr::new(format!(
                    "hola{i}"
                ))
            );
       }
        for u in 0..100 {
            for i in 0..100 {
                let vtemp = GStr::new(format!(
                    "hola{i}"
                ));
                println!("{vtemp}={i}");
                assert_eq!(temp.get(i).unwrap().clone() == vtemp,true );
            }
        }

        handle.join().unwrap();
        temp.clear();
       
    }
}