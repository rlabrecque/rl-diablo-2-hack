pub struct D2Library {
    pub name: String,
    pub handle: winapi::shared::minwindef::HMODULE,
}

impl D2Library {
    pub fn new(name: String) -> Self {
        let file_name: widestring::WideCString = widestring::WideCString::from_str(&name).unwrap();
        let process_handle = rlwindows::load_library(file_name.as_ptr());
        if process_handle.is_null() {
            println!("Failed to load library: {}", name);
        }

        println!("{} successfully loaded at offset {:?}", name, process_handle);

        D2Library {
            name: name,
            handle: process_handle,
        }
    }

    pub fn is_loaded(&self) -> bool {
        !self.handle.is_null()
    }

    pub fn fix_offset(&self, offset: usize) -> usize {
        (self.handle as usize) + offset
    }

    pub unsafe fn read<T>(&self, offset: usize) -> &T {
        &*(self.fix_offset(offset) as *const T)
    }

    pub unsafe fn read_mut<T>(&self, offset: usize) -> &mut T {
        &mut *(self.fix_offset(offset) as *mut T)
    }

    pub unsafe fn write<T>(&mut self, offset: usize, value: T) {
        *(self.fix_offset(offset) as *mut T) = value;
    }
}

impl Drop for D2Library {
    fn drop(&mut self) {
        if self.is_loaded() {
            rlwindows::free_library(self.handle);
        }
    }
}
