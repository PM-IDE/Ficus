use ficus_backend::utils::user_data::{Key, UserData};


#[test]
fn test_user_data() {
    let key = Key::<usize>::new(&"asdasdasda".to_string());
    let mut user_data = UserData::new();
    let b = Box::new(123);
    user_data.put(&key, b);

    assert_eq!(*user_data.get(&key).unwrap(), 123);

    let value = user_data.get_mut(&key).unwrap();
    *value = 321;

    assert_eq!(*user_data.get(&key).unwrap(), 321);
}
