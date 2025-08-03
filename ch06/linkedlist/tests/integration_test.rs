#[test]
fn test_linkedlist_iter() {
    use linkedlist::LinkedList;
    let test_data =
        vec!["first", "second", "third", "fourth", "fifth and last"];

    let mut linked_list = LinkedList::new();
    test_data
        .iter()
        .for_each(|s| linked_list.append(s.to_string()));

    assert_eq!(
        test_data,
        linked_list
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<&str>>()
    );

    dbg!(linked_list);
}

#[test]
fn test_linkedlist_for_loop_by_value() {
    use linkedlist::LinkedList;
    let test_data = vec!["first", "second", "third"];

    let mut linked_list = LinkedList::new();
    test_data
        .iter()
        .for_each(|s| linked_list.append(s.to_string()));

    let mut collected_data = Vec::new();
    for item in linked_list { // Consumes the list
        collected_data.push(item);
    }

    assert_eq!(
        test_data.iter().map(|s| s.to_string()).collect::<Vec<String>>(),
        collected_data
    );
}

#[test]
fn test_linkedlist_for_loop_by_ref() {
    use linkedlist::LinkedList;
    let test_data = vec!["first", "second", "third"];

    let mut linked_list = LinkedList::new();
    test_data
        .iter()
        .for_each(|s| linked_list.append(s.to_string()));

    let mut collected_data = Vec::new();
    for item_ref in &linked_list { // Borrows the list
        collected_data.push(item_ref.clone()); // Clone to own the data for comparison
    }

    assert_eq!(
        test_data.iter().map(|s| s.to_string()).collect::<Vec<String>>(),
        collected_data
    );

    // Ensure the original list is still usable
    assert_eq!(linked_list.iter().count(), test_data.len());
}

#[test]
fn test_linkedlist_for_loop_by_mut_ref() {
    use linkedlist::LinkedList;
    let test_data = vec!["first", "second", "third"];
    let expected_data = vec!["first!", "second!", "third!"];

    let mut linked_list = LinkedList::new();
    test_data
        .iter()
        .for_each(|s| linked_list.append(s.to_string()));

    for item_mut_ref in &mut linked_list { // Mutably borrows the list
        item_mut_ref.push_str("!"); // Modify in place
    }

    assert_eq!(
        expected_data.iter().map(|s| s.to_string()).collect::<Vec<String>>(),
        linked_list.into_iter().collect::<Vec<String>>() // Consume to collect final state
    );
}

#[test]
fn test_linkedlist_iter_mut() {
    use linkedlist::LinkedList;
    let test_data =
        vec!["first", "second", "third", "fourth", "fifth and last"];

    let mut linked_list = LinkedList::new();
    test_data
        .iter()
        .for_each(|s| linked_list.append(s.to_string()));

    assert_eq!(
        test_data,
        linked_list
            .iter_mut()
            .map(|s| s.as_str())
            .collect::<Vec<&str>>()
    );
}
#[test]
fn test_linkedlist_into_iter() {
    use linkedlist::LinkedList;
    let test_data =
        vec!["first", "second", "third", "fourth", "fifth and last"];

    let mut linked_list = LinkedList::new();
    test_data
        .iter()
        .for_each(|s| linked_list.append(s.to_string()));

    assert_eq!(
        test_data
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>(),
        linked_list.into_iter().collect::<Vec<String>>()
    );
}

#[test]
fn test_linkedlist_cloned() {
    use linkedlist::LinkedList;
    let test_data =
        vec!["first", "second", "third", "fourth", "fifth and last"];

    let mut linked_list = LinkedList::new();
    test_data
        .iter()
        .for_each(|s| linked_list.append(s.to_string()));

    let cloned_list = linked_list.clone();

    linked_list
        .into_iter()
        .zip(cloned_list.into_iter())
        .for_each(|(left, right)| {
            assert_eq!(left, right);
            assert!(!std::ptr::eq(&left, &right));
        });
}
