use crate::file_ops::FileOps;
use crate::model::Fixed;

#[allow(unused)]
#[derive(Debug)]
pub struct MaximumProfileTable {
    version: Fixed,                // 0x00010000 (1.0)
    pub num_glyphs: u16,           // the number of glyphs in the font
    max_points: u16,               // points in non-compound glyph
    max_contours: u16,             // contours in non-compound glyph
    max_component_points: u16,     // points in compound glyph
    max_component_contours: u16,   // contours in compound glyph
    max_zones: u16,                // set to 2
    max_twilight_points: u16,      // points used in Twilight Zone (Z0)
    max_storage: u16,              // number of Storage Area locations
    max_function_defs: u16,        // number of FDEFs
    max_instruction_defs: u16,     // number of IDEFs
    max_stack_elements: u16,       // maximum stack depth
    max_size_of_instructions: u16, // byte count for glyph instructions
    max_component_elements: u16,   // number of glyphs referenced at top level
    max_component_depth: u16,      // levels of recursion, set to 0 if font has only simple glyphs
}

impl MaximumProfileTable {
    pub fn from_file(file_ops: &mut FileOps, offset: u32) -> MaximumProfileTable {
        file_ops.seek_from_start(offset);
        let version: Fixed = file_ops.read_fixed();
        let num_glyphs: u16 = file_ops.read_u16();
        let max_points: u16 = file_ops.read_u16();
        let max_contours: u16 = file_ops.read_u16();
        let max_component_points: u16 = file_ops.read_u16();
        let max_component_contours: u16 = file_ops.read_u16();
        let max_zones: u16 = file_ops.read_u16();
        let max_twilight_points: u16 = file_ops.read_u16();
        let max_storage: u16 = file_ops.read_u16();
        let max_function_defs: u16 = file_ops.read_u16();
        let max_instruction_defs: u16 = file_ops.read_u16();
        let max_stack_elements: u16 = file_ops.read_u16();
        let max_size_of_instructions: u16 = file_ops.read_u16();
        let max_component_elements: u16 = file_ops.read_u16();
        let max_component_depth: u16 = file_ops.read_u16();
        MaximumProfileTable {
            version,
            num_glyphs,
            max_points,
            max_contours,
            max_component_points,
            max_component_contours,
            max_zones,
            max_twilight_points,
            max_storage,
            max_function_defs,
            max_instruction_defs,
            max_stack_elements,
            max_size_of_instructions,
            max_component_elements,
            max_component_depth,
        }
    }
}
