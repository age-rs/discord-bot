// STD Dependencies -----------------------------------------------------------
use std::fs::File;


// Internal Dependencies ------------------------------------------------------
use super::{Chunk, Track};


// Audio Writer Trait ---------------------------------------------------------
pub trait AudioWriter: Send {
    fn write_header(&mut self, &mut File, &Track);
    fn write_chunk(&mut self, &mut File, Chunk);
}


// Ogg Writer Implementation --------------------------------------------------
pub struct OggWriter;

impl AudioWriter for OggWriter {

    fn write_header(&mut self, _: &mut File, _: &Track) {

    }

    fn write_chunk(&mut self, _: &mut File, _: Chunk) {

    }

}
