struct Dog {}

trait Animal {
    fn sleep(&self);
}

impl Animal for Dog {
    fn sleep(&self) {
        println!("Dog is sleeping now!!!")
    }
}

/// fix this function 1)Box<dyn Animal>
//fn animal_sleep(animal: Animal){
//    animal.sleep();
//}

fn animal_sleep_v2(animal: Box<dyn Animal>) {
    animal.sleep();
}

fn dog_sleep(dog: Dog) {
    dog.sleep();
}

fn dog_sleep_reference(dog: &Dog) {
    dog.sleep();
}

fn main() {
    let dog = Dog {};
    dog.sleep();

    let box_dog = Box::new(dog);
    animal_sleep_v2(box_dog);

    let dog = Dog {};
    dog_sleep(dog);

    let dog = Dog {};
    dog_sleep_reference(&dog);
}
