// a:effectLst
use super::Glow;
use super::OuterShadow;
use super::SoftEdge;
use writer::driver::*;
use quick_xml::Reader;
use quick_xml::events::{Event, BytesStart};
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Default, Debug)]
pub struct EffectList {
    glow: Option<Glow>,
    outer_shadow: Option<OuterShadow>,
    soft_edge: Option<SoftEdge>,
}
impl EffectList {
    pub fn get_glow(&self) -> &Option<Glow> {
        &self.glow
    }

    pub fn get_glow_mut(&mut self) -> &mut Option<Glow> {
        &mut self.glow
    }

    pub fn set_glow(&mut self, value:Glow) {
        self.glow = Some(value);
    }

    pub fn get_outer_shadow(&self) -> &Option<OuterShadow> {
        &self.outer_shadow
    }

    pub fn get_outer_shadow_mut(&mut self) -> &mut Option<OuterShadow> {
        &mut self.outer_shadow
    }

    pub fn set_outer_shadow(&mut self, value:OuterShadow) {
        self.outer_shadow = Some(value);
    }

    pub fn get_soft_edge(&self) -> &Option<SoftEdge> {
        &self.soft_edge
    }

    pub fn get_soft_edge_mut(&mut self) -> &mut Option<SoftEdge> {
        &mut self.soft_edge
    }

    pub fn set_soft_edge(&mut self, value:SoftEdge) {
        self.soft_edge = Some(value);
    }

    pub(crate) fn set_attributes(
        &mut self,
        reader:&mut Reader<std::io::BufReader<std::fs::File>>,
        _e:&BytesStart
    ) {
        let mut buf = Vec::new();
    
        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Empty(ref e)) => {
                    match e.name() {
                        b"a:softEdge" => {
                            let mut obj = SoftEdge::default();
                            obj.set_attributes(reader, e);
                            &mut self.set_soft_edge(obj);
                        },
                        _ => (),
                    }
                },
                Ok(Event::Start(ref e)) => {
                    match e.name() {
                        b"a:glow" => {
                            let mut obj = Glow::default();
                            obj.set_attributes(reader, e);
                            &mut self.set_glow(obj);
                        },
                        b"a:outerShdw" => {
                            let mut obj = OuterShadow::default();
                            obj.set_attributes(reader, e);
                            &mut self.set_outer_shadow(obj);
                        },
                        _ => (),
                    }
                },
                Ok(Event::End(ref e)) => {
                    match e.name() {
                        b"a:effectLst" => return,
                        _ => (),
                    }
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "a:effectLst"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:effectLst
        write_start_tag(writer, "a:effectLst", vec![], false);

        // a:glow
        match &self.glow {
            Some(v) => v.write_to(writer),
            None => {},
        }

        // a:outerShdow
        match &self.outer_shadow {
            Some(v) => v.write_to(writer),
            None => {},
        }

        // a:softEdge
        match &self.soft_edge {
            Some(v) => v.write_to(writer),
            None => {},
        }

        write_end_tag(writer, "a:effectLst");
    }
}