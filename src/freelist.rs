
pub type PageNum = u64;

pub struct Freelist {
   max_page: PageNum,
   released_pages: Vec<u64>,
}

impl Freelist {
    pub fn new() -> Self {
        Freelist {
            max_page:0,
            released_pages: Vec::new(),
        }
    }
     
    pub fn get_next_page(&mut self) -> PageNum {
        let next_page = self.released_pages.iter().take(1).next();
    
        if let Some(page) = next_page {
            page.to_owned()
        }
        else {
            self.max_page = self.max_page.saturating_add(1);
            self.max_page
        }
            
    }

    fn release_page(&mut self, page_num: PageNum) {
        self.released_pages.push(page_num);
        
    }

    fn serialize<const A: usize>(& self, arr: &mut[u8; A]) {
        let length:u64 = u64::try_from(self.released_pages.len()).unwrap();
        
        self.serializ(arr, length);

        for ele in self.released_pages.iter() {
            self.serializ(arr, ele.clone());
        }

    }

    fn deserialize<const A: usize>(&mut self, arr: [u8; A]) {
        let max_page:u64 = self.deserializ(&arr);
        
        let mut a = arr.chunks_exact(8); let b = arr.chunks_exact(8);

        let max_page = self.byte_to_u64( a.nth(0).unwrap());
        let page_count = self.byte_to_u64( a.nth(0).unwrap());

        for ele in a {
            let val = self.byte_to_u64(ele);
            self.released_pages.push(val);           
        }

        

    }

    pub fn deserializ<const A: usize>(&mut self, array: &[u8; A]) -> u64 {
        let mut l = [0u8; 8];
        l[..8].copy_from_slice(&array[0..8]);
        self.byte_to_u64(&l)
       // let a = &array[0..8].c;

    }

    
    fn byte_to_u64 (&mut self, array: &[u8]) -> u64 {
        ((array[0] as u64) <<  0) +
        ((array[1] as u64) <<  8) +
        ((array[2] as u64) << 16) +
        ((array[3] as u64) << 24) +
        ((array[4] as u64) << 32) +
        ((array[5] as u64) << 40) +
        ((array[6] as u64) << 48) +
        ((array[7] as u64) << 56) 
    }

    pub fn serializ<const A: usize>(&self, arr: &mut[u8; A], num:u64) {
        
        let num :[u8; 8]= num.to_le_bytes();
        assert!(A >= num.len()); //just for a nicer error message, adding #[track_caller] to the function may also be desirable

        arr[..A].copy_from_slice(&num);

       
    }
}