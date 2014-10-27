/// Basic sorting algorithms, just for fun.

#[warn(non_camel_case_types)]
#[warn(unnecessary_qualification)]
#[warn(non_uppercase_statics)]
#[warn(missing_doc)]

use std;

fn choose_pivot<T : Ord>(slice : &[T]) -> uint {
	// if slice.len() <= 2 {return slice.len() - 1;};
	let (mut ismall, imid, mut ibig) = (0, slice.len() / 2, slice.len() - 1);
	if slice[ibig] < slice[ismall] {std::mem::swap(&mut ibig, &mut ismall);}
	if slice[imid] <= slice[ismall] {ismall}
	else if slice[ibig] <= slice[imid] {ibig}
	else{imid}
}

/// choose a pivot, then reorder so that everything to the left of the pivot is smaller, and 
/// everything to the right is greater
/// Assumes slice.len() > 2
fn partition<T : Ord>(slice : &mut [T], pivot : uint) -> uint {
	let mut pivot = pivot;
	let mxix = slice.len() - 1;
	let (mut left, mut right) = (0, mxix);
	while left + 1 < right {
		let mut retry = false;
		if slice[left] < slice[pivot] {left += 1; retry = true;}
		if slice[right] > slice[pivot] {right -= 1; retry = true;}
		if left == pivot {left += 1; retry == true;}
		if right == pivot {right -= 1; retry == true;}
		if retry {continue;}
		
		slice.swap(left, right);
	}
	
	if pivot < left && pivot < right && slice[left] < slice[pivot] {slice.swap(left, pivot); pivot = left;}
	else if pivot > right && pivot > left && slice[right] > slice[pivot] {slice.swap(right, pivot); pivot = right;};
	
	return pivot;
}

/// The quicksort algorithm, for sorting an array.
pub fn quicksort<T : Ord>(slice : &mut [T]){
	if slice.len() <= 1 {return;}
	else if slice.len() == 2 {
		if slice[0] >= slice[1] {slice.swap(0,1);}
		return;
	}
	
	let pivot = choose_pivot(slice);
	let pivot = partition(slice, pivot);
	let (left_slice, right_slice) = slice.split_at_mut(pivot);
	// left_slice is [0 - pivot-1], right_slice is [pivot, end]. We don't want to include the
	// pivot, so reassign right_slice
	let right_slice = right_slice.tail_mut();
	
	quicksort(left_slice);
	quicksort(right_slice);
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Quicksort tests
#[test]
fn test_partition() {
	let tests : &mut [uint] = [1u,2,3];
	let result : &mut [uint] = [1,2,3];
	let p = partition(tests, 1);
	assert_eq!(&tests, &result);
	assert_eq!(p, 1);
	let p = partition(tests, 0);
	assert_eq!(&tests, &result);
	assert_eq!(p, 0);
	let p = partition(tests, 2);
	assert_eq!(&tests, &result);
	assert_eq!(p, 2);
	
	let tests : &mut [uint] = [1u,3,2];
	let p = partition(tests, 1);
	let result : &mut [uint] = [1,2,3];
	assert_eq!(&tests, &result);
	assert_eq!(p, 2);
	
	let tests : &mut [uint] = [1u,3,2];
	let p = partition(tests, 0);
	let result : &mut [uint] = [1,3,2];
	assert_eq!(&tests, &result);
	assert_eq!(p, 0);
	
	let tests : &mut [uint] = [1u,3,2];
	let p = partition(tests, 2);
	let result : &mut [uint] = [1,2,3];
	assert_eq!(&tests, &result);
	assert_eq!(p, 1);
}

/// Test if a slice is in a sorted state.
pub fn is_sorted<T : Ord>(slice: &[T]) -> bool {
	for win in slice.windows(2){
		match win {
			[ref a, ref b] if a < b => continue,
			[_, _] => return false,
			_ => fail!("slice.windows(2) returned a window with size {} != 2", win.len())
		}
	}
	true
}

#[cfg(test)]
fn get_test_vecs() -> Vec<Vec<uint>> {
	vec!(
		vec!(), vec!(1), vec!(1,2), vec!(2,1), vec!(1,2,3), vec!(2,1,3), vec!(3,1,2), 
		vec!(8,5,2,6,9,3), vec!(2,3,5,6,8,9), vec!(9,8,6,5,3,2)
	)
}

#[test]
fn test_quicksort(){
	let mut test_slices = get_test_vecs();
	
	for test_vec in test_slices.iter_mut(){
		let test_slice = test_vec.as_mut_slice();
		quicksort(test_slice);
		assert!(is_sorted(test_slice));
	}
}



////////////////////////////////////////////////////////////////////////////////////////////////////
// Heapsort

/// Index of parent node
#[inline]
fn get_parent(ix : uint) -> uint {
	(ix+1) / 2 - 1
}

/// Index of leaf nodes
#[inline]
fn get_leaves(ix : uint) -> (uint, uint) {
	(ix*2 + 1, ix*2+2)
}

/// Turn the array into a maximal heap
pub fn heapify<T : Ord>(slice : &mut [T]){
	for ix in range(1, slice.len()){
		let mut curix = ix;
		while curix > 0 {
			let pix = get_parent(curix);
			if slice[pix] > slice[curix] {break;}
			
			slice.swap(pix, curix);
			curix = pix;
		}
	}
}

/// Assuming our slice is a heap, take the maximal element (element 0), swap it to the end,
/// take that end-element / now root and filter it down the heap until its in the right place.
/// At the end of this function, the max element is at the end, and elements 0 to (end-1) are a heap
/// again.
fn heap_pop<T : Ord>(slice : &mut [T]){
	if slice.len() <= 1 {return;}
	let mxix = slice.len() - 2; // last index in the new heap
	slice.swap(0, mxix+1);
	
	// Now we filter downwards.
	let mut curix = 0;
	loop {
		let (l,r) = get_leaves(curix);
		if l > mxix {
			// we reached the bottom, there are no more leaves.
			break;
		}
		let switch_ix = if (r > mxix) || (slice[l] > slice[r]) {l} else {r};
		if slice[curix] >= slice[switch_ix] {break;}
		slice.swap(curix, switch_ix);
		curix = switch_ix;
	}
}

/// Turn a heap-array into a sorted array
pub fn heap_to_sorted<T : Ord>(slice : &mut [T]){
	//~ let mut portion = slice;
	//~ while portion.len() > 1 {
		//~ heap_pop(portion);
		//~ portion = portion.init_mut();
	//~ }
	
	let ln = slice.len();
	if ln <= 1 {return;}
	for i in range(0, ln - 1){
		let portion = slice.slice_to_mut(ln - i);
		heap_pop(portion);
	}
}

/// The heapsort algorithm.
/// This turns the array into an in-place binary max heap, then uses that to sort the list.
pub fn heapsort<T : Ord>(slice : &mut [T]){
	heapify(slice);
	heap_to_sorted(slice);
}

#[test]
fn test_indexing(){
	assert_eq!(get_parent(1), 0);
	assert_eq!(get_parent(2), 0);
	assert_eq!(get_parent(3), 1);
	assert_eq!(get_parent(4), 1);
	assert_eq!(get_parent(5), 2);
	assert_eq!(get_parent(6), 2);
	assert_eq!(get_parent(7), 3);

	for i in range(0, 21){
		let (l, r) = get_leaves(i);
		assert_eq!(get_parent(l), i);
		assert_eq!(get_parent(r), i);
	}
}

#[cfg(test)]
fn is_max_heap<T : Ord>(slice : &[T]) -> bool{
	for i in range(1, slice.len()){
		let p = get_parent(i);
		if slice[p] < slice[i] {return false;}
	}
	return true;
}

#[test]
fn test_heapify(){
	let mut test_slices = get_test_vecs();
	
	for test_vec in test_slices.iter_mut(){
		let unsorted_vec = test_vec.clone();
		let test_slice = test_vec.as_mut_slice();
		heapify(test_slice);
		println!("Heapifying: {} -> {}", unsorted_vec.as_slice(), test_slice)
		assert!(is_max_heap(test_slice));
	}
}

#[test]
fn test_heapsort(){
	let mut test_slices = get_test_vecs();
	
	for test_vec in test_slices.iter_mut(){
		let test_slice = test_vec.as_mut_slice();
		heapsort(test_slice);
		assert!(is_sorted(test_slice));
	}
}
