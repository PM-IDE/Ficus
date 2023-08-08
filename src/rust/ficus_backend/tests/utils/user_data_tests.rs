use std::rc::Rc;

use ficus_backend::utils::user_data::{Key, UserData};

#[test]
fn test_user_data() {
    let key = Key::<usize>::new("asdasdasda".to_string());
    let mut user_data = UserData::new();
    let b = Box::new(123);
    user_data.put(&key, b);

    assert_eq!(*user_data.get(&key).unwrap(), 123);
}

#[test]
fn test_user_data_two_keys() {
    let first_key = Key::<Rc<Box<usize>>>::new("1".to_string());
    let second_key = Key::<Rc<Box<usize>>>::new("2".to_string());

    let first_value = Rc::new(Box::new(123));
    let second_value = Rc::new(Box::new(321));
    let box1 = Box::new(Rc::clone(&first_value));
    let box2 = Box::new(Rc::clone(&second_value));

    let mut user_data = UserData::new();

    user_data.put(&first_key, box1);
    user_data.put(&second_key, box2);

    assert_eq!(user_data.get(&first_key).unwrap(), &first_value);
    assert_eq!(user_data.get(&second_key).unwrap(), &second_value);

    assert!(Rc::ptr_eq(user_data.get(&first_key).unwrap(), &first_value));
    assert!(Rc::ptr_eq(user_data.get(&second_key).unwrap(), &second_value));
}

#[test]
fn test_remove_from_user_data() {
    let key = Key::<usize>::new("1".to_string());

    let value = 123;
    let mut user_data = UserData::new();

    user_data.put(&key, Box::new(value));

    assert_eq!(*user_data.get(&key).unwrap(), 123);

    user_data.remove(&key);

    assert_eq!(user_data.get(&key), None);
}
