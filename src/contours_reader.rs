use crate::file_ops::FileOps;
use crate::model::{Contour, Contours, Point, PointType};

pub struct ContoursReader<'a> {
    file_ops: &'a mut FileOps,
}

impl<'a> ContoursReader<'a> {
    pub fn new(file_ops: &mut FileOps) -> ContoursReader {
        ContoursReader { file_ops }
    }

    fn read_coordinates(
        &mut self,
        flags: &ContourFlags,
        is_short_vector: fn(&ControlPointsFlags) -> bool,
        is_same: fn(&ControlPointsFlags) -> bool,
    ) -> Vec<i16> {
        let mut coordinates = vec![];
        let mut last_elem = 0;

        flags.contour_flags.iter().for_each(|contour_flag| {
            let coordinate = if is_short_vector(contour_flag) {
                let coor = self.file_ops.read_u8() as i16;
                if is_same(contour_flag) {
                    coor
                } else {
                    -coor
                }
            } else {
                if is_same(contour_flag) {
                    0
                } else {
                    self.file_ops.read_i16()
                }
            };
            let coordinate = last_elem + coordinate;
            coordinates.push(coordinate);
            last_elem = coordinate;
        });

        coordinates
    }

    pub fn read_contours(&mut self, n: i16) -> Contours {
        let mut end_pts_of_contours: Vec<u16> = (0..n)
            .into_iter()
            .map(|_| self.file_ops.read_u16())
            .collect();
        let instruction_length: u16 = self.file_ops.read_u16();

        self.file_ops.seek_from_current(instruction_length as i32); // Skip instructions

        end_pts_of_contours.insert(0, 0);

        // The number of points is determined by the last entry in the end_pts_of_contours array.
        let flags: ContourFlags =
            ContourFlags::mk_contour_flags(self.file_ops, end_pts_of_contours.clone());

        let x_coordinates =
            self.read_coordinates(&flags, |cf| cf.x_short_vector(), |cf| cf.x_is_same());
        let y_coordinates =
            self.read_coordinates(&flags, |cf| cf.y_short_vector(), |cf| cf.y_is_same());

        // println!("x_coordinates {:?}", x_coordinates);
        // println!("y_coordinates {:?}", y_coordinates);

        let mut points: Vec<Point> = x_coordinates
            .into_iter()
            .zip(y_coordinates)
            .zip(flags.contour_flags)
            .map(|((x, y), flags)| Point::new(x, y, PointType::from(flags.on_curve())))
            .collect();

        let windows: Vec<&[u16]> = end_pts_of_contours.windows(2).collect();
        let points_per_contours: Vec<u16> = windows
            .into_iter()
            .map(|window| {
                let s = window[0];
                let e = window[1];

                if s == 0 {
                    e + 1
                } else {
                    e - s
                }
            })
            .collect();

        let contours: Vec<Contour> = points_per_contours
            .iter()
            .map(|ppc| {
                let size = *ppc as usize;
                //let taken = points.split_off(size);
                let taken = points.splice(0..size, []).collect();
                Contour { points: taken }
            })
            .collect();

        Contours { contours }
    }
}
#[derive(Debug, Copy, Clone)]
struct ControlPointsFlags(u8);

impl ControlPointsFlags {
    fn from_file(file_ops: &mut FileOps) -> ControlPointsFlags {
        ControlPointsFlags(file_ops.read_u8())
    }

    fn is_set(&self, bit: u8) -> bool {
        let shift = 1 << bit;
        self.0 & shift == shift
    }

    fn on_curve(&self) -> bool {
        self.is_set(0)
    }

    fn x_short_vector(&self) -> bool {
        self.is_set(1)
    }

    fn y_short_vector(&self) -> bool {
        self.is_set(2)
    }

    fn repeat(&self) -> bool {
        self.is_set(3)
    }

    fn x_is_same(&self) -> bool {
        self.is_set(4)
    }

    fn y_is_same(&self) -> bool {
        self.is_set(5)
    }

    #[allow(unused)]
    fn pretty_print(&self, id: &str) {
        println!("[{}], on_curve      : {:?}", id, self.on_curve());
        println!("[{}], x_short_vector: {:?}", id, self.x_short_vector());
        println!("[{}], y_short_vector: {:?}", id, self.y_short_vector());
        println!("[{}], repeat        : {:?}", id, self.repeat());
        println!("[{}], x_is_same     : {:?}", id, self.x_is_same());
        println!("[{}], y_is_same     : {:?}", id, self.y_is_same());
    }
}

#[derive(Debug)]
struct ContourFlags {
    contour_flags: Vec<ControlPointsFlags>,
}

impl ContourFlags {
    fn mk_contour_flags(file_ops: &mut FileOps, end_pts_of_contours: Vec<u16>) -> ContourFlags {
        let last = end_pts_of_contours.last().unwrap();
        println!("last                {:?}", last);

        let contour_flags_total: Vec<ControlPointsFlags> =
            Self::_mk_contour_flags(file_ops, *last + 1);

        ContourFlags {
            contour_flags: contour_flags_total,
        }
    }
    fn _mk_contour_flags(
        file_ops: &mut FileOps,
        mut number_of_points: u16,
    ) -> Vec<ControlPointsFlags> {
        let mut contour_flags: Vec<ControlPointsFlags> =
            Vec::with_capacity((number_of_points + 1) as usize);
        while number_of_points > 0 {
            let control_points = ControlPointsFlags::from_file(file_ops);
            if control_points.repeat() {
                // If repeat is set, the next byte specifies the number of additional times this set of flags is to be repeated.
                let mut repeat_times = file_ops.read_u8();

                while repeat_times > 0 {
                    contour_flags.push(control_points);
                    repeat_times -= 1;
                    number_of_points -= 1;
                }
            }
            contour_flags.push(control_points);
            number_of_points -= 1;
        }
        contour_flags
    }
}
