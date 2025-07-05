use crate::core::INFINITY;

pub struct Interval{
    min: f64,
    max: f64,
}

pub enum IntervalWorldChoice{
    Empty,
    Universe,
}
impl Interval{
    pub fn new(min: f64, max: f64) -> Self{
        Self{min, max}
    }

    pub fn default() -> Self{
        Self{
            min: INFINITY,
            max: INFINITY,
        }
    }

    pub fn max(&self) -> f64{
        self.max
    }

    pub fn min(&self)-> f64{
        self.min
    }
    pub fn size(&self) -> f64{
        self.max - self.min
    }

    pub fn contains(&self, x: f64) -> bool{
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f64) -> bool{
        self.min < x && x < self.max
    }

    pub fn world_choice(choice: IntervalWorldChoice) ->Interval{
        let interval = match choice {
            IntervalWorldChoice::Empty => Self::new(INFINITY, -INFINITY),
            IntervalWorldChoice::Universe => Self::new(-INFINITY, INFINITY) 
        };
        interval
    }
}