use std::ops::Add;

#[derive(Debug)]
struct Point<T>{
    x: T,
    y: T,
}

impl<T> Add for Point<T>
    where
    T: Add<Output = T>{
        type Output = Self; //struct point<T>
        fn add(self, right: Self) -> Self::Output{
            Point{
                x: self.x + right.x,
                y: self.y + right.y,
            }
        }
    }

fn main(){
    let coord  = Point{ x: 4, y: 8};
    let coord2 = Point{ x: 2, y: 7};

    let sum =  coord + coord2;
    print!("{:?}", sum);
}