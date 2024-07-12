//! GStr es una librería desarrollada en rust, que evita la duplicación de cadenas en tu codigo.
//! ```rust
//!     let cadena = GStr::new("hola");
//! ```
//! GStr ofrece metodos para:
//! - Evitar la duplicacion de cadenas
//! - forma rapida de saber el numero de caracteres de la cadena
//! - Comparación eficiente sin tener que recorrer la cadena
//! - Envoltorio seguro que permite que la cadena este en varias variables a la vez 
//! 
//! GStr agrega una estructura con el mismo nombre `GStr`.
//! Esta estructura envuelve a la cadena, funciona como un Rc<&str> pero con una funciones especificas.

mod g_norep;
use crate::g_norep::*;
use std::ops::Deref;
use std::ptr;
use std::fmt::{
    Display,Formatter
};
use std::thread;

/// estructura que envuelve la cadena de manera segura.
/// ```rust
///     let cadena = GStr::new("hola");
/// ```
/// GStr funciona como un contador de referencias, para eliminar la cadena una vez que no se necesita.
pub struct GStr {
    value: *mut GStrInterner
}

impl GStr {
    /// esta funcion busca en las cadenas creadas anteriormente una que sea la misma a la que enviaste como parametro.
    /// si existe te devuelve un `GStr` que apunta a la cadena. Si no existe, La crea y te devuelve un `GStr` a la cadena nueva.
    /// ```rust
    ///     let cadena = GStr::new("Esta es la cadena a buscar, y crear si no existe");
    /// ```
    pub fn new(strn: &str) -> GStr{
        let mut len : usize = 0;
        let hash : u32 = ohash(strn,&mut len);
        unsafe {
            // bloquear mutex
            let _unused = GNOREP_LOOCK.lock().expect("No se pudo bloquear el mutex");
            let mut value : *mut GStrInterner = GLIST_NODO.begin;
            // recorrer la lista
            while value != ptr::null_mut() {
                if (*value).compare(hash,len, strn) {
                    (*value).count+=1;
                    return GStr {
                        value: value
                    };
                }
                value = (*value).next;
            }

            let vstr = String::from(strn); // crear una copia
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
    /// metodo rapido para obtener una referencia a la cadena.
    /// ```rust
    ///     let cadena = GStr::new("Esta es la cadena a buscar, y crear si no existe");
    ///     let string : &str = cadena.get();
    /// ```
    pub fn get(&self) -> &str {
        unsafe { (*self.value).value.as_ref() }
    }
    /// devuelve el numero de caracteres que tiene la cadena.
    /// ```rust
    ///     let cadena = GStr::new("Esta es la cadena a buscar, y crear si no existe");
    ///     println!("el numero de caracteres son: {}", cadena.chars_count());
    /// ```
    pub fn chars_count(&self) -> usize {
        unsafe { (*self.value).len } 
    }
}
impl Clone for GStr {
    /// Clonar el `GStr`, no crea una copia de la cadena, simplemente crea un nuevo `GStr` que apunta a la cadena existente y le incrementa el contador.
    /// ```rust
    ///     let cadena = GStr::new("Esta es la cadena a tomar el numero de caracteres");
    ///     let copia_cadena = cadena.clone();
    /// ```
    fn clone(&self) -> Self{ 
        unsafe {(*self.value).count+= 1;}
        GStr {
            value: self.value
        }
    }
}   
impl Drop for GStr {
    /// decrementa el contador de la cadena, y en caso de que sea 0 la elimina.
    /// ```rust
    ///     let cadena = GStr::new("Jhon");
    ///     cadena = GStr::new("Jhon2"); // la cadena anterior se borra
    /// ```
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
    ///     let cadena = GStr::new("Jhon");
    ///     println!("{cadena}");
    /// ```
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}",self.get())
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
        self.get()
    }
}
