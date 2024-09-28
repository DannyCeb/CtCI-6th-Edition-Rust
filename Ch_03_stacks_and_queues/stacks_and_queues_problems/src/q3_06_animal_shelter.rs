pub mod solution {

    #[derive(Clone, Copy, Debug)]
    pub enum Animal {
        Dog(u32),
        Cat(u32),
    }

    impl Animal {
        pub fn get_id(&self) -> u32 {
            match *self {
                Animal::Cat(id) => id,
                Animal::Dog(id) => id,
            }
        }
    }

    impl PartialEq for Animal {
        fn eq(&self, other: &Self) -> bool {
            match (*self, *other) {
                (Animal::Cat(_), Animal::Cat(_)) => true,
                (Animal::Dog(_), Animal::Dog(_)) => true,
                _ => false,
            }
        }
    }

    pub struct AnimalShelter {
        pub cats: Vec<Animal>,
        pub dogs: Vec<Animal>,
    }

    impl AnimalShelter {
        pub fn new() -> Self {
            Self {
                cats: Vec::new(),
                dogs: Vec::new(),
            }
        }

        pub fn enqueue(&mut self, animal: Animal) {
            match animal {
                Animal::Dog(_) => {
                    self.dogs.push(animal);
                }
                Animal::Cat(_) => {
                    self.cats.push(animal);
                }
            }
        }

        pub fn dequeue_cat(&mut self) -> Option<Animal> {
            if !self.cats.is_empty() {
                Some(self.cats.remove(0))
            } else {
                None
            }
        }

        pub fn dequeue_dog(&mut self) -> Option<Animal> {
            if !self.dogs.is_empty() {
                Some(self.dogs.remove(0))
            } else {
                None
            }
        }

        pub fn dequeue_any(&mut self) -> Option<Animal> {
            match (self.cats.is_empty(), self.dogs.is_empty()) {
                (true, false) => self.dequeue_dog(),
                (false, true) => self.dequeue_cat(),
                (false, false) => match self
                    .cats
                    .first()
                    .unwrap()
                    .get_id()
                    .cmp(&self.dogs.first().unwrap().get_id())
                {
                    std::cmp::Ordering::Less => self.dequeue_cat(),
                    _ => self.dequeue_dog(),
                },
                (true, true) => None,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::q3_06_animal_shelter::solution::Animal;

    use super::solution::AnimalShelter;

    #[test]
    fn test_functions() {
        let mut animal_shelter = AnimalShelter::new();

        assert_eq!(None, animal_shelter.dequeue_any());

        for l in 0..100 {
            if l % 2 == 0 {
                animal_shelter.enqueue(super::solution::Animal::Cat(l));
            } else {
                animal_shelter.enqueue(super::solution::Animal::Dog(l));
            }
        }

        assert_eq!(Some(Animal::Cat(0)), animal_shelter.dequeue_any());
        assert_eq!(Some(Animal::Dog(1)), animal_shelter.dequeue_dog());
        assert_eq!(Some(Animal::Dog(3)), animal_shelter.dequeue_dog());
        assert_eq!(Some(Animal::Dog(5)), animal_shelter.dequeue_dog());
        assert_eq!(Some(Animal::Dog(7)), animal_shelter.dequeue_dog());
        assert_eq!(Some(Animal::Cat(2)), animal_shelter.dequeue_any());
    }
}
