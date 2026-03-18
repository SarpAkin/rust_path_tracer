
pub fn encode_pixel(r:f32,g : f32,b:f32,a:f32) -> u32{
    (((r.clamp(0.0, 1.0)* 255.0) as u32) << 00) 
    | (((g.clamp(0.0, 1.0)* 255.0) as u32) << 08) 
    | (((b.clamp(0.0, 1.0)* 255.0) as u32) << 16) 
    | (((a.clamp(0.0, 1.0)* 255.0) as u32) << 24) 
}
