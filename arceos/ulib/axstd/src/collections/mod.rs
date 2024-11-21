
use alloc::vec::Vec;


pub struct HashMap<K, V> {
    // fields and methods will be added here
    bucket:Vec<(K,V)>
}
impl <K,V> HashMap<K,V>{
    pub fn new()->Self{
        HashMap{
            bucket:Vec::new()
        }
    }
    pub fn insert(&mut self,key:K,value:V){
        self.bucket.push((key,value));
    }
    pub fn iter(&self)->core::slice::Iter<(K,V)>{
        self.bucket.iter()
    }

    
}
