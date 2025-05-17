use std::{ops::{Deref, DerefMut}, sync::{Arc, Mutex}};


#[derive(Clone)]
///Just a helper to test drop behavior
pub struct Dropper {
    reference: Arc<Mutex<bool>>,
}

impl Dropper {
    pub fn new() -> Self {
        Dropper{reference: Arc::new(Mutex::new(false))}

    }

    pub fn dropped(&self) -> bool{
        *self.reference.lock().unwrap()
    }

    pub fn reset(&self){
        *self.reference.lock().unwrap().deref_mut() = false;
    }
}

impl Drop for Dropper {
    fn drop(&mut self) {
        *self.reference.lock().unwrap().deref_mut() = true;
    }
}



#[test]
///Ensures that dropper actually works
fn dropper_checks_drop() {


    let dropper = Dropper::new();
    let cloned = dropper.clone();

    assert_eq!(dropper.dropped(), false);


    drop(dropper);


    assert_eq!(cloned.dropped(), true)
}


