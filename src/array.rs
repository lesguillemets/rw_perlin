pub struct TwoDArray<T> {
    pub f: Vec<T>,
    pub w: u32,
    pub h: u32,
}

impl<T> TwoDArray<T> {
    pub fn at(&self, x: u32, y: u32) -> Option<&T> {
        self.f.get(x as usize + y as usize * self.w as usize)
    }
    pub fn at_mut(&mut self, x: u32, y: u32) -> Option<&mut T> {
        self.f.get_mut(x as usize + y as usize * self.w as usize)
    }
    pub fn at_unchecked(&self, x: u32, y: u32) -> &T {
        &self.f[x as usize + y as usize * self.w as usize]
    }
    pub fn at_unchecked_mut(&mut self, x: u32, y: u32) -> &T {
        &mut self.f[x as usize + y as usize * self.w as usize]
    }
}
