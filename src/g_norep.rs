use std::ptr;
use std::sync::Mutex;

// estructura de la lista
pub(crate) struct GListNodo {
	pub(crate) begin: *mut GStrInterner,
	pub(crate) end: *mut GStrInterner
}
// lista
pub(crate) static mut GLIST_NODO : GListNodo = GListNodo{
	begin: ptr::null_mut(),
	end: ptr::null_mut()
};
// mutex
pub(crate) static mut GNOREP_LOOCK : Mutex<()> = Mutex::new(());

pub(crate) struct GStrInterner {
	pub(crate) value: String,
	pub(crate) len: usize,
	pub(crate) count: usize,
	pub(crate) hash: u32,
	pub(crate) next: *mut GStrInterner,
	pub(crate) prev: *mut GStrInterner
}

const CHAR_COUSIN : u32= 486187739;
const CHAR_S : u32= 31;
pub(crate) fn ohash(c: &str, mlen: &mut usize) -> u32 {
	let mut hash : u32 = 0;
	let mut len : usize = 0;
	let mut iter = c.chars();
	while let Some(ch) = iter.next() {
		len+=1;
		hash = hash.wrapping_mul(CHAR_S).wrapping_add(ch as u32).wrapping_mul(len as u32);
	} 
	(*mlen) = len;
	hash % CHAR_COUSIN
}
impl GStrInterner {
	pub(crate) fn remove(&mut self) {
		if self.count == 0{
			unsafe {
				let _unused = GNOREP_LOOCK.lock().expect("No se pudo bloquear el mutex");
				if self.next != ptr::null_mut() {
					(*self.next).prev = self.prev;
				}else{
					GLIST_NODO.end = self.prev;
				}
				if self.prev != ptr::null_mut(){
					(*self.prev).next = self.next;
				}else{
					GLIST_NODO.begin = self.next;
				}
			}
		}
	}
	pub(crate) fn compare(&self, hash: u32, len: usize, strn: &str) -> bool {
		self.hash == hash && 
		self.len == len && 
		self.value == strn
	}
}