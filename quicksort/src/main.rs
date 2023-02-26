/**
 * Program to solve Kattis task "Quicksort"
 * See: https://kth.kattis.com/problems/kth.alginda.quicksort
 * Author: Jonathan Blomlöf <jblomlof@kth.se>
 */
use std::cmp::Ordering;
use std::io::Read;
const THRESHOLD_FOR_INSERT: usize = 100;
fn main() {
    /*
    According to Vilhelm Prytz the kattis tests first spits out amount of digits and then all the digits.

    BTW THANK YOU Prytz for the link to kattis task.
    */

    //It looks like kattis doesnt like my current method to get values
    // so using the solution which (vprytz) implemented.
    // (its just io so i assume its fine.)

    /*let mut _stdin = std::io::stdin().lock();
    //let _file = File::open("test.txt").unwrap();
    //let mut _stdin = BufReader::new(_file);
    let mut _line = String::new();
    _stdin.read_line(&mut _line);
    let mut _values: Vec<isize> = Vec::with_capacity(_line.trim().parse().unwrap_or(100));

    _line.clear();
    while _stdin.read_line(&mut _line).unwrap() > 0 {
        _values.push(_line.trim().parse().unwrap());
        _line.clear();
    }
    */
    let mut _input = String::new();
    std::io::stdin().lock().read_to_string(&mut _input);
    let mut _values: Vec<isize> = _input
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse().unwrap())
        .collect();

    quick_sort(&mut _values[..]);

    for _val in _values {
        print!("{} ", _val);
    }
    println!("");
}

fn quick_sort(values: &mut [isize]) {
    /*
    values.sort();
    return;
    // jk, jk.
    */
    if values.len() <= THRESHOLD_FOR_INSERT {
        insert_sort(values);
        return;
    }

    let mut pivot_index = get_pivot_index(values);
    let mut lower_of_higher = pivot_index + 1;
    let pivot_value = values[pivot_index];
    //we store the dupes after the pivot_index
    let mut dupes_of_pivot_element = 0;
    let mut lower = 0;
    while lower < pivot_index {
        match values[lower].cmp(&values[pivot_index]) {
            Ordering::Equal => {
                pivot_index -= 1;
                dupes_of_pivot_element += 1;
                values[lower] = values[pivot_index];
                values[pivot_index] = pivot_value;
            }
            Ordering::Greater => {
                // assert!(values[lower] > values[pivot_index]);

                values[pivot_index + dupes_of_pivot_element] = values[lower];
                pivot_index -= 1;
                values[lower] = values[pivot_index];
                values[pivot_index] = pivot_value;
            }
            Ordering::Less => {
                // assert!(values[lower] < values[pivot_index]);
                lower += 1;
            }
        }
    }
    //we now have some statements that hold true
    /*
    *  all elements before the pivot_element are less than pivot_elemnt
        ("i < pivot_index" =>(implies) "values[i] < values[pivot_index]")

    * we have dupes_of_pivot_element stored after pivot_element
        ("pivot_index < i <=(less then or equal) pivot_index + dupes_of_pivot_element" =>(implies) "values[i] = values[pivot_index]")

    * all elemnts after the pivot and dupes until start_pivot is greater than pivot_element
        ( "pivot_index + dupes_of_pivot_element < i <= start_pivot" =>(implies) "values[i] > values[pivot_index]")
        (start-pivot is the return value from the func. get_pivot_index)
    */
    // thus we now need to do the above with the rest.

    while lower_of_higher < values.len() {
        match values[lower_of_higher].cmp(&values[pivot_index]) {
            Ordering::Greater => {
                // assert!(values[lower_of_higher] > values[pivot_index])
                lower_of_higher += 1;
            }
            Ordering::Equal => {
                dupes_of_pivot_element += 1;
                let _temp = values[pivot_index + dupes_of_pivot_element];
                values[pivot_index + dupes_of_pivot_element] = pivot_value;
                values[lower_of_higher] = _temp;
                lower_of_higher += 1;
            }
            Ordering::Less => {
                // assert!(values[lower_of_higher] < values[pivot_index])
                values[pivot_index] = values[lower_of_higher];
                pivot_index += 1;
                values[lower_of_higher] = values[pivot_index + dupes_of_pivot_element];
                values[pivot_index + dupes_of_pivot_element] = pivot_value;
                lower_of_higher += 1;
            }
        }
    }
    quick_sort(&mut values[..pivot_index]);
    quick_sort(&mut values[(pivot_index + dupes_of_pivot_element + 1)..])
}

fn get_pivot_index(_values: &[isize]) -> usize {
    let a = 0;
    let b = _values.len() / 2;
    let c = _values.len() - 1;

    if _values[a] < _values[b] {
        if _values[b] < _values[c] {
            b
        } else if _values[c] < _values[a] {
            a
        } else {
            c
        }
    } else {
        // _val[a] >= _val[b]
        if _values[b] > _values[c] {
            b
        } else if _values[c] > _values[a] {
            a
        } else {
            c
        }
    }
}

/*
Insertion sort I wrote in task "task-sorting", which was pushed to https://github.com/IndaPlus22/jblomlof-sorting
And then removed since it wasn't needed for that task.

Modified to no longer keep track of changes made
 */
fn insert_sort(_list: &mut [isize]) {
    for i in 1.._list.len() {
        let x = _list[i];
        let mut j: isize = i as isize - 1;
        while (j >= 0) && (_list[j as usize] > x) {
            _list[j as usize + 1] = _list[j as usize];
            j -= 1;
        }
        _list[(j + 1) as usize] = x;
    }
}