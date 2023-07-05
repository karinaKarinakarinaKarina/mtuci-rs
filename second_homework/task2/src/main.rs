/*
----> ЗАДАНИЕ 2 "Площадь квадрата"

Создать структуру Rect (квадрат), которая задается координатами левого верхнего угла и длиной стороны.
Добавить для этой структуры методы new(top_left: (f32, f32), width: f32) -> Rect
                                   bottom_right -> (f32, f32), // Выводит координаты правого нижнего угла
                                   area -> f32 // Вычисляет площадь квадрата
                                   perimeter -> f32 // Вычисляет периметр квадрата

 */

fn main() {}

struct Rect {
    top_left: {x: f32, y: f32 },
    width: f32
}

impl Rect {
    fn new(top_left: (f32, f32), width: f32) -> Self {
        React {
            top_left: top_left,
            width: width
        }
    }

    fn bottom_right(&self) -> (f32, f32) {
        x = -x.self.top_left;
        y = -y.self.top_left;
        (x,y)
    }

    fn area(&self) -> f32 {
        self.width**2
    }

    fn perimeter(&self) -> f32 {
        self.width * 4
    }
}


// ----> TESTS
#[cfg(test)]
mod tests {
    use crate::Rect;

    #[test]
    fn bottom_right() {
        let rect = Rect::new((1., 2.), 5.);

        assert_eq!((6., -3.), rect.bottom_right())
    }

    #[test]
    fn area() {
        let rect = Rect::new((1., 2.), 5.);

        assert_eq!(25., rect.area())
    }

    #[test]
    fn perimeter() {
        let rect = Rect::new((1., 2.), 5.);

        assert_eq!(20., rect.perimeter())
    }
}
