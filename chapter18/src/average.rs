#[derive(Debug)]
pub struct AvgCollection {
    list: Vec<i32>,
    average: f64,
}

impl AvgCollection {
    pub fn new(list: Vec<i32>) -> AvgCollection {
        let sum: i32 = list.iter().sum();
        let average = sum as f64 / list.len() as f64;
        AvgCollection { list, average }
    }

    pub fn stock(&mut self, element: i32) {
        self.list.push(element);
        self.update_avg();
    }

    pub fn del(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_avg();
                Some(value)
            }
            None => None
        }
    }

    pub fn avg(self) -> f64 {
        self.average
    }

    fn update_avg(&mut self) {
        let sum: i32 = self.list.iter().sum();
        self.average = sum as f64 / self.list.len() as f64;
    }
}
